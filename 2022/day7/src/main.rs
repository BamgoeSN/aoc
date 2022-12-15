use std::{collections::HashMap, io};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let queries = parse(&input).unwrap().1;

    let mut root_content = HashMap::new();
    let mut queryit = queries.iter().cloned();
    let fquery = queryit.next().unwrap();
    initialize(&mut root_content, fquery, &mut queryit);
    let mut root_size = 0;
    set_foldersize(&mut root_content, &mut root_size);

    // Puzzle 1
    let mut ans: u64 = 0;
    search_dir(&root_content, root_size, 100000, &mut ans);
    println!("Puzzle 1: {}", ans);

    // Puzzle 2
    let mut list = vec![];
    dir_size(&root_content, root_size, &mut list);
    list.sort_unstable();
    let i = list.partition_point(|&x| x < 8729145);
    let v = list[i];
    println!("Puzzle 2: {}", v);
}

#[derive(Clone, Debug)]
enum Query<'a> {
    Cd(Option<&'a str>),
    Ls(Vec<LsType<'a>>),
}

#[derive(Clone, Copy, Debug)]
enum LsType<'a> {
    File(&'a str, u64),
    Folder(&'a str),
}

#[derive(Debug)]
enum Content<'a> {
    File {
        size: u64,
    },
    Folder {
        child: HashMap<&'a str, Content<'a>>,
        size: u64,
    },
}

fn initialize<'b, 'a: 'b>(
    curr: &mut HashMap<&'b str, Content<'a>>,
    query: Query<'a>,
    queryit: &mut impl Iterator<Item = Query<'a>>,
) {
    match query {
        Query::Cd(to) => {
            if let Some(to) = to {
                let entry = curr.entry(to).or_insert_with(|| Content::Folder {
                    child: HashMap::new(),
                    size: 0,
                });
                if let Content::Folder {
                    child: next,
                    size: _,
                } = entry
                {
                    if let Some(nq) = queryit.next() {
                        initialize(next, nq, queryit);
                    }
                }
            } else {
                return;
            }
        }
        Query::Ls(arr) => {
            for &x in arr.iter() {
                match x {
                    LsType::File(name, size) => {
                        curr.insert(name, Content::File { size });
                    }
                    LsType::Folder(name) => {
                        curr.insert(
                            name,
                            Content::Folder {
                                child: HashMap::new(),
                                size: 0,
                            },
                        );
                    }
                }
            }
        }
    }
    if let Some(nq) = queryit.next() {
        initialize(curr, nq, queryit);
    }
}

fn set_foldersize(curr: &mut HashMap<&str, Content>, ptr: &mut u64) -> u64 {
    for (_, cont) in curr.iter_mut() {
        match cont {
            Content::File { size } => {
                *ptr += *size;
            }
            Content::Folder { child, size } => {
                *ptr += set_foldersize(child, size);
            }
        }
    }
    *ptr
}

fn search_dir(curr: &HashMap<&str, Content>, mysize: u64, max_val: u64, sum: &mut u64) {
    if mysize <= max_val {
        *sum += mysize;
        return;
    }
    for (_, cont) in curr.iter() {
        if let Content::Folder { child, size } = cont {
            search_dir(child, *size, max_val, sum);
        }
    }
}

fn dir_size(curr: &HashMap<&str, Content>, mysize: u64, list: &mut Vec<u64>) {
    list.push(mysize);
    for (_, cont) in curr.iter() {
        if let Content::Folder { child, size } = cont {
            dir_size(child, *size, list);
        }
    }
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace0, not_line_ending},
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    Parser,
};

fn parse(input: &str) -> nom::IResult<&str, Vec<Query>> {
    preceded(
        tag("$ cd /\n"),
        separated_list0(line_ending, preceded(tag("$ "), alt((parse_cd, parse_ls)))),
    )(input)
}

fn parse_cd(input: &str) -> nom::IResult<&str, Query> {
    preceded(tag("cd "), not_line_ending)
        .map(|s| {
            if s == ".." {
                Query::Cd(None)
            } else {
                Query::Cd(Some(s))
            }
        })
        .parse(input)
}

fn parse_ls(input: &str) -> nom::IResult<&str, Query> {
    preceded(
        tag("ls\n"),
        separated_list0(
            line_ending,
            alt((
                separated_pair(complete::u64, multispace0, not_line_ending)
                    .map(|x| LsType::File(x.1, x.0)),
                preceded(tag("dir "), not_line_ending).map(LsType::Folder),
            )),
        )
        .map(Query::Ls),
    )(input)
}
