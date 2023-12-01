use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DAY_1_DIGITS: &'static [&str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const DAY_2_DIGITS: &'static [&str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn main() {
    println!(
        "[Day 1] The sum of all of the calibration values is: {}",
        sum_calibration_values("./data/input.txt", DAY_1_DIGITS)
    );
    println!(
        "[Day 2] The sum of all of the calibration values is: {}",
        sum_calibration_values("./data/input.txt", DAY_2_DIGITS)
    );
}

fn sum_calibration_values<P>(filename: P, digits: &[&str]) -> u32
where
    P: AsRef<Path>,
{
    let mut sum = 0;
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if let Some(value) = calibration_value(line, digits) {
                    sum += value;
                }
            }
        }
    }
    sum
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calibration_value(line: String, digits: &[&str]) -> Option<u32> {
    // Get first digit from `line`
    let mut first_digit = None;
    let mut first_digit_position = None;
    for p in digits {
        if let Some(pos) = line.find(p) {
            if first_digit_position == None || pos < first_digit_position? {
                first_digit_position = Some(pos);
                first_digit = Some(p);
            }
        }
    }
    // Return if none was found
    if first_digit == None {
        return None;
    }
    // Get the last digit from `line`
    let mut last_digit = first_digit;
    let mut last_digit_position = None;
    for p in digits {
        if let Some(pos) = line.rfind(p) {
            if last_digit_position == None || pos > last_digit_position? {
                last_digit_position = Some(pos);
                last_digit = Some(p);
            }
        }
    }
    // Construct the final value
    Some(
        format!(
            "{}{}",
            translate_digit(first_digit?),
            translate_digit(last_digit?)
        )
        .parse::<u32>()
        .ok()?,
    )
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
        assert_eq!(sum_calibration_values("./data/example1.txt", DAY_1_DIGITS), 142);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(sum_calibration_values("./data/example2.txt", DAY_2_DIGITS), 281);
    }

    #[test]
    fn test_day_1() {
        assert_eq!(sum_calibration_values("./data/input.txt", DAY_1_DIGITS), 55130);
    }

    #[test]
    fn test_day_2() {
        assert_eq!(sum_calibration_values("./data/input.txt", DAY_2_DIGITS), 54985);
    }
}
