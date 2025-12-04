use std::fs;


fn main() {
    let res = fs::read_to_string("./input.txt");
    match res {
        Ok(input) => {
            let grid: Vec<Vec<char>> = input
                .lines()
                .into_iter()
                .map(|line| line
                    .chars()
                    .collect()
                )
                .collect();

            let mut accessible_count = 0;
            for i in (0..grid.len()) {
                for j in (0..grid[0].len()) {
                    if is_accessible(&grid, i as i32, j as i32) {
                        accessible_count += 1;
                    };
                }
            }

            println!("Accessible: {}", accessible_count);
        },
        Err(e) => {
            panic!("{}", e)
        }
    }
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
        let grid = vec![
            vec!['.', '@', '.'],
            vec!['.', '@', '.'],
            vec!['.', '@', '.'],
        ];
        assert_eq!(is_paper(&grid, 0, 1), true);
        assert_eq!(is_paper(&grid, 0, 0), false);
    }

    #[test]
    fn is_paper_inside() {
        let grid = vec![
            vec!['.', '@', '.'],
            vec!['.', '@', '.'],
            vec!['.', '@', '.'],
        ];
        assert_eq!(is_paper(&grid, 1, 1), true);
    }

    #[test]
    fn is_paper_out_of_bounds() {
        let grid = vec![
            vec!['.', '@'],
            vec!['.', '@'],
        ];
        assert_eq!(is_paper(&grid, -1, -1), false);
        assert_eq!(is_paper(&grid, 1, -1), false);
        assert_eq!(is_paper(&grid, -1, 1), false);
    }


    #[test]
    fn is_accessible_inside() {
        let grid = vec![
            vec!['.', '@', '.', '@'],
            vec!['.', '@', '.', '@'],
            vec!['.', '@', '@', '.'],
            vec!['.', '@', '@', '@'],
        ];
        assert_eq!(is_accessible(&grid, 1, 1), true);
        assert_eq!(is_accessible(&grid, 2, 2), false);
    }

    #[test]
    fn is_accessible_outside() {
        let grid = vec![
            vec!['.', '@', '.', '@'],
            vec!['.', '@', '.', '@'],
            vec!['.', '@', '@', '.'],
            vec!['.', '@', '@', '@'],
        ];
        assert_eq!(is_accessible(&grid, 0, 1), true);
        assert_eq!(is_accessible(&grid, 3, 3), true);
    }


    #[test]
    fn is_accessible_no_paper() {
        let grid = vec![
            vec!['.', '@', '.', '@'],
            vec!['.', '.', '.', '@'],
            vec!['.', '.', '.', '.'],
            vec!['.', '@', '@', '@'],
        ];
        assert_eq!(is_accessible(&grid, 0, 0), false);
        assert_eq!(is_accessible(&grid, 1, 1), false);
    }

}