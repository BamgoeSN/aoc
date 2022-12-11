use std::{io, iter};

const DR: [usize; 4] = [1, 0, !0, 0];
const DC: [usize; 4] = [0, 1, 0, !0];

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let arr: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect();
    let tr = arr.len();
    let tc = arr[0].len();

    // Puzzle 1
    let is_visible: Vec<Vec<bool>> = (0..tr)
        .map(|r| (0..tc).map(|c| is_visible(&arr, r, c, tr, tc)).collect())
        .collect();
    let ans: usize = is_visible
        .iter()
        .map(|row| row.iter().filter(|&&b| b).count())
        .sum();
    println!("Puzzle 1: {}", ans);

    // Puzzle 2
    let ans: u64 = (0..tr)
        .filter_map(|r| (0..tc).map(|c| get_score(&arr, r, c, tr, tc)).max())
        .max()
        .unwrap();
    println!("Puzzle 2: {}", ans);
}

fn is_visible(arr: &[Vec<u8>], r: usize, c: usize, tr: usize, tc: usize) -> bool {
    'search: for (&dr, &dc) in iter::zip(DR.iter(), DC.iter()) {
        let (mut nr, mut nc) = (r.wrapping_add(dr), c.wrapping_add(dc));
        while nr < tr && nc < tc {
            if arr[nr][nc] >= arr[r][c] {
                continue 'search;
            }
            (nr, nc) = (nr.wrapping_add(dr), nc.wrapping_add(dc));
        }
        return true;
    }
    false
}

fn get_score(arr: &[Vec<u8>], r: usize, c: usize, tr: usize, tc: usize) -> u64 {
    let mut v: u64 = 1;
    for (&dr, &dc) in iter::zip(DR.iter(), DC.iter()) {
        let (mut nr, mut nc) = (r.wrapping_add(dr), c.wrapping_add(dc));
        let mut cnt: u64 = 0;
        while nr < tr && nc < tc {
            cnt += 1;
            if arr[nr][nc] >= arr[r][c] {
                break;
            }
            (nr, nc) = (nr.wrapping_add(dr), nc.wrapping_add(dc));
        }
        v *= cnt;
    }
    v
}
