use advent_of_code_22::utils::get_char_index;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("day3/input.txt")?;
    let sum: isize = file
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (first, last) = line.split_at(line.len() / 2);
            for c in first.chars() {
                if last.contains(c) {
                    return get_char_index(c);
                }
            }
            return 0;
        })
        .sum();

    println!("Sum {}", sum);

    Ok(())
}
