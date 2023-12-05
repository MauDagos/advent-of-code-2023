use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const DAY_1_DIGITS: &[&str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const DAY_2_DIGITS: &[&str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn main() -> Result<(), Box<dyn error::Error>> {
    println!(
        "[Day 1] The sum of all of the calibration values is: {}",
        sum_calibration_values("./data/input.txt", DAY_1_DIGITS)?
    );
    println!(
        "[Day 2] The sum of all of the calibration values is: {}",
        sum_calibration_values("./data/input.txt", DAY_2_DIGITS)?
    );
    Ok(())
}

fn sum_calibration_values<P>(filename: P, digits: &[&str]) -> Result<u32, Box<dyn error::Error>>
where
    P: AsRef<Path>,
{
    let mut sum = 0;
    for line in BufReader::new(File::open(filename)?).lines().flatten() {
        if let Some(value) = calibration_value(line, digits) {
            sum += value;
        }
    }
    Ok(sum)
}

fn calibration_value(line: String, digits: &[&str]) -> Option<u32> {
    // Get first digit from `line`
    let mut first_digit = None;
    let mut first_digit_position = None;
    for digit in digits {
        if let Some(pos) = line.find(digit) {
            if first_digit_position.is_none() || pos < first_digit_position? {
                first_digit_position = Some(pos);
                first_digit = Some(digit);
            }
        }
    }
    // Return if none was found
    first_digit?;
    // Get the last digit from `line`
    let mut last_digit = first_digit;
    let mut last_digit_position = None;
    for digit in digits {
        if let Some(pos) = line.rfind(digit) {
            if last_digit_position.is_none() || pos > last_digit_position? {
                last_digit_position = Some(pos);
                last_digit = Some(digit);
            }
        }
    }
    // Construct the final value
    format!(
        "{}{}",
        translate_digit(first_digit?),
        translate_digit(last_digit?)
    )
    .parse::<u32>()
    .ok()
}

fn translate_digit(digit: &str) -> &str {
    match digit {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => digit,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        if let Ok(result) = sum_calibration_values("./data/example1.txt", DAY_1_DIGITS) {
            assert_eq!(result, 142)
        }
    }

    #[test]
    fn test_example_2() {
        if let Ok(result) = sum_calibration_values("./data/example2.txt", DAY_2_DIGITS) {
            assert_eq!(result, 281);
        }
    }

    #[test]
    fn test_day_1() {
        if let Ok(result) = sum_calibration_values("./data/input.txt", DAY_1_DIGITS) {
            assert_eq!(result, 55130);
        }
    }

    #[test]
    fn test_day_2() {
        if let Ok(result) = sum_calibration_values("./data/input.txt", DAY_2_DIGITS) {
            assert_eq!(result, 54985);
        }
    }
}
