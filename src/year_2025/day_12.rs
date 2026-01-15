use crate::{define_solver, utils::parse::{AocParseExt, IntParser}};

#[derive(Debug)]
struct Region {
    width: u32,
    height: u32,
    shape_counts: Vec<u32>,
}

struct PresentProblems {
    present_sizes: Vec<u32>,
    regions: Vec<Region>
}

define_solver!(
    Day12Solver,
    "2025",
    "12",
    PresentProblems,
    preprocess,
    part_one,
    part_two
);

fn preprocess(input: &str) -> PresentProblems {
    let mut lines = input.lines();

    let mut present_sizes = vec![];
    for _ in 0..6 {
        let _index = lines.next(); 
        let row_1 = lines.next().unwrap(); 
        let row_2 = lines.next().unwrap(); 
        let row_3 = lines.next().unwrap();  
        let _blank = lines.next(); 

        let row_to_bool = |row: &str| row.split("")
            .filter(|&c| c.len() > 0)
            .map(|c| c == "#")
            .collect();

        let shape: Vec<Vec<_>> = vec![
            row_to_bool(row_1),
            row_to_bool(row_2),
            row_to_bool(row_3),
        ];

        let mut present_size = 0;
        for row in &shape {
            for cell in row {
                if *cell {
                    present_size += 1;
                }
            }
        }

        present_sizes.push(present_size);
    }

    let regions: Vec<_> = lines
        .map(|l| {
            let mut line_parser: IntParser<u32> = l.as_unsigned_iter();
            let width: u32 = line_parser.next().unwrap();
            let height: u32 = line_parser.next().unwrap();
            let shape_counts: Vec<u32> = line_parser.collect();
            Region { width, height, shape_counts }
        })
        .collect();

    PresentProblems { present_sizes, regions }
}

fn part_one(input: &PresentProblems) -> String {
    solve(input).to_string()
}

fn part_two(_: &PresentProblems) -> String {
    "Merry Xmas!".to_string()
}

fn solve(PresentProblems { present_sizes, regions }: &PresentProblems)-> usize {
    regions.iter()
        .filter(|&r|
            r.width * r.height > r.shape_counts
                .iter()
                .enumerate()
                .map(|(j, c)| c * present_sizes[j])
                .sum()
        )
        .count()
}