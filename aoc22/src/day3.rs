use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use runner::Day;

pub struct Day3;

impl Day for Day3 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let priorities = priorities();
        Ok(input
            .lines()
            .map(|line| {
                let line = line.trim();
                let half_point = line.len() / 2;
                (&line[..half_point], &line[half_point..])
            })
            .map(|(first, second)| {
                let second = second.chars().collect::<HashSet<char>>();
                first
                    .chars()
                    .find(|c| second.contains(c))
                    .and_then(|c| priorities.get(&c))
                    .unwrap()
            })
            .sum::<usize>()
            .to_string())
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        let elf_groups = input
            .lines()
            .map(|line| {
                let line = line.trim();
                line.chars().collect::<HashSet<char>>()
            })
            .chunks(3);
        Ok(elf_groups
            .into_iter()
            .map(|group| {
                group
                    .reduce(|acc, next| acc.intersection(&next).copied().collect())
                    .and_then(|group| group.into_iter().next())
                    .unwrap()
            })
            .filter_map(|badge| priorities().get(&badge).copied())
            .sum::<usize>()
            .to_string())
    }

    fn day(&self) -> usize {
        3
    }
}

fn priorities() -> HashMap<char, usize> {
    ('a'..='z').chain('A'..='Z').zip(1..=52).collect()
}

#[cfg(test)]
mod test {
    use runner::Day;

    use super::Day3;

    const INPUT: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
                          jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                          PmmdzqPrVvPwwTWBwg
                          wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                          ttgJtRGJQctTZtZT
                          CrZsJsPPZsGzwwsLwLmpwMDw";
    #[test]
    fn part1() {
        let res = Day3.part1(INPUT).unwrap();

        assert_eq!(res, "157");
    }

    #[test]
    fn part2() {
        let res = Day3.part2(INPUT).unwrap();
        assert_eq!(res, "70");
    }
}
