use advent_of_code_22::Data;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn apply_operation(
    data: &mut Data,
    n: usize,
    from: usize,
    to: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut moved = data.stacks[from - 1]
        .drain(0..n)
        .collect::<VecDeque<char>>();
    moved.append(&mut data.stacks[to - 1]);
    data.stacks[to - 1] = moved;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("day5_2/input.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = Data::new(&mut reader)?;
    for line in reader.lines() {
        let ln = line?;
        if ln.is_empty() {
            continue;
        }

        let split: Vec<&str> = ln.split_ascii_whitespace().collect();
        if split.len() == 6 {
            let n = split[1].parse::<usize>()?;
            let from = split[3].parse::<usize>()?;
            let to = split[5].parse::<usize>()?;
            apply_operation(&mut data, n, from, to)?;
        } else {
            return Err("Invalid input".into());
        }
    }

    for stack in data.stacks {
        let front = stack.front().copied();
        print!("{}", front.unwrap_or('-'));
    }
    println!();

    Ok(())
}
