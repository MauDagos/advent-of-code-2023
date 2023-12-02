use std::collections::HashMap;
use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::Path;

fn main() -> Result<(), Box<dyn error::Error>> {
    let (sum_of_possible_game_ids, sum_of_powers) = check_games("./data/input.txt")?;
    println!(
        "The sum of the possible game IDs is: {}",
        sum_of_possible_game_ids
    );
    println!("The sum of the powers is: {}", sum_of_powers);
    Ok(())
}

fn check_games<P>(filename: P) -> Result<(u32, u32), Box<dyn error::Error>>
where
    P: AsRef<Path>,
{
    let elf_bag = HashMap::from([
        (String::from("red"), 12),
        (String::from("green"), 13),
        (String::from("blue"), 14),
    ]);
    let mut sum_of_possible_game_ids = 0;
    let mut game_id = 0;
    let mut sum_of_powers = 0;
    for line in BufReader::new(File::open(filename)?).lines() {
        if let Ok(line) = line {
            game_id += 1;
            let (is_possible, power) = check_game(line, &elf_bag)?;
            if is_possible {
                sum_of_possible_game_ids += game_id;
            }
            sum_of_powers += power;
        }
    }
    Ok((sum_of_possible_game_ids, sum_of_powers))
}

fn check_game(line: String, elf_bag: &HashMap<String, u32>) -> Result<(bool, u32), ParseIntError> {
    let mut is_possible = true;
    let mut minimum_game_bag = HashMap::new();
    if let Some(colon_pos) = line.find(':') {
        let offset = ": ".len();
        let sets_unparsed = &line[(colon_pos + offset)..];
        for set in sets_unparsed.split(';') {
            for cube in set.split(',') {
                if let Some((amount_unparsed, color)) = cube.trim_start().split_once(' ') {
                    let amount = amount_unparsed.parse::<u32>()?;
                    // Check if the game is possible. It is if it's been
                    // possible until now and if there are enough cubes in the
                    // elf's bag
                    is_possible = is_possible && is_bag_big_enough(elf_bag, color, amount);
                    // Keep track of the minimum bag for this game
                    if !is_bag_big_enough(&minimum_game_bag, color, amount) {
                        minimum_game_bag.insert(color.to_string(), amount);
                    }
                }
            }
        }
    }
    let power = minimum_game_bag.values().product();
    Ok((is_possible, power))
}

fn is_bag_big_enough(bag: &HashMap<String, u32>, color: &str, amount: u32) -> bool {
    bag.contains_key(color) && bag[color] >= amount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        if let Ok((sum_of_possible_game_ids, sum_of_powers)) = check_games("./data/example.txt") {
            assert_eq!(sum_of_possible_game_ids, 8);
            assert_eq!(sum_of_powers, 2286);
        }
    }

    #[test]
    fn test_day_1() {
        if let Ok((sum_of_possible_game_ids, sum_of_powers)) = check_games("./data/input.txt") {
            assert_eq!(sum_of_possible_game_ids, 2348);
            assert_eq!(sum_of_powers, 76008);
        }
    }
}
