use std::cmp::Ordering;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let score = check_bids("./data/example.txt");
    println!("[Part 1] The total winnings are: {}", score);
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum CardLabel {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq)]
struct Cards {
    cards: Vec<CardLabel>,
}

impl Cards {
    fn from_string(input: &str) -> Cards {
        let cards = input
            .chars()
            .filter_map(|c| match c {
                '2' => Some(CardLabel::Two),
                '3' => Some(CardLabel::Three),
                '4' => Some(CardLabel::Four),
                '5' => Some(CardLabel::Five),
                '6' => Some(CardLabel::Six),
                '7' => Some(CardLabel::Seven),
                '8' => Some(CardLabel::Eight),
                '9' => Some(CardLabel::Nine),
                'T' => Some(CardLabel::Ten),
                'J' => Some(CardLabel::Jack),
                'Q' => Some(CardLabel::Queen),
                'K' => Some(CardLabel::King),
                'A' => Some(CardLabel::Ace),
                _ => None,
            })
            .collect();

        Cards { cards }
    }

    fn hand_type(&self) -> HandType {
        let map = self.cards.iter().fold(HashMap::new(), |mut map, card| {
            let count = map.entry(card).or_insert(0);
            *count += 1;
            map
        });

        match map.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if map.values().any(|count| *count == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if map.values().any(|count| *count == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl Ord for Cards {
    fn cmp(&self, other: &Self) -> Ordering {
        for (this_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            if this_card == other_card {
                continue;
            }
            return this_card.cmp(other_card);
        }
        Ordering::Equal
    }
}

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Cards,
    r#type: HandType,
}

impl Hand {
    fn from_string(input: &str) -> Hand {
        let cards = Cards::from_string(input);
        Hand {
            r#type: cards.hand_type(),
            cards,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.r#type == other.r#type {
            self.cards.cmp(&other.cards)
        } else {
            self.r#type.cmp(&other.r#type)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Bid {
    hand: Hand,
    bid: u32,
}

impl Bid {
    fn from_string(input: String) -> Bid {
        let (cards_input, bid_input) = input.split_once(' ').unwrap();
        let hand = Hand::from_string(cards_input);
        let bid = bid_input.parse::<u32>().unwrap();
        Bid { hand, bid }
    }
}

impl Ord for Bid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for Bid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn check_bids<P>(filename: P) -> u32
where
    P: AsRef<Path>,
{
    let mut bids = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|line| Bid::from_string(line.unwrap()))
        .collect::<Vec<Bid>>();

    bids.sort();

    bids.iter()
        .enumerate()
        .map(|(rank, bid)| {
            // println!("{:?}", bid);
            (rank + 1) as u32 * bid.bid
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(check_bids("./data/example.txt"), 6440);
    }

    #[test]
    fn test_input() {
        assert_eq!(check_bids("./data/input.txt"), 248179786);
    }
}
