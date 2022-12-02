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

                Ok(score + round)
            })
            .map(|score| score.to_string())
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        input
            .lines()
            .try_fold(0, |score, line| -> anyhow::Result<usize> {
                let mut split = line.trim().split(' ');
                let elf = split.next().map(Choices::try_from).unwrap()?;
                let me = split.next().map(Game::try_from).unwrap()?;

                let round = elf.add(me);

                Ok(score + round)
            })
            .map(|score| score.to_string())
    }

    fn day(&self) -> usize {
        2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
enum Choices {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Choices {
    fn rigged(self, result: Game) -> Self {
        match (self, result) {
            (Choices::Rock, Game::Win)
            | (Choices::Paper, Game::Draw)
            | (Choices::Scissors, Game::Lose) => Choices::Paper,

            (Choices::Paper, Game::Win)
            | (Choices::Rock, Game::Lose)
            | (Choices::Scissors, Game::Draw) => Choices::Scissors,

            (Choices::Scissors, Game::Win)
            | (Choices::Paper, Game::Lose)
            | (Choices::Rock, Game::Draw) => Choices::Rock,
        }
    }
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

impl Add for Choices {
    type Output = usize;

    fn add(self, rhs: Self) -> Self::Output {
        // convention LHS == elf, RHS == me
        (match self.cmp(&rhs) {
            // wins
            std::cmp::Ordering::Greater => Game::Win,

            // Loses
            std::cmp::Ordering::Less => Game::Lose,

            // draws
            std::cmp::Ordering::Equal => Game::Draw,
        } as usize)
            + (rhs as usize)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
enum Game {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl TryFrom<&str> for Game {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "X" => Game::Lose,
            "Y" => Game::Draw,
            "Z" => Game::Win,
            _ => anyhow::bail!("Unsupported character: \"{}\"", value),
        })
    }
}

impl Add<Game> for Choices {
    type Output = usize;

    fn add(self, rhs: Game) -> Self::Output {
        self + self.rigged(rhs)
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
