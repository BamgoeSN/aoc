use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let prog = parse(&input).unwrap().1;

    // Puzzle 1
    let mut cpu = Cpu::new();
    for &inst in prog.iter() {
        cpu.run(inst);
    }
    println!("Puzzle 1: {}", cpu.signal());

    // Puzzle 2
    println!("\nPuzzle 2");
    let arr = cpu.sprite();
    for line in arr {
        println!("{}", line);
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i64),
}

struct Cpu {
    reg: i64,
    history: Vec<i64>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            reg: 1,
            history: vec![0, 1],
        }
    }

    fn run(&mut self, inst: Instruction) {
        self.history.push(self.reg);
        if let Instruction::Addx(v) = inst {
            self.reg += v;
            self.history.push(self.reg);
        }
    }

    fn signal(&self) -> i64 {
        self.history
            .iter()
            .enumerate()
            .skip(20)
            .step_by(40)
            // .inspect(|x| println!("{:?}", x))
            .map(|(i, v)| i as i64 * v)
            .sum()
    }

    fn sprite(&self) -> Vec<String> {
        let mut buf = vec![];
        for (i, &v) in self.history.iter().skip(1).enumerate() {
            if v.abs_diff(i as i64 % 40) <= 1 {
                buf.push('#');
            } else {
                buf.push('.');
            }
        }
        buf.chunks(40)
            .map(|arr| String::from_iter(arr.iter().copied()))
            .filter(|s| s.len() == 40)
            .collect()
    }
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list0,
    sequence::preceded,
    IResult, Parser,
};

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list0(
        line_ending,
        alt((
            preceded(tag("addx "), complete::i64).map(|x| Instruction::Addx(x)),
            tag("noop").map(|_| Instruction::Noop),
        )),
    )(input)
}
