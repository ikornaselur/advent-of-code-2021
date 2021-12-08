use std::error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

const INPUT: &str = "src/day3/input.txt";

#[derive(Debug, PartialEq)]
struct Day3Error(String);

impl fmt::Display for Day3Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid input: {}", self.0)
    }
}
impl error::Error for Day3Error {}

fn main() -> Result<()> {
    println!("Day 3 - Part 1: {}", part1()?);
    println!("Day 3 - Part 2: {}", part2()?);
    Ok(())
}

fn part1() -> Result<i32> {
    let mut file = File::open(INPUT)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.split('\n');

    let bit_counts = get_bit_counts(lines)?;
    let (gamma, epsilon) = calculate_gamma_epsilon(bit_counts)?;

    Ok(gamma * epsilon)
}

fn part2() -> Result<i32> {
    let mut file = File::open(INPUT)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.split('\n');

    Ok(0)
}

/// Go through a list of bits and calculate the counts of ones and zeroes. Rather then storing the
/// number of ones and number of zeroes, just increment the counter for 1 and decrement for 0. If
/// the end result is positive, the most common bit is 1, if negative, then most common is 0
fn get_bit_counts<'a>(lines: impl Iterator<Item = &'a str>) -> Result<Vec<i32>> {
    // Peek at the first item to get the number of bytes to keep track of
    let mut iter = lines.peekable();

    let length = match iter.peek() {
        Some(&val) => val.len(),
        None => return Err(Box::new(Day3Error("No elements to process".into()))),
    };

    let mut counts = vec![0; length];

    for line in iter {
        for (idx, char) in line.chars().enumerate() {
            match char {
                '1' => counts[idx] += 1,
                '0' => counts[idx] -= 1,
                _ => continue,
            }
        }
    }

    Ok(counts)
}

fn calculate_gamma_epsilon(bit_counts: Vec<i32>) -> Result<(i32, i32)> {
    // Get the most common bits from the bit counts to calculate the gamma
    let mut gamma = 0;
    let mut epsilon = 0;

    let mut val = 1;
    for bit in bit_counts.into_iter().rev() {
        if bit > 0 {
            // Most common was positive, bump gamma
            gamma += val;
        } else {
            // Least common was negative, gump epsilon
            epsilon += val;
        }
        val *= 2;
    }

    // The epsilon is just the gamma bit flipped
    Ok((gamma, epsilon))
}

#[cfg(test)]
mod day3 {
    use super::*;

    #[test]
    fn test_get_bit_counts() {
        let lines = vec!["1010", "0001", "0011", ""].into_iter();

        assert_eq!(get_bit_counts(lines).unwrap(), vec![-1, -3, 1, 1]);
    }

    #[test]
    fn test_calculate_gamma() {
        let (gamma, _) = calculate_gamma_epsilon(vec![1, -1, 1, 1, -1]).unwrap();

        assert_eq!(gamma, 22);
    }

    #[test]
    fn test_calculate_epsilon() {
        let (_, epsilon) = calculate_gamma_epsilon(vec![1, -1, 1, 1, -1]).unwrap();

        assert_eq!(epsilon, 9);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1().unwrap(), 3882564);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2().unwrap(), 0);
    }
}
