use std::error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq)]
struct Day2Error(String);

impl fmt::Display for Day2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid input: {}", self.0)
    }
}
impl error::Error for Day2Error {}

const INPUT: &str = "src/day2/input.txt";

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug, PartialEq, Eq)]
struct Movement {
    distance: i32,
    direction: Direction,
}

#[derive(Debug, PartialEq, Eq)]
struct Position {
    horizontal: i32,
    depth: i32,
}

fn main() -> Result<()> {
    part1()?;
    part2()?;
    Ok(())
}

fn part1() -> Result<()> {
    let mut file = File::open(INPUT)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.split('\n');

    let position = parse_movements(lines)?;

    let part1 = position.horizontal * position.depth;

    println!("Day 2 - Part 1: {}", part1);

    Ok(())
}

fn part2() -> Result<()> {
    let mut file = File::open(INPUT)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.split('\n');

    let position = parse_movements_with_aim(lines)?;

    let part2 = position.horizontal * position.depth;

    println!("Day 2 - Part 2: {}", part2);

    Ok(())
}

fn parse_movements<'a>(lines: impl Iterator<Item = &'a str>) -> Result<Position> {
    let mut position = Position {
        horizontal: 0,
        depth: 0,
    };

    for line in lines {
        match parse_movement(line) {
            Ok(Movement {
                direction: Direction::Forward,
                distance,
            }) => position.horizontal += distance,
            Ok(Movement {
                direction: Direction::Down,
                distance,
            }) => position.depth += distance,
            Ok(Movement {
                direction: Direction::Up,
                distance,
            }) => position.depth -= distance,
            Err(e) => return Err(e),
        }
    }

    Ok(position)
}

fn parse_movements_with_aim<'a>(lines: impl Iterator<Item = &'a str>) -> Result<Position> {
    let mut position = Position {
        horizontal: 0,
        depth: 0,
    };
    let mut aim = 0;

    for line in lines {
        match parse_movement(line) {
            Ok(Movement {
                direction: Direction::Forward,
                distance,
            }) => {
                position.horizontal += distance;
                position.depth += distance * aim;
            }
            Ok(Movement {
                direction: Direction::Down,
                distance,
            }) => aim += distance,
            Ok(Movement {
                direction: Direction::Up,
                distance,
            }) => aim -= distance,
            Err(e) => return Err(e),
        }
    }

    Ok(position)
}

fn parse_movement(line: &str) -> Result<Movement> {
    let mut split = line.split(" ");
    let direction = match split.next() {
        None => return Err(Box::new(Day2Error("No direction element in line".into()))),
        Some(val) => match val {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            "" => {
                return Ok(Movement {
                    direction: Direction::Forward,
                    distance: 0,
                })
            }
            other => {
                return Err(Box::new(Day2Error(
                    format!("Unknown direction: {}", other).into(),
                )))
            }
        },
    };

    let distance = match split.next() {
        None => return Err(Box::new(Day2Error("No distance element in line".into()))),
        Some(val) => val.parse::<i32>()?,
    };

    Ok(Movement {
        direction,
        distance,
    })
}

#[cfg(test)]
mod day2 {
    use super::*;

    #[test]
    fn test_parse_movement() {
        assert_eq!(
            parse_movement("forward 5").unwrap(),
            Movement {
                direction: Direction::Forward,
                distance: 5,
            }
        );

        assert_eq!(
            parse_movement("up 3").unwrap(),
            Movement {
                direction: Direction::Up,
                distance: 3,
            }
        );

        assert_eq!(
            parse_movement("down 500").unwrap(),
            Movement {
                direction: Direction::Down,
                distance: 500,
            }
        );

        assert_eq!(
            parse_movement("").unwrap(),
            Movement {
                direction: Direction::Forward,
                distance: 0,
            }
        );
    }

    #[test]
    fn test_parse_movement_invalid() {
        assert!(parse_movement("forward forward").is_err());
        assert!(parse_movement("forward").is_err());
        assert!(parse_movement("sideways 4").is_err());
    }

    #[test]
    fn test_parse_movements() {
        let lines = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];

        let position = parse_movements(lines.into_iter()).unwrap();

        assert_eq!(
            position,
            Position {
                horizontal: 15,
                depth: 10,
            }
        );
    }

    #[test]
    fn test_parse_movements_with_aim() {
        let lines = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];

        let position = parse_movements_with_aim(lines.into_iter()).unwrap();

        assert_eq!(
            position,
            Position {
                horizontal: 15,
                depth: 60,
            }
        );
    }
}
