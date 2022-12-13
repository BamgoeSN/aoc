use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let lines = Vec::from_iter(input.lines());

    let mut lists = vec![];
    for chunk in lines.chunks(3) {
        let mut set = vec![];
        for line in chunk.iter().take(2) {
            set.push(parse(line.as_bytes()));
        }
        lists.push(set);
    }

    // Puzzle 1
    let mut ans: usize = 0;
    for (i, list) in lists.iter().enumerate() {
        if list[0] <= list[1] {
            ans += i + 1;
        }
    }
    println!("Puzzle 1: {}", ans);

    // Puzzle 2
    let mut whole = Vec::from_iter(lists.into_iter().map(|it| it.into_iter()).flatten());
    let div1 = vec![List(vec![Val(2)])];
    whole.push(div1.clone());
    let div2 = vec![List(vec![Val(6)])];
    whole.push(div2.clone());
    whole.sort_unstable();

    let a = whole.partition_point(|x| x < &div1);
    let b = whole.partition_point(|x| x < &div2);
    println!("Puzzle 2: {}", (a + 1) * (b + 1));
}

#[derive(Clone, Debug)]
enum Elem {
    Val(i64),
    List(Vec<Elem>),
}
use Elem::*;

fn parse(line: &[u8]) -> Vec<Elem> {
    let mut ret = vec![];

    let mut ptr = 1;
    while ptr < line.len() - 1 {
        let l = ptr;
        if line[ptr] == b'[' {
            let mut cnt = 1;
            let mut pptr = ptr + 1;
            while cnt != 0 {
                if line[pptr] == b'[' {
                    cnt += 1;
                } else if line[pptr] == b']' {
                    cnt -= 1;
                }
                pptr += 1;
            }
            let r = pptr;
            ret.push(List(parse(&line[l..r])));
            ptr = r + 1;
        } else {
            let l = ptr;
            while line[ptr] != b',' && line[ptr] != b']' {
                ptr += 1;
            }
            let r = ptr;
            let token = std::str::from_utf8(&line[l..r]).unwrap();
            ret.push(Val(token.parse().unwrap()));
            ptr += 1;
        }
    }

    ret
}

impl PartialEq for Elem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Val(a), Val(b)) => a.eq(b),
            (Val(a), List(_)) => List(vec![Val(*a)]).eq(other),
            (List(_), Val(b)) => self.eq(&List(vec![Val(*b)])),
            (List(x), List(y)) => x.eq(y),
        }
    }
}

impl Eq for Elem {}

impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Val(a), Val(b)) => a.partial_cmp(b),
            (Val(a), List(_)) => List(vec![Val(*a)]).partial_cmp(other),
            (List(_), Val(b)) => self.partial_cmp(&List(vec![Val(*b)])),
            (List(x), List(y)) => x.partial_cmp(y),
        }
    }
}

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Val(a), Val(b)) => a.cmp(b),
            (Val(a), List(_)) => List(vec![Val(*a)]).cmp(other),
            (List(_), Val(b)) => self.cmp(&List(vec![Val(*b)])),
            (List(x), List(y)) => x.cmp(y),
        }
    }
}
