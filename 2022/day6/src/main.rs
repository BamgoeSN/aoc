use std::{collections::HashSet, io};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let arr = input.as_bytes();

    // Puzzle 1
    let ans = solve(arr, 4);
    println!("Puzzle 1: {}", ans.unwrap());

    // Puzzle 2
    let ans = solve(arr, 14);
    println!("Puzzle 2: {}", ans.unwrap());
}

fn solve(arr: &[u8], wdsize: usize) -> Option<usize> {
    for (i, wd) in arr.windows(wdsize).enumerate() {
        let j = i + wdsize;
        if wd.iter().copied().collect::<HashSet<_>>().len() == wdsize {
            return Some(j);
        }
    }
    None
}
