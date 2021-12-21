extern crate regex;

use std::fs::File;
use std::collections::HashMap;
use std::io::prelude::*;

use regex::Regex;

const INPUT_FILE: &str = "2021/05/data/input.txt";

type Point = (u32, u32);

#[derive(Debug)]
struct Line {
    first: Point,
    second: Point,
}

impl Line {
    fn new(first: Point, second: Point) -> Self {
        Self {
            first,
            second,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    File::open(INPUT_FILE)?.read_to_string(&mut input)?;

    let re = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();

    let mut lines: Vec<Line> = vec!();
    for line in input.lines() {
        if let Some(line) = re.captures(line) {
            let first = (line["x1"].parse::<u32>().unwrap(), line["y1"].parse::<u32>().unwrap());
            let second = (line["x2"].parse::<u32>().unwrap(), line["y2"].parse::<u32>().unwrap());

            lines.push(Line::new(first, second));
        };
    }

    let mut counts: HashMap<(u32, u32), u32> = HashMap::new();
    for line in &lines {
        if line.first.0 == line.second.0 {
            let (min, max) = if line.first.1 < line.second.1 {
                (line.first.1, line.second.1)
            } else {
                (line.second.1, line.first.1)
            };

            for y in min..=max {
                let entry = counts.entry((line.first.0, y)).or_insert(0);

                *entry += 1;
            }
        }

        if line.first.1 == line.second.1 {
            let (min, max) = if line.first.0 < line.second.0 {
                (line.first.0, line.second.0)
            } else {
                (line.second.0, line.first.0)
            };

            for x in min..=max {
                let entry = counts.entry((x, line.first.1)).or_insert(0);

                *entry += 1;
            }
        }
    }

    let mut count = 0;
    for (_, k) in &counts {
        if *k >= 2 {
            count += 1;
        }
    }

    println!("{:?}", count);

    Ok(())
}
