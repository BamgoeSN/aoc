use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let mut inst: Vec<(u64, u64)> = vec![];
    for line in input.lines() {
        if let Some((a, b)) = line.trim().split_once(' ') {
            let a = a.as_bytes()[0] - b'A';
            let b = b.as_bytes()[0] - b'X';
            inst.push((a.into(), b.into()));
        }
    }

    // Puzzle 1
    let mut score: u64 = 0;
    for &(opn, you) in inst.iter() {
        score += you
            + if (opn + 1) % 3 == you {
                7
            } else if opn == you {
                4
            } else {
                1
            };
    }
    println!("Puzzle 1: {}", score);

    // Puzzle 2
    let mut score = 0;
    for &(opn, res) in inst.iter() {
        let you = if res == 0 {
            opn + 2
        } else if res == 1 {
            opn
        } else {
            opn + 1
        } % 3;
        score += res * 3 + you + 1;
    }
    println!("Puzzle 2: {}", score);
}
