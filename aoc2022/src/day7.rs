use std::collections::HashMap;

use either::Either::{self, Left, Right};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, u64},
    combinator::rest,
    error::Error,
    sequence::{pair, preceded},
    IResult, Parser,
};
use nom_supreme::{final_parser::final_parser, ParserExt};
use runner::Day;

pub struct Day7;

impl Day for Day7 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let cli = input.lines().map(|l| line(l).unwrap()).collect_vec();
        let mut root = Node::new();
        let mut bread_crumbs = Vec::<String>::new();
        let mut current: &mut Node = &mut root;

        for line in cli {
            match line {
                Left(cmd) => match cmd {
                    Command::Ls => (), //ignore
                    Command::Cd(Directory::Root) => {
                        current = &mut root;
                        bread_crumbs.clear();
                    }
                    Command::Cd(Directory::Out) => {
                        // not efficient but I don't care
                        bread_crumbs.pop();
                        println!("bread crumbs: {:?}", bread_crumbs);
                        println!("nodes: {:#?}", root.children);
                        let mut iter = bread_crumbs.iter();
                        current = if let Some(name) = iter.next() {
                            root.children.get_mut(name).unwrap()
                        } else {
                            &mut root
                        };
                        for dir in iter {
                            current = current.children.get_mut(dir).unwrap()
                        }
                    }
                    Command::Cd(Directory::Name(name)) => {
                        current = current.children.get_mut(&name).unwrap();
                        bread_crumbs.push(name);
                    }
                },
                Right(elem) => {
                    // should be some here
                    match elem {
                        Element::Dir(name) => {
                            current.children.insert(name, Node::new());
                        }
                        Element::File(size, _) => current.size += size,
                    }
                }
            }
        }

        root.calc_sizes();

        let mut dirs = Vec::new();

        root.find_dirs("/".into(), &mut dirs);

        Ok(dirs
            .into_iter()
            .filter(|(_, sz)| *sz <= 100000)
            .map(|(_, sz)| sz)
            .sum::<u64>()
            .to_string())
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        todo!()
    }

    fn day(&self) -> usize {
        7
    }
}

#[derive(Debug)]
struct Node {
    size: u64,
    children: HashMap<String, Node>,
}

impl Node {
    fn new() -> Self {
        Self {
            size: 0,
            children: HashMap::new(),
        }
    }

    fn calc_sizes(&mut self) {
        // sooo inefficient
        for (_, node) in self.children.iter_mut() {
            node.calc_sizes();
            self.size += node.size;
        }
    }

    fn find_dirs(&self, name: String, acc: &mut Vec<(String, u64)>) {
        acc.push((name, self.size));
        for (name, child) in &self.children {
            child.find_dirs(name.clone(), acc);
        }
    }
}

#[derive(Clone)]
enum Command {
    Cd(Directory),
    Ls,
}

#[derive(Clone)]
enum Directory {
    Root,
    Out,
    Name(String),
}

fn command<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Command> {
    preceded(
        tag("$ "),
        tag("ls")
            .value(Command::Ls)
            .or(pair(tag("cd "), directory()).map(|(_, dir)| Command::Cd(dir))),
    )
}

fn directory<'a>() -> impl Parser<&'a str, Directory, Error<&'a str>> {
    tag("/")
        .value(Directory::Root)
        .or(tag("..").value(Directory::Out))
        .or(rest.map(|n: &str| Directory::Name(n.to_owned())))
}

enum Element {
    Dir(String),
    File(u64, String),
}

fn list<'a>() -> impl Parser<&'a str, Element, Error<&'a str>> {
    preceded(tag("dir "), rest)
        .map(|n: &str| Element::Dir(n.to_owned()))
        .or(pair(u64, multispace1.precedes(rest))
            .map(|(size, name): (_, &str)| Element::File(size, name.to_owned())))
}

fn line(input: &str) -> Result<Either<Command, Element>, Error<&str>> {
    final_parser(
        list()
            .map(Right)
            .or(command().map(Left))
            .preceded_by(multispace0),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r"$ cd /
                          $ ls
                          dir a
                          14848514 b.txt
                          8504156 c.dat
                          dir d
                          $ cd a
                          $ ls
                          dir e
                          29116 f
                          2557 g
                          62596 h.lst
                          $ cd e
                          $ ls
                          584 i
                          $ cd ..
                          $ cd ..
                          $ cd d
                          $ ls
                          4060174 j
                          8033020 d.log
                          5626152 d.ext
                          7214296 k";

    #[test]
    fn part1() -> anyhow::Result<()> {
        let res = Day7.part1(INPUT)?;
        assert_eq!(res, "95437");
        Ok(())
    }

    #[test]
    fn part2() -> anyhow::Result<()> {
        let res = Day7.part2(INPUT)?;
        assert_eq!(res, "");
        Ok(())
    }
}
