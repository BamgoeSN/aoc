use std::{collections::VecDeque, io};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let mut grid: Vec<Vec<u8>> = vec![];
    let mut src: (usize, usize) = (0, 0);
    let mut dst: (usize, usize) = (0, 0);
    for (r, line) in input.trim().lines().enumerate() {
        let mut buf = vec![];
        for (c, b) in line.bytes().enumerate() {
            match b {
                b'S' => {
                    src = (r, c);
                    buf.push(0);
                }
                b'E' => {
                    dst = (r, c);
                    buf.push(25);
                }
                _ => {
                    buf.push(b - b'a');
                }
            }
        }
        grid.push(buf);
    }
    let (tr, tc) = (grid.len(), grid[0].len());

    // Puzzle 1
    let mut dist: Vec<Vec<Option<u32>>> = vec![vec![None; tc]; tr];
    dist[src.0][src.1] = Some(0);
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(src);

    while let Some((r, c)) = queue.pop_front() {
        let d = dist[r][c].unwrap();
        for (nr, nc) in DR
            .iter()
            .zip(DC.iter())
            .map(|(&dr, &dc)| (r.wrapping_add(dr), c.wrapping_add(dc)))
            .filter(|&(nr, nc)| nr < tr && nc < tc)
        {
            if dist[nr][nc].is_some() {
                continue;
            }
            if grid[nr][nc] > grid[r][c] + 1 {
                continue;
            }
            dist[nr][nc] = Some(d + 1);
            queue.push_back((nr, nc));
        }
    }

    let ans = dist[dst.0][dst.1].unwrap();
    println!("Puzzle 1: {}", ans);

    // Puzzle 2
    queue.clear();
    for row in dist.iter_mut() {
        row.fill(None);
    }
    for r in 0..tr {
        for c in 0..tc {
            if grid[r][c] == 0 {
                queue.push_back((r, c));
                dist[r][c] = Some(0);
            }
        }
    }

    while let Some((r, c)) = queue.pop_front() {
        let d = dist[r][c].unwrap();
        for (nr, nc) in DR
            .iter()
            .zip(DC.iter())
            .map(|(&dr, &dc)| (r.wrapping_add(dr), c.wrapping_add(dc)))
            .filter(|&(nr, nc)| nr < tr && nc < tc)
        {
            if dist[nr][nc].is_some() {
                continue;
            }
            if grid[nr][nc] > grid[r][c] + 1 {
                continue;
            }
            dist[nr][nc] = Some(d + 1);
            queue.push_back((nr, nc));
        }
    }

    let ans = dist[dst.0][dst.1].unwrap();
    println!("Puzzle 2: {}", ans);
}

const DR: [usize; 4] = [0, 1, 0, !0];
const DC: [usize; 4] = [1, 0, !0, 0];
