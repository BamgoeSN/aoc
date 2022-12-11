use std::{cmp::Reverse, collections::HashMap, io, iter::FromIterator};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let lines = Vec::from_iter(input.lines());

    let mut monke_src = vec![];
    for chunk in lines.chunks(7) {
        let items: Vec<i64> = chunk[1]
            .split_whitespace()
            .skip(2)
            .map(|s| s.trim_end_matches(',').parse().unwrap())
            .collect();

        let ln2_tokens = Vec::from_iter(chunk[2].split_whitespace());
        let opr = if ln2_tokens[4] == "+" {
            let v: i64 = ln2_tokens[5].parse().unwrap();
            Opr::Add(v)
        } else {
            if ln2_tokens[5] == "old" {
                Opr::Sqr
            } else {
                let v: i64 = ln2_tokens[5].parse().unwrap();
                Opr::Mul(v)
            }
        };

        macro_rules! take_last {
            ($i:expr) => {
                chunk[$i]
                    .split_whitespace()
                    .rev()
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap()
            };
        }

        let check: i64 = take_last!(3);
        let iftrue: usize = take_last!(4);
        let iffalse: usize = take_last!(5);

        monke_src.push(Monkey {
            items,
            opr,
            test: (check, iftrue, iffalse),
        });
    }

    // Puzzle 1
    let mut counter: Vec<usize> = vec![0; monke_src.len()];
    let mut monke = monke_src.clone();
    for _round in 0..20 {
        for i in 0..monke.len() {
            counter[i] += monke[i].items.len();
            for it in 0..monke[i].items.len() {
                let old = monke[i].items[it];
                let new = match monke[i].opr {
                    Opr::Add(v) => old + v,
                    Opr::Mul(v) => old * v,
                    Opr::Sqr => old * old,
                } / 3;
                if new % monke[i].test.0 == 0 {
                    let to = monke[i].test.1;
                    monke[to].items.push(new);
                } else {
                    let to = monke[i].test.2;
                    monke[to].items.push(new);
                }
            }
            monke[i].items.clear();
        }
    }
    let mut order: Vec<usize> = (0..monke.len()).collect();
    order.sort_unstable_by_key(|&i| Reverse(counter[i]));
    let ans = counter[order[0]] * counter[order[1]];
    println!("Puzzle 1: {}", ans);

    // Puzzle 2
    let mut mods: Vec<i64> = monke.iter().map(|x| x.test.0).collect();
    mods.sort_unstable();
    mods.dedup();

    let mut modulomonke: Vec<ModuloMonkey> =
        monke_src.iter().map(|x| monke_modulo(x, &mods)).collect();
    let mut counter: Vec<usize> = vec![0; monke_src.len()];
    for _round in 0..10000 {
        for i in 0..modulomonke.len() {
            counter[i] += modulomonke[i].items.len();

            let mut old_items = vec![];
            std::mem::swap(&mut old_items, &mut modulomonke[i].items);
            let mut move_to: Vec<usize> = vec![];

            for item in old_items.iter_mut() {
                match modulomonke[i].opr {
                    Opr::Add(x) => {
                        for (&k, v) in item.iter_mut() {
                            *v = (*v + x) % k;
                        }
                    }
                    Opr::Mul(x) => {
                        for (&k, v) in item.iter_mut() {
                            *v = (*v * x) % k;
                        }
                    }
                    Opr::Sqr => {
                        for (&k, v) in item.iter_mut() {
                            *v = (*v * *v) % k;
                        }
                    }
                }
                if *item.get(&modulomonke[i].test.0).unwrap() == 0 {
                    move_to.push(modulomonke[i].test.1);
                } else {
                    move_to.push(modulomonke[i].test.2);
                }
            }

            for (i, item) in old_items.into_iter().enumerate() {
                modulomonke[move_to[i]].items.push(item);
            }
        }
    }

    let mut order: Vec<usize> = (0..monke.len()).collect();
    order.sort_unstable_by_key(|&i| Reverse(counter[i]));
    let ans = counter[order[0]] * counter[order[1]];
    println!("Puzzle 2: {}", ans);
}

#[derive(Clone, Copy, Debug)]
enum Opr {
    Add(i64),
    Mul(i64),
    Sqr,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i64>,
    opr: Opr,
    test: (i64, usize, usize),
}

#[derive(Clone, Debug)]
struct ModuloMonkey {
    items: Vec<HashMap<i64, i64>>, // (mod, val)
    opr: Opr,
    test: (i64, usize, usize),
}

fn monke_modulo(monke: &Monkey, mods: &[i64]) -> ModuloMonkey {
    ModuloMonkey {
        items: monke
            .items
            .iter()
            .map(|&v| mods.iter().map(|&m| (m, v % m)).collect())
            .collect(),
        opr: monke.opr,
        test: monke.test,
    }
}
