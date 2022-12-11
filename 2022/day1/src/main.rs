use std::{cmp::Reverse, io};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let mut list = vec![];
    let mut buf = vec![];
    for line in input.lines() {
        if line.is_empty() {
            let mut new = vec![];
            std::mem::swap(&mut buf, &mut new);
            list.push(new);
        } else {
            let v: i64 = line.parse().unwrap();
            buf.push(v);
        }
    }
    if !buf.is_empty() {
        let mut new = vec![];
        std::mem::swap(&mut buf, &mut new);
        list.push(new);
    }

    let mut sum_list: Vec<i64> = list.iter().map(|arr| arr.iter().sum()).collect();

    // Puzzle 1
    let ans = *sum_list.iter().max().unwrap();
    println!("Puzzle 1: {}", ans);

    // Puzzle 2
    let (arr, _, _) = sum_list.select_nth_unstable_by_key(3, |&x| Reverse(x));
    let ans: i64 = arr.iter().sum();
    println!("Puzzle 2: {}", ans);
}
