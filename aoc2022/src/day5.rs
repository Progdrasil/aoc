use std::{collections::VecDeque, fmt::Display};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char},
    multi::{count, many1},
    sequence::{delimited, preceded, tuple},
    IResult, Parser,
};
use nom_supreme::{final_parser::final_parser, ParserExt};
use runner::Day;

pub struct Day5;

impl Day for Day5 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let (mut stacks, moves) = parse(input);

        for mv in moves {
            for _i in 0..mv.num {
                let crt = stacks[mv.from()].take();
                stacks[mv.to()].place(crt);
            }
        }

        Ok(stacks
            .into_iter()
            .filter_map(|mut s| s.contents.pop_front())
            .join(""))
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        let (mut stacks, moves) = parse(input);

        for mv in moves {
            let crts = stacks[mv.from()].take_n(mv.num as usize);
            stacks[mv.to()].place_n(crts);
        }
        Ok(stacks
            .into_iter()
            .filter_map(|mut s| s.contents.pop_front())
            .join(""))
    }

    fn day(&self) -> usize {
        5
    }
}

fn parse(input: &str) -> (Vec<Stack>, Vec<Move>) {
    let mut stacks: Vec<Stack> = Vec::new();
    let mut lines = input.lines();

    loop {
        let next = lines.next().unwrap();
        let row = match crates(next) {
            Ok(r) => r.into_iter().enumerate(),
            Err(_) => break, // must be line of column numbers
        };

        if stacks.is_empty() {
            for (i, crt) in row {
                stacks.push(Stack::new(i, crt))
            }
        } else {
            for (i, crt) in row {
                stacks[i].append(crt)
            }
        }
    }
    assert!(lines.next().unwrap().is_empty());

    //moves
    let moves = lines.map(|line| moves(line).unwrap()).collect_vec();
    (stacks, moves)
}

fn crates(input: &str) -> Result<Vec<Option<char>>, nom::error::Error<&str>> {
    final_parser(many1(no_crate.or(delimited(
        char('[').opt_preceded_by(char(' ')),
        anychar.map(Some),
        char(']'),
    ))))(input)
}

fn no_crate(input: &str) -> IResult<&str, Option<char>> {
    count(char(' '), 3)
        .opt_preceded_by(char(' '))
        .map(|_| None)
        .parse(input)
}

struct Move {
    num: u8,
    src: usize,
    dst: usize,
}

impl Move {
    fn from(&self) -> usize {
        self.src - 1
    }

    fn to(&self) -> usize {
        self.dst - 1
    }
}

fn moves(input: &str) -> Result<Move, nom::error::Error<&str>> {
    final_parser(tuple((num, src, dst)).map(|(num, src, dst)| Move { num, src, dst }))(input)
}
fn num(input: &str) -> IResult<&str, u8> {
    preceded(tag("move "), nom::character::complete::u8).parse(input)
}
fn src(input: &str) -> IResult<&str, usize> {
    preceded(tag(" from "), nom::character::complete::u64)
        .map(|u| u as usize)
        .parse(input)
}

fn dst(input: &str) -> IResult<&str, usize> {
    preceded(tag(" to "), nom::character::complete::u64)
        .map(|u| u as usize)
        .parse(input)
}

#[derive(Debug)]
struct Stack {
    idx: usize,
    contents: VecDeque<char>,
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.idx, self.contents.iter().join(", "))
    }
}

impl Stack {
    fn new(idx: usize, first: Option<char>) -> Self {
        let mut stack = Self {
            contents: VecDeque::new(),
            idx: idx + 1,
        };
        stack.append(first);
        stack
    }
    fn append(&mut self, next: Option<char>) {
        if let Some(c) = next {
            self.contents.push_back(c);
        }
    }
    fn take(&mut self) -> char {
        self.contents.pop_front().unwrap()
    }

    fn take_n(&mut self, num: usize) -> Vec<char> {
        self.contents.drain(0..num).collect_vec()
    }
    fn place(&mut self, new: char) {
        self.contents.push_front(new)
    }
    fn place_n(&mut self, mut new: Vec<char>) {
        new.reverse();
        for c in new {
            self.contents.push_front(c)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    \n\
[N] [C]    \n\
[Z] [M] [P]\n\
 1   2   3\n\
\n\
move 1 from 2 to 1\n\
move 3 from 1 to 3\n\
move 2 from 2 to 1\n\
move 1 from 1 to 2";

    #[test]
    fn part1() -> anyhow::Result<()> {
        println!("{}", INPUT);
        let res = Day5.part1(INPUT)?;
        assert_eq!(res, "CMZ");
        Ok(())
    }

    #[test]
    fn part2() -> anyhow::Result<()> {
        let res = Day5.part2(INPUT)?;
        assert_eq!(res, "MCD");
        Ok(())
    }
}
