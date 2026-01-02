use std::collections::BTreeMap;
use std::fs;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct ProblemInput {
    base_path: String,
    pub year: String,
    pub day: String,
}

impl ProblemInput {
    pub fn get_text(&self) -> Result<String, Box<dyn Error>> {
        let text = fs::read_to_string(format!("{}/{}/{}.txt", self.base_path, self.year, self.day))?;
        Ok(text)
    }
}

#[derive(Debug, Clone)]
pub struct ProblemInputs {
    inputs_map: BTreeMap<String, BTreeMap<String, ProblemInput>>,
}

impl ProblemInputs {

    pub fn load_from_path(path: &str) -> Result<ProblemInputs, Box<dyn Error>> {
        let mut inputs_map = BTreeMap::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let year = String::from(entry.file_name().to_string_lossy());
                let year_inputs_map = Self::load_year(path, &year)?;
                inputs_map.insert(year, year_inputs_map);
            }
        }


        Ok(ProblemInputs { inputs_map })
    }

    pub fn get(&self, year: Option<&str>, day: Option<&str>) -> Vec<&ProblemInput> {
        match (year, day) {
            (Some(year), Some(day)) => {
                let Some(problem) = self.day(&day, &year) else {
                    panic!("No input found for year {} day {}", year, day);
                };

                vec![problem]
            },
            (Some(year), None) => {
                self.year(&year)
            }
            (None, None) => {
                self.all().collect()
            },
            (None, Some(_)) => {
                vec![]
            }
        }
    }

    fn load_year(path: &str, year: &str) -> Result<BTreeMap<String, ProblemInput>, Box<dyn Error>> {
        let mut problem_inputs = BTreeMap::new();

        for entry in fs::read_dir(format!("{path}/{year}"))? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let day = String::from(entry.file_name().to_string_lossy());
                let day = day.split('.').next().expect("split results in at least 1 string");
                let day_problem_input = ProblemInput { year: year.to_string(), day: day.to_string(), base_path: path.to_string() };
                problem_inputs.insert(day.to_string(), day_problem_input);
            }
        }
        Ok(problem_inputs)
    }

    fn all(&self) -> impl Iterator<Item = &ProblemInput> {
        self.inputs_map
            .values()
            .flat_map(|day_map| day_map.values())
    }

    fn year(&self, year: &str) -> Vec<&ProblemInput> {
        let Some(y) = self.inputs_map.get(year) else {
            return vec![];
        };

        y.values().collect()
    }

    fn day(&self, day: &str, year: &str) -> Option<&ProblemInput> {
        let Some(y) = self.inputs_map.get(year) else {
            return None;
        };

        y.get(day)
    }
}
