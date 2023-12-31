use advent_of_code_22::utils::get_char_index;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("day3_2/input.txt")?;
    let sum: usize = file
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|elve_group| {
            let line1 = elve_group[0];
            let line2 = elve_group[1];
            let line3 = elve_group[2];
            for ch in line1.chars() {
                if line2.contains(ch) && line3.contains(ch) {
                    return get_char_index(ch);
                }
            }
            0
        })
        .sum();

    println!("Sum {}", sum);

    Ok(())
}
