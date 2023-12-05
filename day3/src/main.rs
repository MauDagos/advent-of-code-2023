use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
    path::Path,
};

fn main() {
    let (parts, gears) = check_engine_schematic("./data/example.txt");
    println!("The sum of the part numbers is: {}", parts);
    println!("The sum of the gear ratios is: {}", gears);
}

struct Symbol {
    c: char,
    row: usize,
    col: usize,
}

struct PartNumber {
    number: u32,
    row: usize,
    range: Range<usize>,
}

impl PartNumber {
    fn from_row(row: &Vec<char>, row_pos: usize, start: usize) -> Self {
        // Go until the end or until the first-found non-digit looking forwards
        let mut end = row.len();
        if let Some(right_dot_pos) = &row[start..].iter().position(|c| !c.is_ascii_digit()) {
            end = start + right_dot_pos;
        }
        // Parse the part number
        let digits = &row[start..end];
        let part_number = digits.iter().collect::<String>().parse::<u32>().unwrap();
        PartNumber {
            number: part_number,
            row: row_pos,
            range: start..end,
        }
    }

    fn is_adjacent_to_symbol(&self, symbol: &Symbol) -> bool {
        let row_range = self.row.saturating_sub(1)..(self.row + 2);
        let col_range = self.range.start.saturating_sub(1)..(self.range.end + 1);
        row_range.contains(&symbol.row) && col_range.contains(&symbol.col)
    }
}

struct Engine {
    part_numbers: Vec<PartNumber>,
    symbols: Vec<Symbol>,
}

impl Engine {
    fn from_file<P>(filename: P) -> Self
    where
        P: AsRef<Path>,
    {
        let mut part_numbers = vec![];
        let mut symbols = vec![];
        for (i, line) in BufReader::new(File::open(filename).unwrap())
            .lines()
            .enumerate()
        {
            let row: Vec<char> = line.unwrap().chars().collect();
            let mut j = 0;
            while j < row.len() {
                let c = row[j];
                if c.is_ascii_digit() {
                    let part_number = PartNumber::from_row(&row, i, j);
                    j = part_number.range.end;
                    part_numbers.push(part_number);
                    continue;
                }
                if c != '.' {
                    symbols.push(Symbol { c, row: i, col: j })
                }
                j += 1;
            }
        }
        Self {
            part_numbers,
            symbols,
        }
    }

    fn valid_part_numbers(&self) -> Vec<u32> {
        self.part_numbers
            .iter()
            .filter(|p| self.symbols.iter().any(|s| p.is_adjacent_to_symbol(s)))
            .map(|p| p.number)
            .collect()
    }

    fn gear_ratios(&self) -> Vec<u32> {
        let mut ratios = vec![];
        for symbol in &self.symbols {
            if symbol.c == '*' {
                let adjacent: Vec<&PartNumber> = self
                    .part_numbers
                    .iter()
                    .filter(|p| p.is_adjacent_to_symbol(symbol))
                    .collect();
                if adjacent.len() == 2 {
                    ratios.push(adjacent[0].number * adjacent[1].number);
                }
            }
        }
        ratios
    }
}

fn check_engine_schematic<P>(filename: P) -> (u32, u32)
where
    P: AsRef<Path>,
{
    let engine = Engine::from_file(filename);
    let part_numbers = engine.valid_part_numbers();
    let gear_ratios = engine.gear_ratios();
    (part_numbers.iter().sum(), gear_ratios.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let (parts, gears) = check_engine_schematic("./data/example.txt");
        assert_eq!(parts, 4361);
        assert_eq!(gears, 467835);
    }

    #[test]
    fn test_day_1() {
        let (parts, gears) = check_engine_schematic("./data/input.txt");
        assert_eq!(parts, 521601);
        assert_eq!(gears, 80694070);
    }
}
