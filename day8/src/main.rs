use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    run_example();
    run_input();
}

fn run_example() {
    let instructions_steps = check_network_instructions("./data/example1.txt");
    println!("[Example] Instructions took {} steps", instructions_steps);
    let ghost_steps = check_network_ghost("./data/example2.txt");
    println!("[Example] Ghost took {} steps", ghost_steps);
}

fn run_input() {
    let instructions_steps = check_network_instructions("./data/input.txt");
    println!("[Input] Instructions took {} steps", instructions_steps);
    let ghost_steps = check_network_ghost("./data/input.txt");
    println!("[Input] Ghost took {} steps", ghost_steps);
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node {
    position: String,
    left: String,
    right: String,
}

impl Node {
    fn from_string(input: String) -> Node {
        // Input looks like: "position = (left, right)"
        let (mut position, options) = input.split_once('=').unwrap();
        position = position.trim();
        let (left, mut right) = options
            .trim()
            .trim_matches(|c| c == '(' || c == ')')
            .split_once(',')
            .unwrap();
        right = right.trim();
        Node {
            position: position.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        }
    }

    fn next_position(&self, instruction: &Instruction) -> String {
        match instruction {
            Instruction::Left => self.left.clone(),
            Instruction::Right => self.right.clone(),
        }
    }
}

struct Network {
    instructions: Vec<Instruction>,
    map: HashMap<String, Node>,
}

impl Network {
    fn from_file<P>(filename: P) -> Self
    where
        P: AsRef<Path>,
    {
        let mut lines_iter = BufReader::new(File::open(filename).unwrap()).lines();
        let instructions = Self::_parse_instructions(lines_iter.next().unwrap().unwrap());
        lines_iter.next().unwrap().unwrap(); // Skip blank line
        let mut map = HashMap::new();
        for line in lines_iter {
            let node = Node::from_string(line.unwrap());
            map.insert(node.position.clone(), node);
        }
        Network { instructions, map }
    }

    fn _parse_instructions(input: String) -> Vec<Instruction> {
        input
            .chars()
            .map(|c| match c {
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                _ => panic!("Invalid instruction"),
            })
            .collect::<Vec<Instruction>>()
    }

    fn next_node(&self, current_node: &Node, instruction: &Instruction) -> &Node {
        let next_position = current_node.next_position(instruction);
        self.map.get(&next_position).unwrap()
    }

    fn run_instructions(&self) -> u64 {
        let start_position = "AAA".to_string();
        let end_position = "ZZZ".to_string();

        let mut steps = 0;
        let mut current_node = self.map.get(&start_position).unwrap();
        loop {
            for instruction in &self.instructions {
                steps += 1;
                current_node = self.next_node(current_node, instruction);
                if current_node.position == end_position {
                    return steps;
                }
            }
        }
    }

    fn run_ghost(&self) -> u64 {
        let starting_positions: Vec<String> = self
            .map
            .keys()
            .filter(|k| k.ends_with('A'))
            .cloned()
            .collect();

        let mut minumum_steps: Vec<u64> = vec![];
        for starting_position in starting_positions {
            let mut steps = 0;
            let mut current_node = self.map.get(&starting_position).unwrap();
            loop {
                let mut found = false;
                for instruction in &self.instructions {
                    steps += 1;
                    current_node = self.next_node(current_node, instruction);
                    if current_node.position.ends_with('Z') {
                        minumum_steps.push(steps);
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
        }
        Self::lcm(&minumum_steps)
    }

    // Copied from https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
    fn lcm(nums: &[u64]) -> u64 {
        if nums.len() == 1 {
            return nums[0];
        }
        let a = nums[0];
        let b = Self::lcm(&nums[1..]);
        a * b / Self::gcd_of_two_numbers(a, b)
    }

    fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
        if b == 0 {
            return a;
        }
        Self::gcd_of_two_numbers(b, a % b)
    }
}

fn check_network_instructions<P>(filename: P) -> u64
where
    P: AsRef<Path>,
{
    Network::from_file(filename).run_instructions()
}

fn check_network_ghost<P>(filename: P) -> u64
where
    P: AsRef<Path>,
{
    Network::from_file(filename).run_ghost()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(check_network_instructions("./data/example1.txt"), 6);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(check_network_ghost("./data/example2.txt"), 6);
    }

    #[test]
    fn test_input_part1() {
        assert_eq!(check_network_instructions("./data/input.txt"), 16531);
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(check_network_ghost("./data/input.txt"), 24035773251517);
    }
}
