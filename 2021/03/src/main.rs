use std::fs::File;
use std::io::prelude::*;

// INPUT_FILE is a sequence of 12-bit ASCII binary encoded numbers. For example,
// "111100001111" for n lines.
const INPUT_FILE: &str = "2021/03/data/input.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    File::open(INPUT_FILE)?.read_to_string(&mut input)?;

    let mut total = 0;
    let mut counts: Vec<u32> = (0..12).map(|_v| 0).collect();
    for line in input.lines().filter(|line| !line.is_empty()) {
        total += 1;
        assert_eq!(line.len(), 12);

        for (i, c) in line.chars().enumerate() {
            match c {
                '1' => {
                    counts[i] += 1;
                },
                '0' => {},
                _ => panic!("unrecognized character: {}", c),
            };
        }
    }

    println!("Total: {:?}", total);
    println!("{:?}", counts);

    let mut eps = 1;
    let mut gamma = 0;

    if counts[0] >= (total / 2) {
        gamma = 1;
        eps = 0;
    }

    for i in counts[1..].iter() {
        if i >= &(total / 2) {
            gamma = gamma << 1;
            gamma += 1;

            eps = eps << 1;
            continue;
        }

        eps = eps << 1;
        eps += 1;

        gamma = gamma << 1;
    }

    println!("gamma: {}, eps: {}", gamma, eps);
    println!("{}", gamma * eps);

    Ok(())
}
