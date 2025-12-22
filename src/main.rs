use std::env;
use std::fs;
use std::error::Error;
use std::time;

mod year_2025;


#[derive(Debug, Clone)]
struct DayInput {
    name: String,
    text: String,
}

#[derive(Debug, Clone)]
struct YearInputs {
    name: String,
    days: Vec<DayInput>,
}


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

    let inputs = load_inputs("./inputs")?;

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


// TODO: pass down days and years for filtering what runs
fn load_inputs(path: &str) -> Result<Vec<YearInputs>, Box<dyn Error>> {
    let mut years_inputs = vec![];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let year_name = String::from(entry.file_name().to_string_lossy());
            let year_inputs = load_year_inputs(path, &year_name)?;
            years_inputs.push(year_inputs);
        }
    }

    Ok(years_inputs)
}

fn load_year_inputs(path: &str, year: &str) -> Result<YearInputs, Box<dyn Error>> {
    let mut day_inputs = vec![];
    for entry in fs::read_dir(format!("{path}/{year}"))? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let day_name = String::from(entry.file_name().to_string_lossy());
            let text = fs::read_to_string(entry.path())?;
            let day_input = DayInput { name: day_name, text };
            day_inputs.push(day_input);
        }
    }
    Ok(YearInputs { name: year.to_string(), days: day_inputs })
}