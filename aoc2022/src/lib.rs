use runner::Runner;

mod day1;
mod day2;
mod day3;

pub fn run() -> anyhow::Result<String> {
    let runner = Runner::new(vec![
        Box::new(day1::Day1),
        Box::new(day2::Day2),
        Box::new(day3::Day3),
    ]);

    runner.run()
}
