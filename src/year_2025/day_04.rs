use crate::{define_solver, utils::{parse::AocParseExt}};
use itertools::iproduct;

type RollCountGrid = Vec<Vec<i32>>;

define_solver!(
    Day4Solver,
    "2025",
    "04",
    RollCountGrid,
    preprocess,
    part_one,
    part_two
);

fn get_neighbour_indices<T>(grid: &Vec<Vec<T>>, (i_start, j_start): (usize, usize)) -> Vec<(usize, usize)> {
    iproduct!(
        0i32.max((i_start as i32)-1) as usize..grid.len().min(i_start+2), 
        0i32.max((j_start as i32)-1) as usize..grid[0].len().min(j_start+2)
    )
    .filter(|&(i,j)| !(i == i_start && j == j_start))
    .collect()
}

fn preprocess(input: &str) -> Vec<Vec<i32>> {
    let grid = input.to_char_grid();
    let mut roll_count_grid: Vec<Vec<i32>> = vec![vec![0; grid.len()]; grid[0].len()];
    for (i, j) in iproduct!(0..grid.len(), 0..grid[0].len()) {
        if grid[i][j] == '@' {
            // No of neighbours for each cell including itself, ie. max is 9.
            roll_count_grid[i][j] = 1 + get_neighbour_indices(&grid, (i, j))
                .into_iter()
                .filter(|&(i,j)| grid[i][j] == '@')
                .count() as i32
        }
    }

    roll_count_grid
}

fn part_one(input: &RollCountGrid) -> String {
    get_accessible_count(input).to_string()
}

fn part_two(input: &RollCountGrid) -> String {
    get_accessible_count_all(&mut input.clone()).to_string()
}

fn roll_is_accessible(roll: i32) -> bool {
    roll > 0 && roll < 5
}

fn get_accessible_count(grid: &Vec<Vec<i32>>) -> usize {
    iproduct!(0..grid.len(), 0..grid[0].len())
        .filter(|&(i,j)| roll_is_accessible(grid[i][j]))
        .count()
}

fn get_accessible_count_all(grid: &mut Vec<Vec<i32>>) -> usize {
    let mut stack: Vec<(usize, usize)> = vec![];
    let mut count = 0;

    for (i,j) in iproduct!(0..grid.len(), 0..grid[0].len()) {

        // Add next qualifying item to the stack otherwise
        if roll_is_accessible(grid[i][j]) {
            stack.push((i,j));
            grid[i][j] = 0;
        }
    }

    // pop from stack and process while items are in the stack
    while let Some((i, j)) = stack.pop() {
        count += 1;
        
        let paper_neighbours: Vec<_> = get_neighbour_indices(grid, (i, j))
            .into_iter()
            .collect();

        for &(i,j) in &paper_neighbours {
            grid[i][j] = 0.max(grid[i][j]-1);
        }
        stack.extend(paper_neighbours.iter().filter(|&&(i, j)| grid[i][j] == 4));
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_accessible_simple() {
        let grid = vec![
            vec![0, 0, 0, 4, 4],
            vec![0, 0, 0, 4, 5],
            vec![0, 0, 0, 0, 4],
            vec![0, 4, 6, 6, 5],
            vec![0, 4, 6, 6, 4],
        ];

        
        assert_eq!(get_accessible_count(&grid), 7);
    }

    #[test]
    fn get_accessible_all_test() {
        let mut grid = vec![
            vec![0, 0, 0, 4, 4],
            vec![0, 0, 0, 4, 5],
            vec![0, 0, 0, 0, 4],
            vec![0, 4, 6, 6, 5],
            vec![0, 4, 6, 6, 4],
        ];

        assert_eq!(get_accessible_count_all(&mut grid), 13);
    }
}