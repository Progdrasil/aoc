use std::collections::VecDeque;
use std::fmt::Display;

use itertools::Itertools;
use nom::bytes::streaming::tag;
use nom::character::complete::{anychar, char};
use nom::multi::{count, many1};
use nom::sequence::{delimited, preceded, tuple};
use nom::{IResult, Parser};
use nom_supreme::error::ErrorTree;
use nom_supreme::final_parser::{final_parser, Location};
use nom_supreme::ParserExt;
use runner::Day;

pub struct Day5;

impl Day for Day5 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
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
        todo!()
    }

    fn day(&self) -> usize {
        5
    }
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
    fn place(&mut self, new: char) {
        self.contents.push_front(new)
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
        assert_eq!(res, "");
        Ok(())
    }
}
