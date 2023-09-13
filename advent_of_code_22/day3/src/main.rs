use std::error::Error;
use std::fs;

fn get_priority(c: char) -> isize {
    if c.is_ascii_lowercase() {
        (c as isize) - ('a' as isize) + 1
    } else if c.is_ascii_uppercase() {
        (c as isize) - ('A' as isize) + 27
    } else {
        0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;
    let sum: isize = file
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (first, last) = line.split_at(line.len() / 2);
            for c in first.chars() {
                if last.contains(c) {
                    return get_priority(c);
                }
            }
            return 0;
        })
        .sum();

    println!("Sum {}", sum);

    Ok(())
}
