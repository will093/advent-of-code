use crate::define_solver;
use std::u32;

define_solver!(
    Day4Solver,
    "2025",
    "04",
    String,
    preprocess,
    part_one,
    part_two
);

fn preprocess(input: &str) -> String {
    String::from(input)
}

fn part_one(input: &str) -> String {
    solve(input, false)
}

fn part_two(input: &str) -> String {
    solve(input, true)
}

fn solve(input: &str, multi_iter: bool) -> String {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .into_iter()
        .map(|line| line
            .chars()
            .collect()
        )
        .collect();

    let mut accessible_count = 0;

    let iter_count = if multi_iter { u32::MAX } else { 1 };
    for _ in 0..iter_count {
        let mut iteration_accessible_count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if is_accessible(&grid, i as i32, j as i32) {
                    iteration_accessible_count += 1;
                    if multi_iter {
                        grid[i][j] = 'X';
                    }
                };
            }
        }

        if iteration_accessible_count == 0 {
            break;
        }

        accessible_count += iteration_accessible_count;
    }
    accessible_count.to_string()
}



fn is_accessible(grid: &Vec<Vec<char>>, i: i32, j: i32) -> bool {

    let threshold = 4;

    let is_current_paper = is_paper(grid, i, j);
    
    let is_surrounding_paper = vec![
        is_paper(grid, i-1, j-1),
        is_paper(grid, i-1, j),
        is_paper(grid, i-1, j+1),
        is_paper(grid, i, j-1),
        is_paper(grid, i, j+1),
        is_paper(grid, i+1, j-1),
        is_paper(grid, i+1, j),
        is_paper(grid, i+1, j+1),
    ];

    let surrounding_paper_count = is_surrounding_paper
        .into_iter()
        .filter(|&x| x)
        .count();

    is_current_paper && surrounding_paper_count < threshold
}


fn is_paper(grid: &Vec<Vec<char>>, i: i32, j: i32) -> bool {

    if i >= grid.len() as i32 || i < 0 {
        return false;
    }

    if j >= grid[0].len() as i32 || j < 0 {
        return false;
    }

    grid[i as usize][j as usize] == '@'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_paper_edge() {
        let mut grid = vec![
            vec!['.', '@', '.'],
            vec!['.', '@', '.'],
            vec!['.', '@', '.'],
        ];
        assert_eq!(is_paper(&mut grid, 0, 1), true);
        assert_eq!(is_paper(&mut grid, 0, 0), false);
    }

    #[test]
    fn is_paper_inside() {
        let mut grid = vec![
            vec!['.', '@', '.'],
            vec!['.', '@', '.'],
            vec!['.', '@', '.'],
        ];
        assert_eq!(is_paper(&mut grid, 1, 1), true);
    }

    #[test]
    fn is_paper_out_of_bounds() {
        let mut grid = vec![
            vec!['.', '@'],
            vec!['.', '@'],
        ];
        assert_eq!(is_paper(&mut grid, -1, -1), false);
        assert_eq!(is_paper(&mut grid, 1, -1), false);
        assert_eq!(is_paper(&mut grid, -1, 1), false);
    }


    #[test]
    fn is_accessible_inside() {
        let mut grid = vec![
            vec!['.', '@', '.', '@'],
            vec!['.', '@', '.', '@'],
            vec!['.', '@', '@', '.'],
            vec!['.', '@', '@', '@'],
        ];
        assert_eq!(is_accessible(&mut grid, 1, 1), true);
        assert_eq!(is_accessible(&mut grid, 2, 2), false);
    }

    #[test]
    fn is_accessible_outside() {
        let mut grid = vec![
            vec!['.', '@', '.', '@'],
            vec!['.', '@', '.', '@'],
            vec!['.', '@', '@', '.'],
            vec!['.', '@', '@', '@'],
        ];
        assert_eq!(is_accessible(&mut grid, 0, 1), true);
        assert_eq!(is_accessible(&mut grid, 3, 3), true);
    }


    #[test]
    fn is_accessible_no_paper() {
        let mut grid = vec![
            vec!['.', '@', '.', '@'],
            vec!['.', '.', '.', '@'],
            vec!['.', '.', '.', '.'],
            vec!['.', '@', '@', '@'],
        ];
        assert_eq!(is_accessible(&mut grid, 0, 0), false);
        assert_eq!(is_accessible(&mut grid, 1, 1), false);
    }

}