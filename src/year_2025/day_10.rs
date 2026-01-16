use crate::define_solver;
use std::collections::{BTreeSet};
use crate::utils::parse::{AocParseExt};
use itertools::Itertools;

define_solver!(
    Day10Solver,
    "2025",
    "10",
    Vec<Machine>,
    preprocess,
    part_one,
    part_two
);

type Machine = (usize, Vec<usize>, Vec<i32>);

fn preprocess(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(parse_machine)
        .collect()
}

fn part_one(machines: &Vec<Machine>) -> String {
    configure_lights_sum(machines).to_string()
}

fn part_two(machines: &Vec<Machine>) -> String {
    configure_joltages_sum(machines).to_string()
}

fn parse_machine(line: &str) -> Machine {
    let tokens: Vec<_> = line.split_ascii_whitespace().collect();
    let last = tokens.len() - 1;

    let lights = tokens[0]
        .bytes()
        .skip(1)
        .enumerate()
        .fold(0, |light, (i, b)| light | (usize::from(b == b'#') << i));

    let buttons = tokens[1..last]
        .iter()
        .map(|token| token.as_unsigned_iter::<usize>().fold(0, |button, i| button | (1 << i)))
        .collect();

    let joltages = tokens[last].as_signed_iter::<i32>().collect();

    (lights, buttons, joltages)
}

fn configure_lights_sum(machines: &Vec<Machine>) -> u32 {
    machines
        .iter()
        .map(configure_lights)
        .sum::<u32>()
} 

fn configure_lights((lights, buttons, _): &Machine) -> u32 {
    let mut pushes: u32 = 0;

    // We try all combinations of 1, 2... etc. pushes until we find one which works.
    loop { 
        let results: Vec<usize> = buttons
            .iter()
            .combinations(pushes as usize)
            .map(|combo| combo.into_iter().fold(0, |acc, b| acc ^ b))
            .collect();

        for result in results {
            if result == *lights {
                return pushes
            }
        }

        pushes += 1;
    }
}

fn configure_joltages_sum(machines: &Vec<Machine>) -> i32 {
    machines
        .iter()
        .map(configure_joltage)
        .sum::<i32>()
} 


fn configure_joltage(machine: &Machine) -> i32 {
    
    // Step 1: Build a system of linear equations from the buttons and joltages
    let (width, height, maximums, mut equations) = build_linear_system(machine);

    // Step 2 - reduce to row echelon form using Gaussian elimination
    // Row echelon form - we want to iterate through the cols and for each col find a pivot row whose value before the pivot should equal 0.
    // Ideally we want to reduce the pivot col to 1 to make the rest of the algorithm simpler.
    // We may need to iterate multiple times to achieve row echelon form in some cases.
    let (free_var_cols, equations) = reduce_linear_system(width, height, &mut equations);

    // Step 3: Brute force search the free variables
    // Our starting point is that all free variables are equal to zero, and 
    // the number of presses is calculated based on this from the fixed vars.
    let (
        pivot_row_count, 
        presses, 
        free_var_coefficients, 
        free_var_maximums, 
        free_var_costs, 
        mut rhs, 
        current_index
    ) = build_optimise_free_var_inputs(equations, width, height, maximums, free_var_cols);

    // Now we search all possible values of each free variable, 
    // with each level of recursion corresponding to a free variable index.
    optimise_free_vars(OptimiseFreeVarsInput { 
        pivot_row_count, 
        presses, 
        free_var_coefficients: &free_var_coefficients, 
        free_var_maximums: &free_var_maximums, 
        free_var_costs: &free_var_costs, 
        rhs: &mut rhs, 
        current_index 
    }).unwrap()
}

fn build_linear_system((_, buttons, joltages): &Machine) -> (usize, usize, Vec<i32>, Vec<Vec<i32>>) {
    let width = buttons.len() + 1;
    let height = joltages.len();
    let mut equations: Vec<Vec<i32>> = vec![vec![0; width]; height];

    for ((i, eq), joltage) in equations.iter_mut().enumerate().zip(joltages) {
        eq[width - 1] = *joltage;

        for (j, b) in buttons.iter().enumerate() {
            if (b & (1 << i)) != 0 {
                eq[j] = 1;
            }
        }
    }

    let mut maximums: Vec<i32> = vec![0; width - 1];
    for i in 0..width-1 {
        maximums[i] = equations.iter().filter(|eq| eq[i] != 0).map(|eq| eq[eq.len()-1] / eq[i]).min().unwrap();
    }

    (width, height, maximums, equations)
}

fn reduce_linear_system(width: usize, height: usize, equations: &mut Vec<Vec<i32>>) -> (BTreeSet<usize>,  &mut Vec<Vec<i32>>) {
    let mut current_free_vars: BTreeSet<_> = (0..width-1).collect();
    let mut prev_free_vars: BTreeSet<usize> = BTreeSet::new();

    while current_free_vars != prev_free_vars {
        let mut pivot_row = 0;
        let mut pivot_col = 0;

        prev_free_vars = current_free_vars;
        current_free_vars = (0..width-1).collect();
    
        while pivot_row < height && pivot_col < width - 1 {
            let Some((selected, _)) = equations[pivot_row..]
                .iter()
                .find_position(|eq| eq[pivot_col] != 0 && eq.iter().all(|val| (val % eq[pivot_col]) == 0)) else {
                    pivot_col += 1;
                    continue;
                };

            // Move the pivot row to its required location above all other pivot rows.
            equations.swap(pivot_row, selected + pivot_row);

            // Scale the pivot row so pivot col equals 1
            equations[pivot_row] = equations[pivot_row].iter().map(|val| val/equations[pivot_row][pivot_col]).collect();

            let pivot_equation = equations[pivot_row].clone();

            for (i, eq) in equations.iter_mut().enumerate() {
                if i != pivot_row {
                    let multiple = eq[pivot_col];

                    for j in 0..eq.len() {
                        eq[j] -= multiple * pivot_equation[j];
                    }
                }
            }

            current_free_vars.remove(&pivot_col);

            pivot_row += 1;
            pivot_col += 1;
        }        
    }

    (current_free_vars, equations)
}

