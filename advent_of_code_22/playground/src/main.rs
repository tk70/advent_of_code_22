use std::collections::VecDeque;

fn f1() {
    let mut src: VecDeque<isize> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let mut dst: VecDeque<isize> = vec![11, 12, 13, 14, 15].into_iter().collect();
    let mut drained = src.drain(1..=2);
    dst.extend(&mut drained);
    drop(drained);
    dbg!(src);
    dbg!(dst);
}

fn f2() {
    let mut src: VecDeque<isize> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let mut dst: VecDeque<isize> = vec![11, 12, 13, 14, 15].into_iter().collect();
    let drained = src.drain(1..=2);
    dst.extend(drained);
    dbg!(src);
    dbg!(dst);
}

fn f3() {
    let mut src: VecDeque<isize> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let mut dst: VecDeque<isize> = vec![11, 12, 13, 14, 15].into_iter().collect();
    let drained = src.drain(1..=2);
    let mut collected = drained.collect::<VecDeque<isize>>();
    dst.append(&mut collected);
    dbg!(src);
    dbg!(dst);
}

fn f4() {
    let mut src: VecDeque<isize> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let mut dst: VecDeque<isize> = vec![11, 12, 13, 14, 15].into_iter().collect();
    let drained = src.drain(1..=2);
    dst.append(&mut drained.collect::<VecDeque<isize>>());
    dbg!(src);
    dbg!(dst);
}

fn main() {
    f1();
    f2();
    f3();
    f4();
}
