use runner::Day;
use std::{collections::VecDeque, io::BufRead};

pub struct Day1;

struct Elf {
    total_calories: usize,
    meals: Vec<usize>,
}

impl Elf {
    fn new() -> Self {
        Self {
            meals: Vec::new(),
            total_calories: 0,
        }
    }

    fn insert(&mut self, meal: usize) {
        self.total_calories += meal;
        self.meals.push(meal);
    }
}

struct Elves(VecDeque<Elf>);

impl Elves {
    fn new() -> Self {
        Self(VecDeque::new())
    }
    fn insert(&mut self, value: Elf) {
        // insert next elf into elves
        let index = self
            .0
            .partition_point(|elf| elf.total_calories > value.total_calories);
        self.0.insert(index, value);
    }

    fn first(&self) -> &Elf {
        self.0.front().unwrap()
    }
}

fn calculate_all_elves<R: BufRead>(read: R) -> Elves {
    let mut elves = Elves::new();
    let mut next_elf = Elf::new();
    for line in read.lines() {
        let line = line.expect("error reading line");
        let line = line.trim();
        if line.is_empty() {
            elves.insert(next_elf);
            next_elf = Elf::new();
        } else {
            let meal = line.parse().expect("not a number");
            next_elf.insert(meal);
        }
    }
    // insert last elf
    elves.insert(next_elf);

    elves
}

impl Day for Day1 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let elves = calculate_all_elves(input.as_bytes());
        Ok(elves.first().total_calories.to_string())
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        let mut elves = calculate_all_elves(input.as_bytes());
        Ok(elves.0.make_contiguous()[..3]
            .iter()
            .map(|e| e.total_calories)
            .sum::<usize>()
            .to_string())
    }

    fn day(&self) -> usize {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let input = r"1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000";
        let elf = Day1.part1(input).unwrap();
        assert_eq!(elf, "24000");
    }
}
