use crate::utils::solver::Solver;
use regex::Regex;
use rand::Rng;
use good_lp::{variables, variable, SolverModel, Solution, constraint};
use good_lp::solvers::coin_cbc;
use std::time::Instant;

pub struct Day10Part1;
pub struct Day10Part2;

impl Solver for Day10Part1 {
    fn year(&self) -> &str { "2025" }
    fn day(&self) -> &str { "10" }
    fn label(&self) -> &str { "Day 10 Part 1" }
    fn solve(&self, input: &str) -> String {
        solve(input).0
    }
}

impl Solver for Day10Part2 {
    fn year(&self) -> &str { "2025" }
    fn day(&self) -> &str { "10" }
    fn label(&self) -> &str { "Day 10 Part 2" }
    fn solve(&self, input: &str) -> String {
        solve(input).1
    }
}

#[derive(Debug)]
struct Machine {
    joltage_current: Vec<i32>,
    joltage_configured: Vec<i32>,
    indicators_current: Vec<bool>,
    indicators_configured: Vec<bool>,
    switches: Vec<Vec<usize>>,
    switches_matrix: Vec<Vec<i32>>
}

impl Machine {

    fn pull_switch_config(&mut self, i: usize) {
        let switch = &self.switches[i];
        for n in switch {
            self.indicators_current[*n] = !self.indicators_current[*n];
        }
    }

    fn safe_pull_switch_joltage(&mut self, i: usize) -> bool {
        let switch = &self.switches[i];

        let mut safe = true;

        for n in switch {
            if self.joltage_current[*n] + 1 > self.joltage_configured[*n] {
                safe = false;
            }
        }
        if safe {
            for n in switch {
                self.joltage_current[*n] += 1;
            }
        }
        safe
    }

    fn new(joltage_configured: Vec<i32>, indicators_configured: Vec<bool>, switches: Vec<Vec<usize>>) -> Machine {

        let mut machine = Self {
            joltage_configured,
            indicators_configured,
            switches,
            joltage_current: vec![],
            indicators_current: vec![],
            switches_matrix: vec![],
        };

        machine.reset();

        machine.switches_matrix = transpose(machine.switches
            .iter()
            .map(|s| {
                let mut row: Vec<_> = machine.joltage_current.iter().map(|_| 0).collect();
                for n in s {
                    row[*n] = 1;
                }
                row
            })
            .collect());

        machine
    }

    fn reset(&mut self) {
        self.indicators_current = self.indicators_configured.iter().map(|_| false).collect();
        self.joltage_current = self.joltage_configured.iter().map(|_| 0).collect();
    }

    fn solve_with_good_lp(&self) -> i32 {
        let mut vars = variables!();

        let sw_vars: Vec<_> = self.switches
            .iter()
            .map(|_| vars.add(variable().min(0).integer()))
            .collect();

        let mut problem = vars
            .minimise(sw_vars.iter().sum::<good_lp::Expression>())
            .using(coin_cbc::coin_cbc);


        let constraints: Vec<_> = self.switches_matrix
            .iter()
            .zip(&self.joltage_configured)
            .map(|(sw_row, jolts)| { 
                let expr = sw_row.iter().zip(&sw_vars).fold(0.into(), |acc, (c, x)| *c * *x + acc);
                constraint!(expr == *jolts)
            })
            .collect();

        for c in constraints {
            problem = problem.with(c);
        }

        let solution = problem.solve().unwrap();

        let min_pulls: i32 = sw_vars.iter().map(|v| solution.value(*v) as i32).sum();

        min_pulls
    }
}

fn transpose<T: Clone>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() { return vec![]; }
    let row_len = matrix[0].len();
    (0..row_len)
        .map(|i| matrix.iter().map(|row| row[i].clone()).collect())
        .collect()
}

fn solve(input: &str) -> (String, String) {
    let start = Instant::now();
    
    let machines: Vec<Machine> = input
        .lines()
        .map(|line| {
            let indicators_re = Regex::new(r"\[(.*?)\]").unwrap();
            let indicators_configured: Vec<bool> = indicators_re
                .captures_iter(line)
                .next()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .split("")
                .filter(|x| x.len() > 0)
                .map(|x| if x == "#" { true } else { false })
                .collect();

            let switches_re = Regex::new(r"\((.*?)\)").unwrap();

            let switches: Vec<Vec<usize>> = switches_re
                .captures_iter(line)
                .filter_map(|cap| cap.get(1).map(|m| m.as_str()))
                .map(|s| s.split(",").map(|x| x.parse().unwrap()).collect())
                .collect();

            let joltage_re = Regex::new(r"\{(.*?)\}").unwrap();
            let joltage_configured: Vec<i32> = joltage_re
                .captures_iter(line)
                .next()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect();

            Machine::new(
                joltage_configured,
                indicators_configured,
                switches,
            )
        })
        .collect();

    let min_pulls_total: i32 = machines.iter().map(|m| m.solve_with_good_lp()).sum();

    (0.to_string(), min_pulls_total.to_string())
}


fn configure_joltage(machines: &mut Vec<Machine>) -> i32 {
    let mut rng = rand::thread_rng();

    let total = machines
        .iter_mut()
        .fold(0, |sum, machine| {
            let mut min_pull_count = 10000;

            let mut found_correct = false;
            for _ in 0..10000 {
                machine.reset();

                let mut pull_count = 0;
                for _ in 0..1000 {
                    if machine.joltage_current == machine.joltage_configured {
                        found_correct = true;
                        break;
                    }

                    let rand_switch_index = rng.gen_range(0..machine.switches.len());
                    let did_pull = machine.safe_pull_switch_joltage(rand_switch_index);

                    if !did_pull {
                        continue;
                    }

                    pull_count += 1
                }

                min_pull_count = min_pull_count.min(pull_count);
            }

            if min_pull_count == 10000 || !found_correct {
                panic!("Joltage could not be configured!");
            }

            sum + min_pull_count
        });

    total
}

fn configure_indicators(machines: &mut Vec<Machine>) -> i32 {
    let mut rng = rand::thread_rng();

    let total = machines
        .iter_mut()
        .fold(0, |sum, machine| {
            let mut min_pull_count = 10000;

            for _ in 0..10000 {
                machine.reset();

                let mut pull_count = 0;
                loop {
                    if machine.indicators_current == machine.indicators_configured {
                        break;
                    }

                    let rand_switch_index = rng.gen_range(0..machine.switches.len());
                    machine.pull_switch_config(rand_switch_index);
                    pull_count += 1
                }

                min_pull_count = min_pull_count.min(pull_count);
            }

            sum + min_pull_count
        });

    total
}