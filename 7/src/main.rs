use std::fs;
use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let char_grid: Vec<Vec<char>> = fs::read_to_string("./input.txt")?
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start_pos = char_grid[0].iter().position(|&c| c == 'S').unwrap();

    let total_splits = beam(
        &char_grid, 
        0, 
        start_pos as i32, 
        &mut HashMap::new()
    );

    println!("Total splits: {}", total_splits);

    let total_quantum_splits = beam_quantum(
        &char_grid, 
        0, 
        start_pos as i32,
        &mut HashMap::new()
    );

    println!("Total quantum splits: {}", total_quantum_splits);
    Ok(())
}

fn beam(grid: &Vec<Vec<char>>, i: i32, j: i32, memo: &mut HashMap<String, bool>) -> u32 {
    let key = format!("{}-{}", i, j);
    if i >= grid.len() as i32 
        || j < 0 
        || j >= grid[0].len() as i32
        || memo.get(&key).is_some() {
        return 0;
    };
    memo.insert(key, true);

    match grid[i as usize][j as usize] {
        'S' => beam(grid, i + 1, j, memo),
        '^' => 1 + beam(grid, i, j + 1, memo) + beam(grid, i, j - 1, memo),
        '.' => beam(grid, i + 1, j, memo),
        ch => panic!("Unexpected char {}", ch)
    }
}

fn beam_quantum(grid: &Vec<Vec<char>>, i: i32, j: i32, memo: &mut HashMap<String, u64>) -> u64 {
    let key = format!("{}-{}", i, j);
    let memo_val = memo.get(&key);
    if memo_val.is_some() { return memo_val.unwrap().clone(); }

    let total_paths: u64;

    if j < 0 || j >= grid[0].len() as i32 {
        total_paths = 0;
    } else if i >= grid.len() as i32 {
        total_paths = 1;
    } else {
        match grid[i as usize][j as usize] {
            'S' => { 
                total_paths = beam_quantum(grid, i + 1, j, memo); 
            },
            '^' => { 
                total_paths = beam_quantum(grid, i, j + 1, memo) + beam_quantum(grid, i, j - 1, memo);
            },
            '.' => { 
                total_paths = beam_quantum(grid, i + 1, j, memo);
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
        assert_eq!(beam(&mut grid, 0, 6, &mut memo), 5);
    }
}
