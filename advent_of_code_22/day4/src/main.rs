use std::fs;

fn main() {
    let f = fs::read_to_string("day4/input.txt");
    let file = f.unwrap();
    let sum: usize = file
        .split(|c: char| !c.is_numeric())
        .filter(|token| !token.is_empty())
        .map(|value| value.parse::<isize>().unwrap())
        .collect::<Vec<_>>()
        .chunks_exact(4)
        .map(|group| {
            let within = |(al, ar), (bl, br)| -> bool { al >= bl && ar <= br };
            (within((group[0], group[1]), (group[2], group[3]))
                || within((group[2], group[3]), (group[0], group[1]))) as usize
        })
        .sum();
    print!("sum: {}", sum);
}
