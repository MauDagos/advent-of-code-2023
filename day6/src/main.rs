use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let (ways_to_beat_races, ways_to_beat_race) = check_races("./data/input.txt");
    println!("[Part 1] The product of the ways to beat the races is: {ways_to_beat_races}");
    println!("[Part 2] The number of ways to beat the race is: {ways_to_beat_race}");
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn is_time_held_new_record(&self, time_held: u64) -> bool {
        if time_held > self.time {
            return false;
        }

        let time_available = self.time - time_held;
        let speed = time_held;
        time_available * speed > self.distance
    }

    fn ways_to_beat_record(&self) -> u64 {
        let middle = self.time / 2;

        let left_pos = (0..middle)
            .rev()
            .find(|t| !self.is_time_held_new_record(*t))
            .unwrap_or(middle);

        let right_pos = (middle..self.time)
            .find(|t| !self.is_time_held_new_record(*t))
            .unwrap_or(middle);

        right_pos - left_pos - 1
    }
}

fn check_races<P>(filename: P) -> (u64, u64)
where
    P: AsRef<Path>,
{
    let mut iter = BufReader::new(File::open(filename).unwrap()).lines();
    let times_line = iter.next().unwrap().unwrap();
    let distances_line = iter.next().unwrap().unwrap();

    (
        part1(&times_line, &distances_line),
        part2(&times_line, &distances_line),
    )
}

fn part1(times_line: &str, distances_line: &str) -> u64 {
    let times = nums_for_line(times_line);
    let distances = nums_for_line(distances_line);

    let races = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect::<Vec<Race>>();

    races.iter().map(|r| r.ways_to_beat_record()).product()
}

fn nums_for_line(line: &str) -> Vec<u64> {
    line.split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

fn part2(times_line: &str, distances_line: &str) -> u64 {
    let race = Race {
        time: num_for_line(times_line),
        distance: num_for_line(distances_line),
    };

    race.ways_to_beat_record()
}

fn num_for_line(line: &str) -> u64 {
    line.split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(check_races("./data/example.txt"), (288, 71503));
    }

    #[test]
    fn test_input() {
        assert_eq!(check_races("./data/input.txt"), (219849, 29432455));
    }
}
