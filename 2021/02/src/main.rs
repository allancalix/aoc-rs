use std::fs::File;
use std::io::prelude::*;

const INPUT_FILE: &str = "2021/02/data/input.txt";

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
    Backward,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    value: i32,
}

impl From<&str> for Instruction {
    fn from(v: &str) -> Instruction {
        let mut split = v.split(" ").take(2);
        let instruction = split.next().unwrap();
        let value = split.next().unwrap();

        let d = match instruction {
            "backward" => Direction::Backward,
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!("no matching instruction direction for: {}", instruction),
        };

        Instruction {
            direction: d,
            value: value.parse::<i32>().unwrap(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    File::open(INPUT_FILE)?.read_to_string(&mut input)?;

    let instructions: Vec<Instruction> = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| Instruction::from(line))
        .collect();

    let mut x = 0;
    let mut y = 0;
    for i in instructions {
        match i.direction {
            Direction::Forward => {
                x += i.value;
            },
            Direction::Backward => {
                x -= i.value;
            },
            Direction::Up => {
                y -= i.value;
            },
            Direction::Down => {
                y += i.value;
            },
        }
    }

    println!("x: {}, y: {}", x, y);
    println!("final: {}", x * y);

    Ok(())
}
