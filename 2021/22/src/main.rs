extern crate regex;

use std::fs::File;
use std::collections::HashSet;
use std::io::prelude::*;

use regex::Regex;

const INPUT_FILE: &str = "2021/22/data/input.txt";

type CoordinatePair = (i32, i32);

#[derive(Debug)]
struct CoordinatePlane {
    on: HashSet<(i32, i32, i32)>,
}

impl CoordinatePlane {
    fn new() -> Self {
        Self {
            on: HashSet::new(),
        }
    }

    fn turnon_ranges(&mut self, x_range: CoordinatePair, y_range: CoordinatePair, z_range: CoordinatePair) {
        for x in x_range.0..=x_range.1 {
            for y in y_range.0..=y_range.1 {
                for z in z_range.0..=z_range.1 {
                    self.on.insert((x, y, z));
                }
            }
        }
    }

    fn turnoff_ranges(&mut self, x_range: CoordinatePair, y_range: CoordinatePair, z_range: CoordinatePair) {
        for x in x_range.0..=x_range.1 {
            for y in y_range.0..=y_range.1 {
                for z in z_range.0..=z_range.1 {
                    self.on.remove(&(x, y, z));
                }
            }
        }
    }

    fn reboot_step(&mut self, cmd: &str, x_range: CoordinatePair, y_range: CoordinatePair,
                   z_range: CoordinatePair) {
        if x_range.0 == x_range.1 || y_range.0 == y_range.1 || z_range.0 == z_range.1 {
            return;
        }

        match cmd {
            "on" => {
                self.turnon_ranges(x_range, y_range, z_range);
            },
            "off" => {
                self.turnoff_ranges(x_range, y_range, z_range);
            },
            _ => unimplemented!(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    File::open(INPUT_FILE)?.read_to_string(&mut input)?;

    let re = Regex::new(r"(?P<cmd>(on|off)) x=(?P<x_start>-?\d+)..(?P<x_end>-?\d+),y=(?P<y_start>-?\d+)..(?P<y_end>-?\d+),z=(?P<z_start>-?\d+)..(?P<z_end>-?\d+)").unwrap();

    let mut plane = CoordinatePlane::new();
    for line in input.lines().filter(|line| !line.is_empty()) {
        if let Some(line) = re.captures(line) {
            let cmd = line["cmd"].to_string();
            let x = (line["x_start"].parse::<i32>().unwrap().min(50).max(-50), line["x_end"].parse::<i32>().unwrap().min(50).max(-50));
            let y = (line["y_start"].parse::<i32>().unwrap().min(50).max(-50), line["y_end"].parse::<i32>().unwrap().min(50).max(-50));
            let z = (line["z_start"].parse::<i32>().unwrap().min(50).max(-50), line["z_end"].parse::<i32>().unwrap().min(50).max(-50));

            println!("{} {:?} {:?} {:?}", cmd, x, y, z);
            plane.reboot_step(&cmd, x, y, z);
        };
    }

    println!("{:?}", plane.on.len());

    Ok(())
}
