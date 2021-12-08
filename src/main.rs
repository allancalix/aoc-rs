use std::io::prelude::*;
use std::fs::File;

const INPUT_FILE: &str = "2021/01/data/input.txt";

fn count_increases(input: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let pruned_input: Vec<u32> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    let mut prev = pruned_input[0];
    let mut times_increased = 0;
    for input in &pruned_input {
        if input > &prev {
            times_increased += 1;
        }

        prev = *input;
    }

    Ok(times_increased)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    File::open(INPUT_FILE)?.read_to_string(&mut input)?;

    let times_increased = count_increases(&input)?;
    println!("{}", times_increased);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_is_valid() {
        const INPUT: &str = r#"
199
200
208
210
200
207
240
269
260
263
"#;

        let actual = count_increases(INPUT).unwrap();
        assert_eq!(actual, 7);
    }
}
