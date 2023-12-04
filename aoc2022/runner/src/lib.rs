use std::{fs::File, io::Read, path::PathBuf};

use clap::{Arg, Command};

pub struct Runner {
    days: Vec<Box<dyn Day>>,
}

pub trait Day {
    fn part1(&self, input: &str) -> anyhow::Result<String>;
    fn part2(&self, input: &str) -> anyhow::Result<String>;
    fn day(&self) -> usize;
}

impl Runner {
    pub fn new(days: Vec<Box<dyn Day>>) -> Self {
        Self { days }
    }

    pub fn run(mut self) -> anyhow::Result<String> {
        self.days.sort_by_key(|a| a.day());
        let matches = Command::new("Advent Of Code")
            .author("Rene Leveille")
            .version("1")
            .about("Runner for advent of code")
            .arg(Arg::new("day").value_parser(clap::value_parser!(usize)))
            .arg(Arg::new("input").value_parser(clap::value_parser!(PathBuf)))
            .arg(
                Arg::new("part")
                    .short('p')
                    .default_value("1")
                    .value_parser(clap::value_parser!(u8).range(1..=2)),
            )
            .get_matches();
        let Some(day) = matches.get_one::<usize>("day") else  {
            anyhow::bail!("No day given")
        };
        let Some(input) = matches.get_one::<PathBuf>("input") else {
            anyhow::bail!("no input file given")
        };
        let Some(part) = matches.get_one::<u8>("part") else {
            unreachable!()
        };
        let Some(day_runner) = self.days.get(day-1) else {
            anyhow::bail!("Day {} not configured", day)
        };
        let mut input_file = File::open(input)?;
        let mut input = String::new();
        input_file.read_to_string(&mut input)?;

        match part {
            1 => day_runner.part1(&input),
            2 => day_runner.part2(&input),
            _ => unreachable!(),
        }
    }
}
