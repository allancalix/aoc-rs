use std::fs::File;
use std::io::prelude::*;

const INPUT_FILE: &str = "2021/04/data/input.txt";

#[derive(Debug, Clone, PartialEq)]
enum State {
    Unset,
    Set,
}

#[derive(Debug, Clone)]
struct Cell {
    state: State,
    value: String,
}

impl Cell {
    fn new(value: &str) -> Cell {
        Cell {
            state: State::Unset,
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    board: Vec<Vec<Cell>>,
}

impl Board {
    pub fn update(&mut self, n: &str) {
        for i in &mut self.board {
            for j in i {
                if &j.value == n {
                    j.state = State::Set;
                }
            }
        }
    }

    pub fn is_complete(&self) -> bool {
        let mut columns = vec!(
            vec!(Cell::new(""), Cell::new(""), Cell::new(""), Cell::new(""), Cell::new("")),
            vec!(Cell::new(""), Cell::new(""), Cell::new(""), Cell::new(""), Cell::new("")),
            vec!(Cell::new(""), Cell::new(""), Cell::new(""), Cell::new(""), Cell::new("")),
            vec!(Cell::new(""), Cell::new(""), Cell::new(""), Cell::new(""), Cell::new("")),
            vec!(Cell::new(""), Cell::new(""), Cell::new(""), Cell::new(""), Cell::new("")),
        );

        for (i, row) in self.board.iter().enumerate() {
            let mut done = 0;

            for (j, cell) in row.iter().enumerate() {
                columns[j][i] = cell.clone();

                if cell.state == State::Set {
                    done += 1;
                }
            }

            if done == 5 {
                println!("{:?}", row);
                return true;
            }
        }

        for i in columns {
            let mut done = 0;

            for j in &i {
                if j.state == State::Set {
                    done += 1;
                }
            }

            if done == 5 {
                println!("{:?}", i);
                return true;
            }
        }

        false
    }

    pub fn sum_unmarked(&self) -> i32 {
        let mut sum = 0;

        for i in &self.board {
            for j in i {
                if j.state == State::Unset {
                    sum += j.value.parse::<i32>().unwrap();
                }
            }
        }

        sum
    }
}

fn parse_board(raw_board: &Vec<&str>) -> Board {
    let b: Vec<Vec<Cell>> = raw_board.iter().map(|line| {
        line
            .split(" ").map(|v| Cell::new(v.into()))
            .filter(|v| !v.value.is_empty())
            .collect()
    }).collect();

    Board {
        board: b
    }
}

fn do_round(round_num: &str, boards: &mut [Board]) {
    for board in boards {
        board.update(round_num);
    }
}

fn check_boards(boards: &[Board]) -> Option<Board> {
    for b in boards {
        if b.is_complete() {
            return Some(b.clone());
        }
    }

    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    File::open(INPUT_FILE)?.read_to_string(&mut input)?;

    let mut lines = input.lines();
    let rounds: Vec<String> = lines.next().unwrap()
        .split(",")
        .map(|v| v.into())
        .collect();

    let mut chunk = vec!();
    let mut boards: Vec<Board> = vec!();
    for next in lines {
        if next.is_empty() && chunk.len() == 5 {
            boards.push(parse_board(&chunk));
            chunk.clear();
        }

        if next.is_empty() {
            continue;
        }

        chunk.push(next);
    }

    let mut winning_board = None;
    for round in &rounds {
        if winning_board.is_some() {
            break;
        }

        do_round(round.as_str(), &mut boards);

        if let Some(board) = check_boards(&boards) {
            println!("{}", round.parse::<i32>().unwrap() * board.sum_unmarked());
            winning_board = Some(board);
        }
    }

    println!("{:?}", winning_board);
    println!("{:?}", rounds);

    Ok(())
}
