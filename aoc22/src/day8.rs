use itertools::Itertools;
use runner::Day;

pub struct Day8;

impl Day for Day8 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let map: Vec<Vec<u8>> = input
            .lines()
            .map(str::trim)
            .map(|line| {
                line.chars()
                    .filter_map(|n| n.to_digit(10))
                    .map(|n| n as u8)
                    .collect()
            })
            .collect();
        let rows = map.len();
        let cols = map[0].len();
        let mut visible = rows * 2 + cols * 2 - 4;

        // this is O(n^3) but oh well
        for r in 1..rows - 1 {
            for c in 1..cols - 1 {
                let tree = map[r][c];
                // check left
                let left = &map[r][..c];
                let right = &map[r][c + 1..];
                let mut above = map[..r].iter().map(|s| s[c]);
                let mut below = map[r + 1..].iter().map(|s| s[c]);
                let is_hidden = left.iter().any(|l| *l >= tree)
                    && right.iter().any(|l| *l >= tree)
                    && above.any(|l| l >= tree)
                    && below.any(|l| l >= tree);
                if !is_hidden {
                    visible += 1;
                }
            }
        }
        Ok(visible.to_string())
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        let map: Vec<Vec<u8>> = input
            .lines()
            .map(str::trim)
            .map(|line| {
                line.chars()
                    .filter_map(|n| n.to_digit(10))
                    .map(|n| n as u8)
                    .collect()
            })
            .collect();
        let rows = map.len();
        let cols = map[0].len();

        let mut scores = Vec::new();

        for r in 1..rows - 1 {
            for c in 1..cols - 1 {
                let tree = map[r][c];

                let left = &map[r][..c];
                let right = &map[r][c + 1..];
                let above = map[..r].iter().map(|s| s[c]).collect_vec();
                let below = map[r + 1..].iter().map(|s| s[c]).collect_vec();
                let score = count_visible(tree, left.iter().rev(), left.len())
                    * count_visible(tree, right, right.len())
                    * count_visible(tree, above.iter().rev(), above.len())
                    * count_visible(tree, &below, below.len());
                scores.push(score);
            }
        }
        Ok(scores.iter().max().unwrap().to_string())
    }

    fn day(&self) -> usize {
        8
    }
}

fn count_visible<'a, I: IntoIterator<Item = &'a u8>>(
    src: u8,
    direction: I,
    dir_len: usize,
) -> usize {
    let count = direction.into_iter().take_while(|u| **u < src).count();

    if dir_len > count {
        count + 1
    } else {
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r"30373
                          25512
                          65332
                          33549
                          35390";

    #[test]
    fn part1() -> anyhow::Result<()> {
        let res = Day8.part1(INPUT)?;
        assert_eq!(res, "21");
        Ok(())
    }

    #[test]
    fn part2() -> anyhow::Result<()> {
        let res = Day8.part2(INPUT)?;
        assert_eq!(res, "8");
        Ok(())
    }
}
