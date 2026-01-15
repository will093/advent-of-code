use crate::utils::parse::{AocParseExt};
use crate::define_solver;
use std::collections::HashMap;

type TachyonManifold = (Vec<Vec<char>>, usize);

define_solver!(
    Day7Solver,
    "2025",
    "07",
    TachyonManifold,
    preprocess,
    part_one,
    part_two
);

fn preprocess(input: &str) -> TachyonManifold {
    let grid: Vec<Vec<_>> = input.to_char_grid();
    let start_pos = grid[0].iter().position(|&c| c == 'S').unwrap();
    (grid, start_pos)
}

fn part_one((char_grid, start_pos): &TachyonManifold) -> String  {
    get_beam_splits(
        &char_grid, 
        0, 
        *start_pos as i32, 
        &mut HashMap::new()
    ).to_string()
}

fn part_two((char_grid, start_pos): &TachyonManifold) -> String {
    get_beam_quantum_splits(
        &char_grid, 
        0, 
        *start_pos as i32,
        &mut HashMap::new()
    ).to_string()
}

fn get_beam_splits(grid: &Vec<Vec<char>>, i: i32, j: i32, memo: &mut HashMap<String, bool>) -> u32 {
    let key = format!("{}-{}", i, j);
    if i >= grid.len() as i32 
        || j < 0 
        || j >= grid[0].len() as i32
        || memo.get(&key).is_some() {
        return 0;
    };
    memo.insert(key, true);

    match grid[i as usize][j as usize] {
        'S' => get_beam_splits(grid, i + 1, j, memo),
        '^' => 1 + get_beam_splits(grid, i, j + 1, memo) + get_beam_splits(grid, i, j - 1, memo),
        '.' => get_beam_splits(grid, i + 1, j, memo),
        ch => panic!("Unexpected char {}", ch)
    }
}

fn get_beam_quantum_splits(grid: &Vec<Vec<char>>, i: i32, j: i32, memo: &mut HashMap<String, u64>) -> u64 {
    let key = format!("{}-{}", i, j);
    let memo_val = memo.get(&key);
    if memo_val.is_some() { return *memo_val.unwrap(); }

    let total_paths: u64;

    if j < 0 || j >= grid[0].len() as i32 {
        total_paths = 0;
    } else if i >= grid.len() as i32 {
        total_paths = 1;
    } else {
        match grid[i as usize][j as usize] {
            'S' => { 
                total_paths = get_beam_quantum_splits(grid, i + 1, j, memo); 
            },
            '^' => { 
                total_paths = get_beam_quantum_splits(grid, i, j + 1, memo) + get_beam_quantum_splits(grid, i, j - 1, memo);
            },
            '.' => { 
                total_paths = get_beam_quantum_splits(grid, i + 1, j, memo);
            }
            ch => panic!("Unexpected char {}", ch)
        }
    }

    memo.insert(key, total_paths);
    total_paths
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beam_example() {
        let mut grid: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '.', '.', '.', 'S', '.'],
            vec!['.', '.', '.', '.', '.', '.', '^', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '^'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '^', '.', '^'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '^', '.', '^', '.', '^', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],

        ];
        let mut memo = HashMap::new();
        assert_eq!(get_beam_splits(&mut grid, 0, 6, &mut memo), 5);
    }
}

