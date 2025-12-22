use std::env;
use std::error::Error;
use std::time;

mod year_2025;
mod utils;

macro_rules! print_run_time {
    ($label: expr, $expression:expr) => {{
        let start = time::SystemTime::now().duration_since(time::UNIX_EPOCH).expect("System time before Unix epoch");
        let result = $expression;
        let end = time::SystemTime::now().duration_since(time::UNIX_EPOCH).expect("System time before Unix epoch");
        let total_time = end - start;
        println!("{} executed in {:?}", $label, total_time);
        result
    }};
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let year = args.nth(1);
    let day = args.next();

    let inputs = utils::inputs::load("./inputs")?;

    // TODO: Run the solution for each input.

    let input = &inputs
        .iter()
        .find(|&y| y.name == "2025")
        .unwrap()
        .days
        .iter()
        .find(|&d| d.name == "day1.txt")
        .unwrap()
        .text;

    let result = print_run_time!("2025 Day 1", year_2025::day1::solve(input));

    Ok(())
}