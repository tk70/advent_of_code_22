use advent_of_code_22::utils::get_char_index;
use std::{fs::File, io::Read, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pattern_length = 14;
    let path = Path::new("day6/input.txt");
    let file = File::open(&path)?;
    let mut char_registry = [0; get_char_index('z') as usize + 1];
    for (i, byte) in file.bytes().enumerate() {
        let character = byte? as char;
        let index = get_char_index(character) as usize;
        char_registry[index] = pattern_length;
        let unique_recent: isize = char_registry.into_iter().map(|ch| (ch > 0) as isize).sum();
        for ch in char_registry.iter_mut() {
            *ch -= 1;
        }
        if unique_recent >= pattern_length {
            println!("{}", i + 1);
            break;
        }
    }

    Ok(())
}
