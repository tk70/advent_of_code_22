use std::fs;

fn main() {
    let f = fs::read_to_string("input.txt");
    let file = f.unwrap();
    let sum: usize = file
        .split(|c: char| !c.is_numeric())
        .filter(|token| !token.is_empty())
        .map(|value| value.parse::<isize>().unwrap())
        .collect::<Vec<_>>()
        .chunks_exact(4)
        .map(|group| {
            let ((al, ar), (bl, br)) = ((group[0], group[1]), (group[2], group[3]));
            let double_distance = (al + ar).abs_diff(bl + br);
            let size_sum = ar.abs_diff(al) + br.abs_diff(bl);
            (double_distance <= size_sum) as usize
        })
        .sum();
    print!("sum: {}", sum);
}
