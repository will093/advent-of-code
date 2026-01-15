use crate::define_solver;
use std::collections::{HashSet};
use crate::utils::parse::{AocParseExt, IntParser};

#[derive(Clone)]
struct Junction {
    x: i64,
    y: i64,
    z: i64,
}

impl Junction {
    fn distance(&self, j: &Junction) -> i64 {
        (
            (self.x - j.x).pow(2) + 
            (self.y - j.y).pow(2) + 
            (self.z - j.z).pow(2)
        ).isqrt()
    }
}

struct JunctionPairDistance {
    j1_index: usize,
    j2_index: usize,
    distance: i64,
}

define_solver!(
    Day8Solver,
    "2025",
    "08",
    (String, String),
    preprocess,
    part_one,
    part_two
);

fn preprocess(input: &str) -> (String, String) {
    solve(input)
}

fn part_one((one, _): &(String, String)) -> String {
    String::from(one)
}

fn part_two((_, two): &(String, String)) -> String {
    String::from(two)
}

fn solve(input: &str) -> (String, String) {
    let mut parser: IntParser<i64> = input.as_signed_iter();

    let mut junctions: Vec<Junction> = vec![];
    while let (Some(x), Some(y), Some(z)) = (parser.next(),parser.next(), parser.next()) {
        junctions.push(Junction { x, y, z})
    }

    let mut junction_distances: Vec<JunctionPairDistance> = vec![];
        
    for i in 0..junctions.len() {
        for j in i+1..junctions.len() {
            junction_distances.push(JunctionPairDistance { j1_index: i, j2_index: j, distance: junctions[i].distance(&junctions[j]) });
        } 
    }
    junction_distances.sort_by_key(|t| t.distance);

    let mut shortest_distances_iter = junction_distances.iter();

    // We create a set for every junction.
    let mut circuit_sets: Vec<HashSet<i64>> = junctions.clone().into_iter().enumerate().map(|(i,_)| {
        let mut s = HashSet::new();
        s.insert(i as i64);
        s
    }).collect();

    let mut count = 0;
    let mut part1_result = 0;

    // Iteratively look at the junctions with the shortest distance between them and join their sets.
    loop {
        let distance = shortest_distances_iter.next()
            .expect("expected not to reach end of shortest_distances_iter");

        let i1 = circuit_sets.iter().position(|set| set.contains(&(distance.j1_index as i64))).unwrap();
        let i2 = circuit_sets.iter().position(|set| set.contains(&(distance.j2_index as i64))).unwrap();

        if count == 1000 {
            circuit_sets.sort_by_key(|s| usize::MAX - s.len());
            part1_result = circuit_sets[0].len() * circuit_sets[1].len() * circuit_sets[2].len();
        }
        count += 1;

        if i1 == i2 {
            continue;
        }

        if circuit_sets.len() == 2 {
            let part2_result = junctions[distance.j1_index].x * junctions[distance.j2_index].x;
            return (part1_result.to_string(), part2_result.to_string());
        }
        let mut next_set = HashSet::new();

        next_set.extend(circuit_sets.remove(i1.max(i2)));
        next_set.extend(circuit_sets.remove(i1.min(i2)));
        circuit_sets.push(next_set);
    }
}