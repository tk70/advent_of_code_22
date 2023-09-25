use std::{
    collections::{HashMap, VecDeque},
    ptr::null_mut,
};

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

fn f5() {
    let mut map: std::collections::HashMap<String, isize> = HashMap::new();
    let mut val: *mut isize = std::ptr::null_mut::<isize>();
    map.insert(String::from("two"), 2);
    if let Some(s) = map.get_mut(&String::from("two")) {
        val = &mut *s;
    }
    unsafe {
        if !val.is_null() {
            *val = 3;
        }
    }
    dbg!(map);
}

fn f6() {
    let a = String::from("a");
    let b = String::from("b");
    let c = a + &b;
    dbg!(&c);
}

fn main() {
    // f1();
    // f2();
    // f3();
    // f4();
    // f5();
    f6();
}
