use std::{collections::HashSet, io, iter};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    // Puzzle 1
    let mut rope = Short::new(); // Can also use Rope::new(2);
    for line in input.lines() {
        let tokens = line.split_once(' ').unwrap();
        let dir = get_dir(tokens.0.as_bytes()[0]);
        let cnt: u32 = tokens.1.parse().unwrap();

        for _ in 0..cnt {
            rope.step(dir);
        }
    }
    println!("Puzzle 1: {}", rope.visit.len());

    // Puzzle 2
    let mut rope = Rope::new(10);
    for line in input.lines() {
        let tokens = line.split_once(' ').unwrap();
        let dir = get_dir(tokens.0.as_bytes()[0]);
        let cnt: usize = tokens.1.parse().unwrap();

        rope.step(dir, cnt);
    }
    println!("Puzzle 2: {}", rope.visit.len());
}

// RDLU
const DR: [i32; 4] = [0, 1, 0, -1];
const DC: [i32; 4] = [1, 0, -1, 0];

const DGR: [i32; 4] = [1, 1, -1, -1];
const DGC: [i32; 4] = [1, -1, -1, 1];

fn get_dir(dir: u8) -> usize {
    match dir {
        b'R' => 0,
        b'D' => 1,
        b'L' => 2,
        b'U' => 3,
        _ => 4,
    }
}

struct Short {
    head: (i32, i32),
    tail: (i32, i32),
    visit: HashSet<(i32, i32)>,
}

impl Short {
    fn new() -> Self {
        let mut visit = HashSet::new();
        visit.insert((0, 0));
        Self {
            head: (0, 0),
            tail: (0, 0),
            visit,
        }
    }

    fn step(&mut self, dir: usize) {
        let (dr, dc) = (DR[dir], DC[dir]);

        let old_head = self.head;
        self.head.0 += dr;
        self.head.1 += dc;
        if self.head.0.abs_diff(self.tail.0) >= 2 || self.head.1.abs_diff(self.tail.1) >= 2 {
            self.tail = old_head;
        }

        self.visit.insert(self.tail);
    }
}

struct Rope {
    pos: Vec<(i32, i32)>,
    visit: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(n: usize) -> Self {
        let mut visit = HashSet::new();
        visit.insert((0, 0));
        Self {
            pos: vec![(0, 0); n],
            visit,
        }
    }

    fn step(&mut self, dir: usize, cnt: usize) {
        for _ in 0..cnt {
            let (dr, dc) = (DR[dir], DC[dir]);
            self.pos[0].0 += dr;
            self.pos[0].1 += dc;
            for t in 1..self.pos.len() {
                let h = t - 1;
                if self.pos[h].0.abs_diff(self.pos[t].0) < 2
                    && self.pos[h].1.abs_diff(self.pos[t].1) < 2
                {
                    continue;
                }

                if self.pos[h].0 == self.pos[t].0 || self.pos[h].1 == self.pos[t].1 {
                    self.fetch_tail(h, t, &DR, &DC);
                } else {
                    self.fetch_tail(h, t, &DGR, &DGC);
                }
            }
            self.visit.insert(*self.pos.last().unwrap());
        }
    }

    fn fetch_tail(&mut self, h: usize, t: usize, rarr: &[i32], carr: &[i32]) {
        for (&dr, &dc) in iter::zip(rarr.iter(), carr.iter()) {
            self.pos[t].0 += dr;
            self.pos[t].1 += dc;

            if self.pos[h].0.abs_diff(self.pos[t].0) < 2
                && self.pos[h].1.abs_diff(self.pos[t].1) < 2
            {
                break;
            }
            self.pos[t].0 -= dr;
            self.pos[t].1 -= dc;
        }
    }
}
