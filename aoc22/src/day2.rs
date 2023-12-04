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

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
enum Choices {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Choices {
    fn rigged(self, result: Game) -> Self {
        let u = (2 + (result as usize)) % 3;
        let calc = ((self as usize) + u) % 3;
        match calc {
            0 => Choices::Scissors,
            1 => Choices::Rock,
            _ => Choices::Paper,
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

impl Add for Choices {
    type Output = usize;

    fn add(self, rhs: Self) -> Self::Output {
        // convention LHS == elf, RHS == me
        let result = (3 + (rhs as usize) - (self as usize)) % 3;
        3 * (match result {
            0 => Game::Draw,
            1 => Game::Win,
            _ => Game::Lose,
        } as usize)
            + (rhs as usize)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
enum Game {
    Win = 2,
    Draw = 1,
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
    #[test]
    fn part2() -> anyhow::Result<()> {
        let response = Day2.part2(INPUT)?;

        assert_eq!(response, "12");
        Ok(())
    }
}
