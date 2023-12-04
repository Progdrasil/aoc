use std::collections::HashSet;

use itertools::Itertools;
use runner::Day;

pub struct Day4;

impl Day for Day4 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        Ok(input
            .lines()
            .filter_map(|line| line.trim().split_once(','))
            .map(|(first, second)| {
                (
                    Sections::try_from(first).unwrap(),
                    Sections::try_from(second).unwrap(),
                )
            })
            .filter(|(first, second)| first.completly_intersects(second))
            .count()
            .to_string())
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        Ok(input
            .lines()
            .filter_map(|line| {
                line.trim()
                    .split(',')
                    .map(|elf| Sections::try_from(elf).unwrap())
                    .collect_tuple()
            })
            .filter(|(first, second)| first.0.intersection(&second.0).count() != 0)
            .count()
            .to_string())
    }

    fn day(&self) -> usize {
        4
    }
}

struct Sections(HashSet<usize>);

impl Sections {
    fn completly_intersects(&self, other: &Self) -> bool {
        let intersection = self
            .0
            .intersection(&other.0)
            .copied()
            .collect::<HashSet<usize>>();
        intersection == self.0 || intersection == other.0
    }
}

impl TryFrom<&str> for Sections {
    type Error = std::num::ParseIntError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let (first, last) = src
            .split('-')
            .map(|i| i.parse::<usize>())
            .collect_tuple()
            .unwrap();
        Ok(Sections((first?..=last?).collect()))
    }
}

#[cfg(test)]
mod tests {
    use runner::Day;

    use super::Day4;

    const INPUT: &str = r"2-4,6-8
                          2-3,4-5
                          5-7,7-9
                          2-8,3-7
                          6-6,4-6
                          2-6,4-8";

    #[test]
    fn part1() -> anyhow::Result<()> {
        let res = Day4.part1(INPUT)?;
        assert_eq!(res, "2");
        Ok(())
    }

    #[test]
    fn part2() -> anyhow::Result<()> {
        let res = Day4.part2(INPUT)?;
        assert_eq!(res, "4");
        Ok(())
    }
}
