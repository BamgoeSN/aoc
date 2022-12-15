use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let prim_list = Vec::from_iter(input.lines().map(|line| parse(line)));
    let list: Vec<Vec<(usize, usize)>> = Vec::from_iter(prim_list.into_iter().map(|list| {
        list.into_iter()
            .map(|(r, c)| (r + 1000, c + 1000))
            .collect()
    }));

    let mxr = list
        .iter()
        .map(|arr| arr.iter())
        .flatten()
        .map(|&(r, _)| r)
        .max()
        .unwrap();
    let mxc = list
        .iter()
        .map(|arr| arr.iter())
        .flatten()
        .map(|&(_, c)| c)
        .max()
        .unwrap();
    let (tr, tc) = (mxr + 1000, mxc + 1000);

    let mut cave = Cave::new((tr, tc), &list, (1000, 1500));

    // Puzzle 1
    while cave.put_sand() {}
    println!("Puzzle 1: {}", cave.sand);

    // Puzzle 2
    let mut nlist = list.clone();
    nlist.push(vec![(mxr + 2, 0), (mxr + 2, tc - 1)]);
    let mut ncave = Cave::new((tr, tc), &nlist, (1000, 1500));
    while ncave.put_sand() {}
    println!("Puzzle 2: {}", ncave.sand);
}

fn parse(line: &str) -> Vec<(usize, usize)> {
    use nom::{bytes::complete::tag, character::complete, multi::separated_list0};
    use nom::{sequence::separated_pair, Parser};
    let res: Result<_, nom::Err<nom::error::Error<&str>>> = separated_list0(
        tag(" -> "),
        separated_pair(complete::u64, tag(","), complete::u64)
            .map(|(a, b)| (b as usize, a as usize)),
    )(line);
    res.unwrap().1
}

struct Cave {
    grid: Vec<Vec<Option<()>>>, // Some if occupied, None if empty
    sand: usize,
    tsize: (usize, usize),
    src: (usize, usize),
}

impl Cave {
    fn new(tsize: (usize, usize), list: &[Vec<(usize, usize)>], src: (usize, usize)) -> Self {
        let (tr, tc) = tsize;
        let mut grid = vec![vec![None; tc]; tr];

        for arr in list.iter() {
            for wd in arr.windows(2) {
                let from = wd[0];
                let to = wd[1];
                if from.0 == to.0 {
                    let mn = from.1.min(to.1);
                    let mx = from.1.max(to.1);
                    for c in mn..=mx {
                        grid[to.0][c] = Some(());
                    }
                } else {
                    let mn = from.0.min(to.0);
                    let mx = from.0.max(to.0);
                    for r in mn..=mx {
                        grid[r][to.1] = Some(());
                    }
                }
            }
        }
        Self {
            grid,
            sand: 0,
            tsize,
            src,
        }
    }

    fn put_sand(&mut self) -> bool {
        let mut ptr = self.src;
        if self.grid[ptr.0][ptr.1].is_some() {
            return false;
        }

        while ptr.0 < self.tsize.0 && ptr.1 < self.tsize.1 {
            let (r, c) = ptr;
            for (nr, nc) in DC
                .iter()
                .map(|&dc| (r + 1, c.wrapping_add(dc)))
                .filter(|&(nr, nc)| nr < self.tsize.0 && nc < self.tsize.1)
            {
                if self.grid[nr][nc].is_none() {
                    ptr = (nr, nc);
                    break;
                }
            }
            if (r, c) == ptr {
                break;
            }
        }

        if ptr.0 < self.tsize.0 - 1 && ptr.1 < self.tsize.1 {
            self.grid[ptr.0][ptr.1] = Some(());
            self.sand += 1;
            true
        } else {
            false
        }
    }
}

const DC: [usize; 3] = [0, !0, 1];
