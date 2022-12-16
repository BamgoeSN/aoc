use std::{
    collections::{HashMap, HashSet},
    io,
    num::NonZeroI64,
};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let puzzle = parse(&input).unwrap().1;

    let mut valve_set = HashSet::new();
    let mut get_id = HashMap::new();
    for x in puzzle.iter() {
        if valve_set.insert(x.0) {
            get_id.insert(x.0, valve_set.len() - 1);
        }
    }

    let n = get_id.len();
    let mut graph = vec![vec![]; n];
    for x in puzzle.iter() {
        let id = *get_id.get(x.0).unwrap();
        for y in x.2.iter() {
            let j = *get_id.get(y).unwrap();
            graph[id].push(j);
        }
    }

    // Floyd-Warshall
    let mut grid: Vec<Vec<Option<NonZeroI64>>> = vec![vec![None; n]; n];
    for i in 0..n {
        for &j in graph[i].iter() {
            grid[i][j] = Some(NonZeroI64::new(1).unwrap());
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let (Some(u), Some(v)) = (grid[i][k], grid[k][j]) {
                    if grid[i][j].map_or(true, |w| w.get() > u.get() + v.get()) {
                        grid[i][j] = Some(NonZeroI64::new(u.get() + v.get()).unwrap());
                    }
                }
            }
        }
    }

    // Construct graphs between nonzero valves and source
    let meaningful_pts: Vec<&str> = puzzle
        .iter()
        .filter(|x| x.0 == "AA" || x.1 != 0)
        .map(|x| x.0)
        .collect();
    let src = meaningful_pts.iter().position(|&x| x == "AA").unwrap();
    let nonzero_ids: Vec<usize> = meaningful_pts
        .iter()
        .map(|k| *get_id.get(k).unwrap())
        .collect();
    let n = nonzero_ids.len();
    let mut graph = vec![vec![]; n];
    for (a, &i) in nonzero_ids.iter().enumerate() {
        for (b, &j) in nonzero_ids.iter().enumerate() {
            if a >= b {
                continue;
            }
            if let Some(v) = grid[i][j] {
                graph[a].push((b, v.get()));
                graph[b].push((a, v.get()));
            }
        }
    }
    for arr in graph.iter_mut() {
        arr.sort_unstable_by_key(|x| (x.1, x.0));
        arr.dedup();
    }

    let cap: Vec<i64> = nonzero_ids.iter().map(|&i| puzzle[i].1).collect();

    // Puzzle 1
    let mut visit = vec![false; n];
    visit[src] = true;
    let ans = solve1(src, 30, 0, 0, &graph, &cap, &mut visit);
    println!("Puzzle 1: {}", ans);
}

fn solve1(
    curr: usize,
    remain: i64,
    stay: i64,
    opened: i64,
    graph: &[Vec<(usize, i64)>],
    cap: &[i64],
    visit: &mut [bool],
) -> i64 {
    let mut ret = stay;
    for &(next, v) in graph[curr].iter() {
        if remain - v - 1 < 0 {
            continue;
        } else if visit[next] {
            continue;
        }
        visit[next] = true;
        let nexttime = remain - v - 1;
        let nextopen = opened + cap[next];
        let nextstay = stay + nexttime * cap[next];
        let x = solve1(next, nexttime, nextstay, nextopen, graph, cap, visit);
        ret = ret.max(x);
        visit[next] = false;
    }
    ret
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    multi::separated_list0,
    sequence::{pair, preceded},
    IResult, Parser,
};

fn parse(input: &str) -> IResult<&str, Vec<(&str, i64, Vec<&str>)>> {
    separated_list0(
        line_ending,
        pair(
            pair(
                preceded(tag("Valve "), alpha1),
                preceded(tag(" has flow rate="), complete::i64),
            ),
            preceded(
                alt((
                    tag("; tunnels lead to valves "),
                    tag("; tunnel leads to valve "),
                )),
                separated_list0(tag(", "), alpha1),
            ),
        )
        .map(|x| (x.0 .0, x.0 .1, x.1)),
    )(input)
}
