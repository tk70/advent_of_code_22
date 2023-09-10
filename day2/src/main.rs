use std::error::Error;
use std::fs;

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => panic!("Wrong move"),
        }
    }
}

#[rustfmt::skip]
fn score(my_move: Move, opponent_move: Move) -> isize {
    match (my_move, opponent_move) {
        (Move::Rock, Move::Rock) =>         1 + 3,
        (Move::Rock, Move::Paper) =>        1 + 0,
        (Move::Rock, Move::Scissors) =>     1 + 6,
        (Move::Paper, Move::Rock) =>        2 + 6,
        (Move::Paper, Move::Paper) =>       2 + 3,
        (Move::Paper, Move::Scissors) =>    2 + 0,
        (Move::Scissors, Move::Rock) =>     3 + 0,
        (Move::Scissors, Move::Paper) =>    3 + 6,
        (Move::Scissors, Move::Scissors) => 3 + 3,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("input.txt")?;
    let lines = file.split("\n").filter(|&line| !line.is_empty());
    let mut sum = 0;
    for line in lines {
        assert!(line.len() >= 3);
        let opponent_move: Move = line.chars().nth(0).unwrap().into();
        let my_move: Move = line.chars().nth(2).unwrap().into();
        sum += score(my_move, opponent_move);
    }

    println!("Score! {}", sum);

    Ok(())
}
