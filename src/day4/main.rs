use std::error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

const INPUT: &str = "src/day4/input.txt";

#[derive(Debug, PartialEq)]
struct Day4Error(String);

impl fmt::Display for Day4Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid input: {}", self.0)
    }
}
impl error::Error for Day4Error {}

fn main() -> Result<()> {
    println!("Day 4 - Part 1: {}", part1()?);
    println!("Day 4 - Part 2: {}", part2()?);
    Ok(())
}

fn part1() -> Result<i32> {
    let mut file = File::open(INPUT)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut lines = contents.split('\n');

    // First line is moves
    let mut draws = lines
        .next()
        .unwrap()
        .split(',')
        .map(|draw| draw.parse::<i32>().unwrap());

    // Skip the empty line before cards start
    lines.next().unwrap();

    let mut cards: Vec<Card> = vec![];

    let mut lines_peekable = lines.peekable();
    while lines_peekable.peek().is_some() {
        // Get 5 lines for the card
        cards.push(Card::from(
            lines_peekable
                .by_ref()
                .take(5)
                .collect::<Vec<&str>>()
                .join("\n")
                .as_str(),
        ));

        // Skip the empty line that follows
        lines_peekable.next().unwrap();
    }

    loop {
        // Play the draws until first win
        let draw = draws.next().unwrap();
        for card in cards.iter_mut() {
            card.check(draw);
        }
        if let Some(card) = cards.iter().find(|card| card.won()) {
            return Ok(card.unchecked_sum() * draw);
        }
    }
}

