use std::{collections::HashSet, io};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let pairs = Vec::from_iter(
        input
            .lines()
            .filter(|s| !s.is_empty())
            .map(|line| parse(line).unwrap().1),
    );
    let beacons: HashSet<_> = pairs.iter().map(|p| &p.1).collect();

    // Puzzle 1
    let base_y = 2000000;
    let flat = get_unavail(&pairs, base_y);
    let mut ans: i64 = flat.iter().map(|&(s, t)| t - s).sum();
    for beacon in beacons.iter() {
        if beacon.1 == base_y {
            ans -= 1;
        }
    }
    println!("Puzzle 1: {}", ans);

    // Puzzle 2
    let x_max: i64 = 4000000;
    let y_max: i64 = 4000000;
    let mut avail = vec![];
    for y in 0..=y_max {
        let mut ret = vec![];
        let unavail_range = get_unavail(&pairs, y);
        let unavail_range = Vec::from_iter(
            unavail_range
                .into_iter()
                .filter(|&(s, t)| s <= x_max && t > 0),
        );
        if unavail_range.is_empty() {
            ret.push((0, x_max + 1));
        } else {
            if unavail_range[0].0 > 0 {
                ret.push((0, unavail_range[0].0));
            }
            if unavail_range[unavail_range.len() - 1].1 <= x_max {
                ret.push((unavail_range[0].1, x_max + 1));
            }
            if unavail_range.len() >= 2 {
                for wd in unavail_range.windows(2) {
                    ret.push((wd[0].1, wd[1].0));
                }
            }
        }
        avail.push(ret);
    }
    let mut pos: (i64, i64) = (0, 0);
    for y in 0..=y_max {
        if avail[y as usize].len() != 0 {
            pos = (avail[y as usize][0].0, y);
            break;
        }
    }
    println!("Puzzle 2: {}", pos.0 * 4000000 + pos.1);
}

fn parse(line: &str) -> nom::IResult<&str, ((i64, i64), (i64, i64))> {
    use nom::{
        bytes::complete::tag,
        sequence::{preceded, separated_pair},
    };
    separated_pair(
        preceded(
            tag("Sensor at x="),
            separated_pair(
                nom::character::complete::i64,
                tag(", y="),
                nom::character::complete::i64,
            ),
        ),
        tag(": "),
        preceded(
            tag("closest beacon is at x="),
            separated_pair(
                nom::character::complete::i64,
                tag(", y="),
                nom::character::complete::i64,
            ),
        ),
    )(line)
}

fn get_unavail(pairs: &[((i64, i64), (i64, i64))], base_y: i64) -> Vec<(i64, i64)> {
    let mut ranges: Vec<(i64, i64)> = vec![];
    for (sensor, beacon) in pairs.iter() {
        let dist = sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1);
        let dy = sensor.1.abs_diff(base_y);

        if dist < dy {
            continue;
        }
        let dx = dist - dy;
        ranges.push((sensor.0 - dx as i64, sensor.0 + dx as i64 + 1))
    }
    flatten_range(&mut ranges)
}

fn flatten_range(ranges: &mut [(i64, i64)]) -> Vec<(i64, i64)> {
    ranges.sort_unstable_by_key(|p| p.0);

    let mut ret = vec![];
    let mut start: i64 = 0;
    let mut term: Option<i64> = None;

    for p in ranges.iter() {
        if let Some(t) = term {
            if t < p.0 {
                ret.push((start, t));
                start = p.0;
                term = Some(p.1);
            } else {
                term = Some(t.max(p.1));
            }
        } else {
            start = p.0;
            term = Some(p.1);
        }
    }

    if let Some(t) = term {
        ret.push((start, t));
    }
    ret
}
