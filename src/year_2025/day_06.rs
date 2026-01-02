use crate::utils::solver::Solver;

pub struct Day6Part1;
pub struct Day6Part2;

impl Solver for Day6Part1 {
    fn year(&self) -> &str { "2025" }
    fn day(&self) -> &str { "06" }
    fn label(&self) -> &str { "Day 6 Part 1" }
    fn solve(&self, input: &str) -> String {
        solve_one(input)
    }
}

impl Solver for Day6Part2 {
    fn year(&self) -> &str { "2025" }
    fn day(&self) -> &str { "06" }
    fn label(&self) -> &str { "Day 6 Part 2" }
    fn solve(&self, input: &str) -> String {
        solve_two(input)
    }
}

fn solve_two(input: &str) -> String {

    let rows: Vec<Vec<&str>> = input
        .lines()
        .into_iter()
        .map(|x| x.split("").filter(|&x| x != "").collect())
        .collect();

    let mut curr_operator = "";
    let mut curr_value: u64 = 0;
    let total = (0..rows[0].len()).fold(0, |sum, i| {
        let col_operator = rows[rows.len() - 1][i];

        let is_spacer_col = rows
            .clone()
            .into_iter()
            .filter(|row| row[i] != " ")
            .count() == 0;

        if is_spacer_col {
            return sum + curr_value;
        }

        match col_operator {
            "*" => {
                curr_operator = col_operator;
                curr_value = 1;
            },
            "+" => {
                curr_operator = col_operator;
                curr_value = 0;
            },
            _ => {}
        }

        let column_num: u64 = rows[0..rows.len() - 1]
            .into_iter()
            .filter(|row| row[i].trim() != "")
            .fold("".to_string(), |acc, row| format!("{}{}", acc, row[i]))
            .parse::<u64>()
            .unwrap();

        match curr_operator {
            "*" => {
                curr_value = curr_value * column_num;
            },
            "+" => {
                curr_value = curr_value + column_num;
            },
            op => panic!("Unknown operator {}", op)
        }

        sum
    });

    (total + curr_value).to_string()
}


fn solve_one(input: &str) -> String {
    let nums: Vec<Vec<&str>> = input
        .lines()
        .into_iter()
        .map(|line| line.split(" ").filter(|x| x.trim() != "").collect())
        .collect();

    let out= (0..nums[0].len())
        .map(|i| {
            match nums[nums.len() - 1][i] {
                "*" => {
                    nums[0..nums.len() - 1].into_iter().fold(1, |acc, rows| acc * rows[i].parse::<u64>().unwrap())
                },
                "+" => {
                    nums[0..nums.len() - 1].into_iter().fold(0, |acc, rows| acc + rows[i].parse::<u64>().unwrap())
                },
                op => panic!("Unknown operator {}", op)

            }
        })
        .fold(0u64, |total, col_total| total + col_total);
    out.to_string()
}
