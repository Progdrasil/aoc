use std::collections::HashSet;

use itertools::Itertools;
use runner::Day;

pub struct Day6;

impl Day for Day6 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        Ok(input
            .trim()
            .char_indices()
            .collect_vec()
            .windows(4)
            .find_map(|c| {
                let set: HashSet<char> = c.iter().map(|(_, c)| *c).collect();
                (set.len() == 4).then_some(c.last().unwrap().0 + 1)
            })
            .unwrap()
            .to_string())
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        Ok(input
            .trim()
            .char_indices()
            .collect_vec()
            .windows(14)
            .find_map(|c| {
                let set: HashSet<char> = c.iter().map(|(_, c)| *c).collect();
                (set.len() == 14).then_some(c.last().unwrap().0 + 1)
            })
            .unwrap()
            .to_string())
    }

    fn day(&self) -> usize {
        6
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUTS: [(&str, &str, &str); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "7", "19"),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", "5", "23"),
        ("nppdvjthqldpwncqszvftbrmjlhg", "6", "23"),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10", "29"),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "11", "26"),
    ];

    #[test]
    fn part1() -> anyhow::Result<()> {
        for (input, expected, _) in INPUTS {
            let res = Day6.part1(input)?;
            assert_eq!(res, expected);
        }
        Ok(())
    }

    #[test]
    fn part2() -> anyhow::Result<()> {
        for (input, _, expected) in INPUTS {
            let res = Day6.part2(input)?;
            assert_eq!(res, expected);
        }
        Ok(())
    }
}
