use std::env;
use std::error::Error;
use std::time;

mod year_2025;
mod utils;

use utils::inputs::ProblemInputs;

macro_rules! time_execution {
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

    let problem_inputs = ProblemInputs::load_from_path("./inputs")?;
    let inputs = problem_inputs.get(year.as_deref(), day.as_deref());

    let solutions: Vec<Box<dyn utils::solver::Solver>> = vec![
        Box::new(year_2025::day_01::Day1Part1),
        Box::new(year_2025::day_01::Day1Part2),
        Box::new(year_2025::day_02::Day2Part1),
        Box::new(year_2025::day_02::Day2Part2),
        Box::new(year_2025::day_03::Day3Part1),
        Box::new(year_2025::day_03::Day3Part2),
        Box::new(year_2025::day_04::Day4Part1),
        Box::new(year_2025::day_04::Day4Part2),
        Box::new(year_2025::day_05::Day5Part1),
        Box::new(year_2025::day_05::Day5Part2),
        Box::new(year_2025::day_06::Day6Part1),
        Box::new(year_2025::day_06::Day6Part2),
        Box::new(year_2025::day_07::Day7Part1),
        Box::new(year_2025::day_07::Day7Part2),
        Box::new(year_2025::day_08::Day8Part1),
        Box::new(year_2025::day_08::Day8Part2),
        Box::new(year_2025::day_09::Day9Part1),
        Box::new(year_2025::day_09::Day9Part2),
        Box::new(year_2025::day_10::Day10Part1),
        Box::new(year_2025::day_10::Day10Part2),
        Box::new(year_2025::day_11::Day11Part1),
        Box::new(year_2025::day_11::Day11Part2), 
        Box::new(year_2025::day_12::Day12), 
    ];

    for input in inputs {
        let input_text = input.get_text()?;

        solutions
            .iter()
            .filter(|&solver| {
                let solver = solver.as_ref();
                solver.year() == input.year && solver.day() == input.day
            })
            .for_each(|solver| {
                let solver = solver.as_ref();
                let result = time_execution!(solver.label(), solver.solve(&input_text));
                println!("Result: {}", result);
                println!("- - - - - - - - - - - - - - - -");
            });
    }

    Ok(())
}