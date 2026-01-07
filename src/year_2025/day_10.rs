use std::env::var;
use std::iter::zip;

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
type Input = (Vec<i32>, Vec<i32>);

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
        .map(|m| configure_joltage(m))
        .sum::<u32>()
        .to_string()
} 


fn configure_joltage((_, buttons, joltages): &Machine) -> u32 {
    
    // Step 1: Build a system of linear equations from the buttons and joltages
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


    println!("original");
    print_matrix(&equations);

    // Step 2: convert to row echelon form using Gaussian elimination
    let mut pivot_row = 0;
    'outer: for col in 0..(width-1) {
        // Find the first row where col is not zero and can be reduced to 1.
        while let Some((row_index, _)) = equations[pivot_row..].iter().find_position(|&eq| eq[col] != 0 && eq.iter().all(|v| v % eq[col] == 0)) {
            let pivot_col = col;
            // Put the pivot row into the right position for row echelon form.
            equations.swap(pivot_row, row_index + pivot_row);

            let (pivot_equation, remaining_equations) = equations[pivot_row..].split_at_mut(1);
            
            // Divide each col in pivot row by pivot col (make pivot col equal to 1).
            pivot_equation[0] = pivot_equation[0].iter().map(|col| col / pivot_equation[0][pivot_col]).collect();


            for eq in remaining_equations.iter_mut() {
                let multiple = eq[pivot_col];
                for i in 0..eq.len() {
                    eq[i] = eq[i] - (multiple * pivot_equation[0][i]);
                }
            }

            pivot_row += 1;
            if pivot_row >= height {
                break 'outer;
            }
        }
    }

    println!("row echelon");
    print_matrix(&equations);

    // Step 3: Brute force the free variables
    let variables: Vec<Option<i32>> = vec![None; width - 1];
    let solutions = get_solutions(&variables, &equations);

    // println!("Solutions");
    // dbg!(&solutions);

    // Approach - give each variable a min and max, generate these combinations, calculate which solve the problem 
    // and then return the minimum of these.


    solutions.len() as u32
}

fn get_solutions(variables: &Vec<Option<i32>>, equations: &Vec<Vec<i32>>) -> Vec<Vec<Option<i32>>> {
    // println!("vars {:?}", variables);
    for eq in equations.iter().rev() {

        let rhs = *eq.iter().last().expect("equation should have variables");
        let lhs = &eq[..(eq.len()-1)];

        // Find the first free variable if there is one
        let free_var = lhs.iter().zip(variables).find_position(|&(coeff, var)| {
            *coeff != 0 && *var == None
        });

        // println!("free var {:?} {} {:?}", &free_var, rhs, eq);
        match free_var {
            // No free variables - check if it is a correct solution.
            None => {
                // println!("solution found {:?} {:?}", variables, eq);
                let sum_lhs = lhs.iter()
                    .zip(variables)
                    .fold(0,|acc, (coeff, var)| {
                        if *coeff == 0 {
                            return acc;
                        } else {
                            return var.expect("already established no free vars with non-zero coefficient") * coeff + acc
                        }
                    });
                if rhs == sum_lhs {
                    continue;
                } else {
                    return vec![];
                }
            },
            // Has a free variable - so we try fixing it to each possible value
            Some((index, _)) => {
                return (0..=40).flat_map(|value| {
                    let mut try_variables = variables.clone();
                    try_variables[index] = Some(value);
                    // println!("Vars to try");
                    // dbg!(&try_variables);
                    // get_solutions(&try_variables, equations)
                    return vec![];
                }).collect()
            },
            Some(_) => panic!("Should not reach this")
        }
    }

    vec![variables.clone()]
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
