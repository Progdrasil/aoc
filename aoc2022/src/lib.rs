use runner::Runner;

mod day1;
mod day2;

pub fn run() -> anyhow::Result<String> {
    let runner = Runner::new(vec![Box::new(day1::Day1), Box::new(day2::Day2)]);

    runner.run()
}