fn part2() -> Result<i32> {
    let mut file = File::open(INPUT)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut lines = contents.split('\n');

    // First line is moves
    let mut draws = lines
        .next()
        .unwrap()
        .split(',')
        .map(|draw| draw.parse::<i32>().unwrap());

    // Skip the empty line before cards start
    lines.next().unwrap();

    let mut cards: Vec<Card> = vec![];

    let mut lines_peekable = lines.peekable();
    while lines_peekable.peek().is_some() {
        // Get 5 lines for the card
        cards.push(Card::from(
            lines_peekable
                .by_ref()
                .take(5)
                .collect::<Vec<&str>>()
                .join("\n")
                .as_str(),
        ));

        // Skip the empty line that follows
        lines_peekable.next().unwrap();
    }

    let card_count = cards.len();
    let mut last_board_idx: Option<usize> = None;

    loop {
        // Play the draws until last win
        let draw = draws.next().unwrap();
        for card in cards.iter_mut() {
            card.check(draw);
        }
        let total_wins = cards.iter().filter(|card| card.won()).count();
        if total_wins == card_count - 1 && last_board_idx.is_none() {
            // We have all except one winning cards, figure out which one is the last one
            last_board_idx = Some(
                cards
                    .iter()
                    .enumerate()
                    .find(|(_idx, card)| !card.won())
                    .unwrap()
                    .0,
            )
        }
        if let Some(idx) = last_board_idx {
            if cards[idx].won() {
                return Ok(cards[idx].unchecked_sum() * draw);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Cell {
    val: i32,
    checked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    cells: [[Cell; 5]; 5],
}

impl Card {
    /// Check if the card is a winning card
    fn won(self) -> bool {
        // Check any rows
        if self
            .cells
            .iter()
            .any(|row| row.iter().all(|cell| cell.checked))
        {
            return true;
        }
        // Check any columns
        if (0..5).any(|row_idx| (0..5).all(|col_idx| self.cells[col_idx][row_idx].checked)) {
            return true;
        }
        // Check diagonals
        if (0..5).all(|idx| self.cells[idx][idx].checked)
            || (0..5).all(|idx| self.cells[4 - idx][idx].checked)
        {
            return true;
        }
        false
    }

    fn unchecked_sum(self) -> i32 {
        self.cells
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|cell| !cell.checked)
                    .map(|cell| cell.val)
                    .sum::<i32>()
            })
            .sum()
    }

    fn check(&mut self, val: i32) {
        for row in self.cells.iter_mut() {
            for mut cell in row.iter_mut() {
                if cell.val == val {
                    cell.checked = true;
                    return;
                }
            }
        }
    }
}

impl From<&str> for Card {
    #[rustfmt::skip]
    fn from(raw_input: &str) -> Self {
        let rows = raw_input.split('\n').map(|row| row.split_whitespace().collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

        Card {
            cells: [
                [
                    Cell { val: rows[0][0].parse().unwrap(), checked: false},
                    Cell { val: rows[0][1].parse().unwrap(), checked: false},
                    Cell { val: rows[0][2].parse().unwrap(), checked: false},
                    Cell { val: rows[0][3].parse().unwrap(), checked: false},
                    Cell { val: rows[0][4].parse().unwrap(), checked: false},
                ],
                [
                    Cell { val: rows[1][0].parse().unwrap(), checked: false},
                    Cell { val: rows[1][1].parse().unwrap(), checked: false},
                    Cell { val: rows[1][2].parse().unwrap(), checked: false},
                    Cell { val: rows[1][3].parse().unwrap(), checked: false},
                    Cell { val: rows[1][4].parse().unwrap(), checked: false},
                ],
                [
                    Cell { val: rows[2][0].parse().unwrap(), checked: false},
                    Cell { val: rows[2][1].parse().unwrap(), checked: false},
                    Cell { val: rows[2][2].parse().unwrap(), checked: false},
                    Cell { val: rows[2][3].parse().unwrap(), checked: false},
                    Cell { val: rows[2][4].parse().unwrap(), checked: false},
                ],
                [
                    Cell { val: rows[3][0].parse().unwrap(), checked: false},
                    Cell { val: rows[3][1].parse().unwrap(), checked: false},
                    Cell { val: rows[3][2].parse().unwrap(), checked: false},
                    Cell { val: rows[3][3].parse().unwrap(), checked: false},
                    Cell { val: rows[3][4].parse().unwrap(), checked: false},
                ],
                [
                    Cell { val: rows[4][0].parse().unwrap(), checked: false},
                    Cell { val: rows[4][1].parse().unwrap(), checked: false},
                    Cell { val: rows[4][2].parse().unwrap(), checked: false},
                    Cell { val: rows[4][3].parse().unwrap(), checked: false},
                    Cell { val: rows[4][4].parse().unwrap(), checked: false},
                ],
            ],
        }
    }
}

#[cfg(test)]
mod day4 {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1().unwrap(), 35711);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2().unwrap(), 5586);
    }

    #[test]
    fn test_card_from_str() {
        let input = "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19";

        let card = Card::from(input);

        #[rustfmt::skip]
        assert_eq!(
            card,
            Card {
                cells: [
                    [
                        Cell { val: 22, checked: false },
                        Cell { val: 13, checked: false },
                        Cell { val: 17, checked: false },
                        Cell { val: 11, checked: false },
                        Cell { val: 0, checked: false },
                    ],
                    [
                        Cell { val: 8, checked: false },
                        Cell { val: 2, checked: false },
                        Cell { val: 23, checked: false },
                        Cell { val: 4, checked: false },
                        Cell { val: 24, checked: false },
                    ],
                    [
                        Cell { val: 21, checked: false },
                        Cell { val: 9, checked: false },
                        Cell { val: 14, checked: false },
                        Cell { val: 16, checked: false },
                        Cell { val: 7, checked: false },
                    ],
                    [
                        Cell { val: 6, checked: false },
                        Cell { val: 10, checked: false },
                        Cell { val: 3, checked: false },
                        Cell { val: 18, checked: false },
                        Cell { val: 5, checked: false },
                    ],
                    [
                        Cell { val: 1, checked: false },
                        Cell { val: 12, checked: false },
                        Cell { val: 20, checked: false },
                        Cell { val: 15, checked: false },
                        Cell { val: 19, checked: false },
                    ],
                ]
            }
        );
    }

    #[test]
    fn test_card_won_row() {
        let input = "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19";
        let mut card = Card::from(input);

        assert!(!card.won());

        for idx in 0..5 {
            card.cells[2][idx].checked = true;
        }

        assert!(card.won());
    }

    #[test]
    fn test_card_won_column() {
        let input = "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19";
        let mut card = Card::from(input);

        assert!(!card.won());

        for idx in 0..5 {
            card.cells[idx][2].checked = true;
        }

        assert!(card.won());
    }

    #[test]
    fn test_card_won_diagonal_1() {
        let input = "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19";
        let mut card = Card::from(input);

        assert!(!card.won());

        for idx in 0..5 {
            card.cells[idx][idx].checked = true;
        }

        assert!(card.won());
    }

    #[test]
    fn test_card_won_diagonal_2() {
        let input = "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19";
        let mut card = Card::from(input);

        assert!(!card.won());

        for idx in 0..5 {
            card.cells[4 - idx][idx].checked = true;
        }

        assert!(card.won());
    }

    #[test]
    fn test_card_unchecked_sum() {
        let input = "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19";
        let mut card = Card::from(input);

        assert_eq!(card.unchecked_sum(), 300);

        card.cells[0][0].checked = true; // 22
        card.cells[2][0].checked = true; // 21
        card.cells[4][4].checked = true; // 19

        assert_eq!(card.unchecked_sum(), 300 - 22 - 21 - 19);
    }

    #[test]
    fn test_card_check_val() {
        let input = "22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19";
        let mut card = Card::from(input);

        card.check(17);
        assert!(card.cells[0][2].checked);
    }
}
