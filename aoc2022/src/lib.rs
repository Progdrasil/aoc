use runner::Runner;

mod day1;

pub fn run() -> anyhow::Result<String> {
    let runner = Runner::new(vec![Box::new(day1::Day1)]);

    runner.run()
}
