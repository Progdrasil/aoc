use itertools::Itertools;
use std::{collections::VecDeque, io::BufRead};

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

pub fn entrypoint<R: BufRead>(read: R) -> usize {
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

    elves.first().total_calories
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
        let elf = entrypoint(input.as_bytes());
        assert_eq!(elf, 24000);
    }
}
