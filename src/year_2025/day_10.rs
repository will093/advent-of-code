use std::collections::{BTreeSet};
use crate::utils::solver::Solver;
use crate::utils::parse::{AocParseExt};
use itertools::Itertools;

pub struct Day10Part1;
pub struct Day10Part2;

impl Solver for Day10Part1 {
    fn year(&self) -> &str { "2025" }
    fn day(&self) -> &str { "10" }
    fn label(&self) -> &str { "Day 10 Part 1" }
    fn solve(&self, input: &str) -> String {
        let mut parsed = parse(input);
        solve_one(&mut parsed)
    }
}

impl Solver for Day10Part2 {
    fn year(&self) -> &str { "2025" }
    fn day(&self) -> &str { "10" }
    fn label(&self) -> &str { "Day 10 Part 2" }
    fn solve(&self, input: &str) -> String {
        let mut parsed = parse(input);
        solve_two(&mut parsed)
    }
}

type Machine = (usize, Vec<usize>, Vec<i32>);

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(parse_machine)
        .collect()
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

fn solve_one(machines: &mut Vec<Machine>) -> String {
    machines
        .iter()
        .map(|m| configure_lights(m))
        .sum::<u32>()
        .to_string()
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

fn solve_two(machines: &mut Vec<Machine>) -> String {
    machines
        .iter()
        .enumerate()
        .map(|(i, m)| { 
            println!("machine {}", i);
            configure_joltage(m) 
        })
        .sum::<i32>()
        .to_string()
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
        current_free_vars = (0..width).collect();
    
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

            pivot_row += 1;
            pivot_col += 1;

            current_free_vars.remove(&pivot_col);
        }        
    }

    (current_free_vars, equations)
}

fn configure_joltage(machine: &Machine) -> i32 {
    
    // Step 1: Build a system of linear equations from the buttons and joltages
    let (width, height, maximums, mut equations) = build_linear_system(machine);

    println!("finished step 1");
    print_matrix(&equations);

    // Step 2 - reduce to row echelon form using Gaussian elimination
    // Row echelon form - we want to iterate through the cols and for each col find a pivot row whose value before the pivot should equal 0.
    // Ideally we want to reduce the pivot col to 1 to make the rest of the algorithm simpler.
    // We may need to iterate multiple times to achieve row echelon form in some cases.


    let (free_var_cols, equations) = reduce_linear_system(width, height, &mut equations);

    println!("finished step 2");
    print_matrix(&equations);

    // First imagine that all free vars are 0.
    // Sum all fixed cols and this is the presses from fixed vars
    // for each free var, free search a value between 0 and its max
    // subtract coefficient * val from the RHS of each equation
    // we *should* end up with a solution where RHS is 0 for rows with no fixed vars and positive for rows with fixed vars

    let pivot_row_count = width - 1 - free_var_cols.len();
    let presses = equations[..pivot_row_count].iter().flat_map(|v| v.iter().last()).sum::<i32>();

    let mut free_var_coefficients: Vec<Vec<i32>> = vec![vec![0; height]; free_var_cols.len()];
    let mut cost: Vec<i32> = vec![0; width - free_var_cols.len()];

    for i in 0..free_var_cols.len() {
        cost[i] = 1 - equations[..pivot_row_count].iter().map(|eq| eq[pivot_row_count + i]).sum::<i32>();
        for j in 0..height {
            free_var_coefficients[i][j] = equations[j][i+pivot_row_count]
        }
    }

    let mut rhs: Vec<Vec<i32>> = vec![vec![0; height]; free_var_cols.len() + 1];
    rhs[0] = equations
        .iter()
        .map(|eq| *eq.iter().last().unwrap())
        .collect();

    // Step 3: Brute force the free variables


    solve_reduced_equations(presses, &free_var_coefficients, &mut rhs, 0, pivot_row_count, &maximums, &cost).unwrap()
}

fn solve_reduced_equations(presses: i32, free_var_coefficients: &Vec<Vec<i32>>, rhs: &mut Vec<Vec<i32>>, depth: usize, pivot_row_count: usize, maximums: &Vec<i32>, cost: &Vec<i32>) -> Option<i32> {
    // println!("- - - - - Solving - - - - -");
    // println!("presses {}", presses);
    // println!("free var coefficients");
    // print_matrix(free_var_coefficients);
    // println!("rhs");
    // print_matrix(rhs);
    // println!("depth {}", depth);
    // println!("pivot row count {}", pivot_row_count);
    // println!("maximums {:?}", maximums);
    // println!("cost {:?}", cost);
    
    if depth == free_var_coefficients.len() {
        let pivot_rows_positive = rhs[depth].iter().take(pivot_row_count).all(|&val| val > 0);
        let non_pivot_rows_zero = rhs[depth].iter().skip(pivot_row_count).all(|&val| val == 0);
        if pivot_rows_positive && non_pivot_rows_zero {
            return Some(presses);
        } else {
            // println!("depth {:?} {}", rhs[depth], pivot_row_count);
            return None;
        }
    } else {
        (0..maximums[depth])
            .filter_map(|x| {
                let updated_presses = presses + (x * cost[depth]);

                for row in 0..rhs[0].len() {
                    // print_matrix(&rhs);
                    rhs[depth+1][row] = rhs[depth][row] - (x * free_var_coefficients[depth][row]);
                }

                solve_reduced_equations(updated_presses, free_var_coefficients, rhs, depth+1, pivot_row_count, maximums, cost)
            })
            .min()
    }

}

// TODO: move to helper
fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        for val in row {
            print!("{:4} ", val); // Adjust width as needed
        }
        println!();
    }
}

mod tests {

    use crate::year_2025::day_11::Day11Part2;

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

    // #[test]
    // fn configure_joltages_simple() {
    //     let input = "[#.##] (0) (0,2,3) (1,2) (2,3) {15,8,19,11}";
    //     let solver = Day10Part2;
    //     let res = solver.solve(input);
    //     assert_eq!(res, "99");
    // }

    #[test]
    fn configure_joltages_hard() {
        let input = "[.#.....##.] (0,3,6,9) (1,2,4,6,7,8,9) (0,5,9) (0,1,2,4,7,8) (0,6,7,8,9) (3,4,5,6,7,8) (0,3,5,7) (2,3,4,6,7) (2,3,4,5,6,7,8) (1,2,6,9) (2,7,8) (0) (0,2,3,4,5,6,7,9) {71,46,95,181,172,148,199,205,151,71}";
        let solver = Day10Part2;
        let res = solver.solve(input);
        assert_eq!(res, "99");
    }
}
