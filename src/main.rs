use std::env;
use std::error::Error;
use std::time;

mod year_2025;
mod utils;

use utils::inputs::ProblemInputs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let year = args.nth(1);
    let day = args.next();

    let problem_inputs = ProblemInputs::load_from_path("./inputs")?;
    let inputs = problem_inputs.get(year.as_deref(), day.as_deref());

    let solutions: Vec<Box<dyn utils::solver::SolverDyn>> = vec![
        Box::new(year_2025::day_01::Day1Solver),
        Box::new(year_2025::day_02::Day2Solver),
        Box::new(year_2025::day_03::Day3Solver),
        Box::new(year_2025::day_04::Day4Solver),
        Box::new(year_2025::day_05::Day5Solver),
        Box::new(year_2025::day_06::Day6Solver),
        Box::new(year_2025::day_07::Day7Solver),
        Box::new(year_2025::day_08::Day8Solver),
        Box::new(year_2025::day_09::Day9Solver),
        Box::new(year_2025::day_10::Day10Solver),
        Box::new(year_2025::day_11::Day11Solver),
        Box::new(year_2025::day_12::Day12Solver),
    ];

    let mut count = 0;
    time_execution!(format!("{} solvers", count), {
        for input in inputs {
            let input_text = input.get_text()?;

            solutions
                .iter()
                .filter(|&solver| {
                    let solver = solver.as_ref();
                    solver.year() == input.year && solver.day() == input.day
                })
                .for_each(|solver| {
                    count += 1;
                    solver.as_ref().solve(&input_text);
                });
        }
        print!("\n")
    });

    Ok(())
}