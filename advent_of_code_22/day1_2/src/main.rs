use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("day1_2/input.txt")?;
    let reader = BufReader::new(file);

    let mut sum: i32 = 0;
    let mut heap = BinaryHeap::<i32>::new();
    let mut n = 3;

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            heap.push(sum);
            sum = 0;
        } else {
            let number: i32 = line.parse().unwrap();
            sum += number;
        }
    }

    let mut total: i32 = 0;

    while let Some(x) = heap.pop() {
        println!("sum #{} = {}", n, x);
        total += x;
        n -= 1;
        if n <= 0 {
            break;
        }
    }

    println!("total: {}", total);

    Ok(())
}
