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
                time_execution!(solver.label(), solver.solve(&input_text));
            });
    }

    Ok(())
}