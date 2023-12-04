use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let (sum, total) = check_cards("./data/example.txt");
    println!("The sum of the points of all cards is: {}", sum);
    println!("The total amount of cards is: {}", total)
}

struct Card {
    winning: Vec<u32>,
    my_numbers: Vec<u32>,
}

impl Card {
    fn from_line(line: &str) -> Self {
        let (_, nums) = line.split_once(':').unwrap();
        let (wins, mine) = nums.split_once('|').unwrap();
        let winning = Self::parse_numbers(wins);
        let my_numbers = Self::parse_numbers(mine);
        Card {
            winning,
            my_numbers,
        }
    }

    fn parse_numbers(nums: &str) -> Vec<u32> {
        nums.split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect()
    }

    fn wins(&self) -> usize {
        self.my_numbers
            .iter()
            .filter(|x| self.winning.contains(x))
            .count()
    }

    fn points(&self) -> u32 {
        let wins = self.wins();
        if wins > 0 {
            2_u32.pow(wins as u32 - 1)
        } else {
            0
        }
    }
}

struct CardPileEntry {
    card: Card,
    copies: u32,
}

impl CardPileEntry {
    fn incr_copies(&mut self, amount: u32) {
        self.copies += amount;
    }
}

struct CardPile {
    pile: Vec<CardPileEntry>,
}

impl CardPile {
    fn from_cards(cards: Vec<Card>) -> Self {
        let pile = cards
            .into_iter()
            .map(|c| CardPileEntry { card: c, copies: 1 })
            .collect();
        CardPile { pile }
    }

    fn total_cards(&mut self) -> u32 {
        let pile = &mut self.pile;
        let size = pile.len();
        // Update card copies
        for i in 0..size {
            let entry = &pile[i];
            let copies = entry.copies;
            let wins = entry.card.wins();
            if wins > 0 {
                for j in (i + 1)..(i + 1 + wins as usize).min(size) {
                    let entry_won = &mut pile[j];
                    entry_won.incr_copies(copies)
                }
            }
        }
        // Return the sum of copies
        pile.iter().map(|e| e.copies).sum()
    }
}

fn check_cards<P>(filename: P) -> (u32, u32)
where
    P: AsRef<Path>,
{
    let cards: Vec<Card> = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| Card::from_line(&l.unwrap()))
        .collect();
    // Part 1
    let all_points = cards.iter().map(|c| c.points()).sum();
    // Part 2
    let mut pile = CardPile::from_cards(cards);
    let total_cards = pile.total_cards();
    (all_points, total_cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(check_cards("./data/example.txt"), (13, 30));
    }

    #[test]
    fn test_input() {
        assert_eq!(check_cards("./data/input.txt"), (23750, 13261850));
    }
}
