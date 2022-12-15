use std::{cmp::Reverse, io};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let list = parse(&input).unwrap().1;
    let mut sum_list: Vec<i64> = list.iter().map(|arr| arr.iter().sum()).collect();

    // Puzzle 1
    let ans = *sum_list.iter().max().unwrap();
    println!("Puzzle 1: {}", ans);

    // Puzzle 2
    let (arr, _, _) = sum_list.select_nth_unstable_by_key(3, |&x| Reverse(x));
    let ans: i64 = arr.iter().sum();
    println!("Puzzle 2: {}", ans);
}

fn parse(input: &str) -> nom::IResult<&str, Vec<Vec<i64>>> {
    use nom::{bytes::complete::tag, multi::separated_list0};
    separated_list0(
        tag("\n"),
        separated_list0(tag("\n"), nom::character::complete::i64),
    )(input)
}
