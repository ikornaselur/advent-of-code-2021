use std::error;
use std::fs::File;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let mut file = File::open("src/day1/input.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    let lines = contents.split('\n');
    let part1 = count_increases(lines.clone())?;

    println!("Day 1 - Part 1: {}", part1);

    let part2 = count_increases_window(lines, 3)?;

    println!("Day 1 - Part 2: {}", part2);

    Ok(())
}

fn count_increases<'a>(mut lines: impl Iterator<Item = &'a str>) -> Result<usize> {
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

fn count_increases_window<'a>(
    mut lines: impl Iterator<Item = &'a str> + Clone,
    window_size: usize,
) -> Result<usize> {
    let mut increases = 0;

    let mut left = lines.clone();
    let mut sum = lines
        .by_ref()
        .take(window_size)
        .fold(0, |acc, x| acc + x.parse::<i32>().unwrap());

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let earliest = left.next().unwrap().parse::<i32>()?;
        let latest = line.parse::<i32>()?;
        let next_sum = sum - earliest + latest;

        if next_sum > sum {
            increases += 1;
        }
        sum = next_sum;
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

    #[test]
    fn test_count_increases_window() {
        let input = vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ];

        assert_eq!(count_increases_window(input.into_iter(), 3).unwrap(), 5);
    }
}
