use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("src/day1/input.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    let lines = contents.split('\n');
    let part1 = count_increases(lines)?;

    println!("Day 1 - Part 1: {}", part1);

    Ok(())
}

fn count_increases<'a>(mut lines: impl Iterator<Item = &'a str>) -> Result<usize, Box<dyn Error>> {
    let mut increases = 0;

    let mut last: i32 = match lines.next() {
        None => 0,
        Some(val) => val.parse()?,
    };

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let current: i32 = line.parse()?;
        if current > last {
            increases += 1;
        }
        last = current;
    }

    Ok(increases)
}

#[cfg(test)]
mod day1 {
    use super::*;

    #[test]
    fn test_count_increases() {
        let input = vec!["10", "11", "9", "12"];

        assert_eq!(count_increases(input.into_iter()).unwrap(), 2);
    }

    #[test]
    fn test_count_increases_empty() {
        let input = vec![];

        assert_eq!(count_increases(input.into_iter()).unwrap(), 0);
    }

    #[test]
    fn test_count_increases_handles_empty_lines() {
        let input = vec!["10", "", "11", "9", "12", ""];

        assert_eq!(count_increases(input.into_iter()).unwrap(), 2);
    }
}
