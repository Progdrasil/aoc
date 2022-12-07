use runner::Runner;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

pub fn run() -> anyhow::Result<String> {
    let runner = Runner::new(vec![
        Box::new(day1::Day1),
        Box::new(day2::Day2),
        Box::new(day3::Day3),
        Box::new(day4::Day4),
        Box::new(day5::Day5),
        Box::new(day6::Day6),
        Box::new(day7::Day7),
    ]);

    runner.run()
}