fn build_optimise_free_var_inputs(equations: &Vec<Vec<i32>>, width: usize, height: usize, maximums: Vec<i32>, free_var_cols: BTreeSet<usize>) -> (usize, i32, Vec<Vec<i32>>, Vec<i32>, Vec<i32>, Vec<Vec<i32>>, usize) {
    let pivot_row_count = width - 1 - free_var_cols.len();
    let presses = equations[..pivot_row_count].iter().flat_map(|v| v.iter().last()).sum::<i32>();
    let mut free_var_coefficients: Vec<Vec<i32>> = vec![vec![0; height]; free_var_cols.len()];
    let mut free_var_maximums = vec![0; free_var_cols.len()];
    let mut free_var_costs: Vec<i32> = vec![0; width - free_var_cols.len()];
    let mut rhs: Vec<Vec<i32>> = vec![vec![0; height]; free_var_cols.len() + 1];
    let current_index = 0;

    for (i, &col_index) in free_var_cols.iter().enumerate() {
        for j in 0..height {
            free_var_coefficients[i][j] = equations[j][col_index]
        }
        free_var_maximums[i] = maximums[col_index];
        free_var_costs[i] = 1 - equations[..pivot_row_count].iter().map(|eq| eq[col_index]).sum::<i32>();
    }

    rhs[0] = equations
        .iter()
        .map(|eq| *eq.iter().last().unwrap())
        .collect();
    
    (pivot_row_count, presses, free_var_coefficients, free_var_maximums, free_var_costs, rhs, current_index)
}

struct OptimiseFreeVarsInput<'a> {
    pivot_row_count: usize,
    presses: i32, 
    free_var_coefficients: &'a Vec<Vec<i32>>, 
    free_var_maximums: &'a Vec<i32>, 
    free_var_costs: &'a Vec<i32>,
    rhs: &'a mut Vec<Vec<i32>>, 
    current_index: usize,
}

fn optimise_free_vars(OptimiseFreeVarsInput {
    pivot_row_count, 
    presses, 
    free_var_coefficients, 
    free_var_maximums, 
    free_var_costs,
    rhs, 
    current_index}: OptimiseFreeVarsInput) -> Option<i32> {

    if current_index == free_var_coefficients.len() {
        let pivot_rows_positive = rhs[current_index].iter().take(pivot_row_count).all(|&val| val >= 0);
        let non_pivot_rows_zero = rhs[current_index].iter().skip(pivot_row_count).all(|&val| val == 0);
        if pivot_rows_positive && non_pivot_rows_zero {
            return Some(presses);
        } else {
            return None;
        }
    } else {
        (0..=free_var_maximums[current_index])
            .filter_map(|x| {
                let updated_presses = presses + (x * free_var_costs[current_index]);

                for row in 0..rhs[0].len() {
                    rhs[current_index+1][row] = rhs[current_index][row] - (x * free_var_coefficients[current_index][row]);
                }

                optimise_free_vars(OptimiseFreeVarsInput {
                    pivot_row_count,
                    presses: updated_presses, 
                    free_var_coefficients, 
                    free_var_maximums, 
                    free_var_costs,
                    rhs,
                    current_index: current_index+1
                })
            })
            .min()
    }
}


mod tests {

    use crate::{utils::solver::Solver};
    use super::*;

    #[test]
    fn configure_lights_one_push() {
        let machine = (4, vec![4, 5], vec![]);
        let res = configure_lights(&machine);
        assert_eq!(res, 1);
    }


    #[test]
    fn configure_lights_two_pushes() {
        let machine = (4, vec![5, 1, 2], vec![]);
        let res = configure_lights(&machine);
        assert_eq!(res, 2);
    }

    #[test]
    fn configure_lights_example() {
        let input_1 = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        assert_eq!(Day10Solver.solve(input_1).0, "2");

        let input_2 = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        assert_eq!(Day10Solver.solve(input_2).0, "3");

        let input_3 = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(Day10Solver.solve(input_3).0, "2");
    }

    #[test]
    fn configure_joltages_simple() {
        let input = "[#.##] (0) (0,2,3) (1,2) (2,3) {15,8,19,11}";
        let res = Day10Solver.solve(input);
        assert_eq!(res.1, "23");
    }

    #[test]
    fn configure_joltages_example() {
        let input_1 = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        assert_eq!(Day10Solver.solve(input_1).1, "10");

        let input_2 = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        assert_eq!(Day10Solver.solve(input_2).1, "12");

        let input_3 = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(Day10Solver.solve(input_3).1, "11");
    }
}