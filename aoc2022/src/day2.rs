use std::ops::Add;

use runner::Day;

pub struct Day2;

impl Day for Day2 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        input
            .lines()
            .try_fold(0, |score, line| -> anyhow::Result<usize> {
                let mut split = line.trim().split(' ').map(Choices::try_from);
                let elf = split.next().unwrap()?;
                let me = split.next().unwrap()?;

                let round = elf.add(me);
                println!("{:?} vs {:?} = {}", elf, me, round);

                Ok(score + round)
            })
            .map(|score| score.to_string())
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        todo!()
    }

    fn day(&self) -> usize {
        2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choices {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for Choices {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "A" | "X" => Choices::Rock,
            "B" | "Y" => Choices::Paper,
            "C" | "Z" => Choices::Scissors,
            _ => anyhow::bail!("Invalid character: \"{}\"", value),
        })
    }
}

impl From<Choices> for usize {
    fn from(src: Choices) -> Self {
        match src {
            Choices::Rock => 1,
            Choices::Paper => 2,
            Choices::Scissors => 3,
        }
    }
}

impl PartialOrd for Choices {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}
impl Ord for Choices {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        match (self, rhs) {
            // wins
            (Choices::Rock, Choices::Paper)
            | (Choices::Paper, Choices::Scissors)
            | (Choices::Scissors, Choices::Rock) => std::cmp::Ordering::Greater,

            // Loses
            (Choices::Rock, Choices::Scissors)
            | (Choices::Paper, Choices::Rock)
            | (Choices::Scissors, Choices::Paper) => std::cmp::Ordering::Less,

            // draws
            (Choices::Rock, Choices::Rock)
            | (Choices::Paper, Choices::Paper)
            | (Choices::Scissors, Choices::Scissors) => std::cmp::Ordering::Equal,
        }
    }
}

const WIN: usize = 6;
const DRAW: usize = 3;
const LOSE: usize = 0;

impl Add for Choices {
    type Output = usize;

    fn add(self, rhs: Self) -> Self::Output {
        // convention LHS == elf, RHS == me
        (match self.cmp(&rhs) {
            // wins
            std::cmp::Ordering::Greater => WIN,

            // Loses
            std::cmp::Ordering::Less => LOSE,

            // draws
            std::cmp::Ordering::Equal => DRAW,
        }) + usize::from(rhs)
    }
}

#[cfg(test)]
mod tests {
    use runner::Day;

    use super::Day2;

    const INPUT: &str = r"A Y
                          B X
                          C Z";

    #[test]
    fn part1() -> anyhow::Result<()> {
        let response = Day2.part1(INPUT)?;

        assert_eq!(response, "15");
        Ok(())
    }
}
