use crate::utils::solver::Solver;

pub struct Day12;

impl Solver for Day12 {
    fn year(&self) -> &str { "2025" }
    fn day(&self) -> &str { "12" }
    fn label(&self) -> &str { "Day 12 Part 1" }
    fn solve(&self, input: &str) -> String {
        solve(input)
    }
}

#[derive(Debug)]
struct Region {
    width: u32,
    height: u32,
    shape_counts: Vec<u32>,
}

fn solve(input: &str)-> String {

    let mut lines = input.lines();

    let mut block_counts = vec![];
    for _ in 0..6 {
        let _index = lines.next(); 
        let row_1 = lines.next().unwrap(); 
        let row_2 = lines.next().unwrap(); 
        let row_3 = lines.next().unwrap();  
        let _blank = lines.next(); 

        let row_to_bool = |row: &str| row.split("").filter(|&c| c.len() > 0).map(|c| c == "#").collect();

        let shape: Vec<Vec<_>> = vec![
            row_to_bool(row_1),
            row_to_bool(row_2),
            row_to_bool(row_3),
        ];

        let mut block_count = 0;
        for row in &shape {
            for cell in row {
                if *cell {
                    block_count += 1;
                }
            }
        }

        block_counts.push(block_count);
    }

    let regions: Vec<_> = lines
        .map(|l| {
            let mut segments = l.split(" ").filter(|l| l.len() > 0);
            let width_height = segments.next().unwrap();
            let width: u32 = width_height[0..2].parse().unwrap();
            let height: u32 = width_height[3..5].parse().unwrap();
            let shape_counts: Vec<u32> = segments.map(|s| s.trim().parse().unwrap()).collect();

            Region { width, height, shape_counts }
        })
        .collect();

    let mut fit_count = 0;
    for i in 0..regions.len() {
        let r = &regions[i];
        let region_area = r.width * r.height;

        let shape_area: u32 = r.shape_counts
            .iter()
            .enumerate()
            .map(|(j, c)| c * block_counts[j])
            .sum();


        if region_area <= shape_area {
            continue;
        }

        fit_count += 1;
    }

    fit_count.to_string()
}