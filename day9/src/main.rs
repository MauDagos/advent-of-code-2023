use std::{
    fs::File, io::{BufRead, BufReader}, path::Path
};

fn main() {
    println!("[Example] {:?}", check_oasis_report("./data/example.txt"));
    println!("[Input] {:?}", check_oasis_report("./data/input.txt"));
}

struct OasisHistory {
    values: Vec<i32>,
}

impl OasisHistory {
    fn from_string(input: String) -> Self {
        let values = input
            .split_whitespace()
            .map(|value| value.parse::<i32>().unwrap())
            .collect();
        OasisHistory { values }
    }

    fn predict_next_value(&self, backwards: bool) -> i32 {
        Self::get_next_difference_value(self.values.clone(), backwards)
    }

    fn get_next_difference_value(values: Vec<i32>, backwards: bool) -> i32 {
        let mut differences = vec![];
        for index in 1..values.len() {
            let this = values[index];
            let prev = values[index - 1];
            differences.push(this - prev);
        }
        
        let next_diff_value = if differences.iter().all(|x| *x == 0) {
            0
        } else {
            Self::get_next_difference_value(differences, backwards)
        };

        if backwards {
            values[0] - next_diff_value
        } else {
            values[values.len() - 1] + next_diff_value
        }
    }
}

struct OasisReport {
    entries: Vec<OasisHistory>,
}

impl OasisReport {
    fn from_file<P>(filename: P) -> Self
    where
        P: AsRef<Path>,
    {
        let entries: Vec<OasisHistory> = BufReader::new(File::open(filename).unwrap())
            .lines()
            .flatten()
            .map(OasisHistory::from_string)
            .collect();
        OasisReport { entries }
    }

    fn predict_next_values(&self, backwards: bool) -> Vec<i32> {
        self.entries.iter().map(|entry| entry.predict_next_value(backwards)).collect()
    }
}

fn check_oasis_report<P>(filename: P) -> (i32, i32)
where
    P: AsRef<Path>,
{
    let report = OasisReport::from_file(filename);
    let predicted = report.predict_next_values(false);
    let predicted_backwards = report.predict_next_values(true);
    (predicted.iter().sum(), predicted_backwards.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(check_oasis_report("./data/example.txt"), (114, 2));
    }

    #[test]
    fn test_input() {
        assert_eq!(check_oasis_report("./data/input.txt"), (1974232246, 928));
    }
}
