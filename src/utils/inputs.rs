use std::fs;
use std::error::Error;


#[derive(Debug, Clone)]
pub struct DayInput {
    pub name: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct YearInputs {
    pub name: String,
    pub days: Vec<DayInput>,
}



// TODO: pass down days and years for filtering what runs
pub fn load(path: &str) -> Result<Vec<YearInputs>, Box<dyn Error>> {
    let mut years_inputs = vec![];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let year_name = String::from(entry.file_name().to_string_lossy());
            let year_inputs = load_year(path, &year_name)?;
            years_inputs.push(year_inputs);
        }
    }

    Ok(years_inputs)
}

fn load_year(path: &str, year: &str) -> Result<YearInputs, Box<dyn Error>> {
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