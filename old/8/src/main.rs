use std::fs;
use std::collections::HashSet;

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

fn main() -> Result<(), std::io::Error> {
   let junctions: Vec<Junction> = fs::read_to_string("./input.txt")?
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let parts: Vec<&str> = line.split(',').collect();
            let x = parts[0].parse::<i64>().unwrap();
            let y = parts[1].parse::<i64>().unwrap();
            let z = parts[2].parse::<i64>().unwrap();
            return Junction { x, y, z};
        })
        .collect();

    let mut distances: Vec<(usize, usize, i64)> = vec![];
    for i in 0..junctions.len() {
        for j in i+1..junctions.len() {
            distances.push((i,j, junctions[i].distance(&junctions[j])))
        } 
    }
    distances.sort_by_key(|t| t.2);

    let mut shortest_distances_iter = distances.iter();

    let mut circuit_sets: Vec<HashSet<i64>> = junctions.clone().into_iter().enumerate().map(|(i,_)| {
        let mut s = HashSet::new();
        s.insert(i as i64);
        s
    }).collect();

    loop {
        let dist = shortest_distances_iter.next().unwrap();

        let i1 = circuit_sets.iter().position(|set| set.contains(&(dist.0 as i64))).unwrap();
        let i2 = circuit_sets.iter().position(|set| set.contains(&(dist.1 as i64))).unwrap();

        if circuit_sets.len() == 2 && i1 != i2 {
            println!("Final Junctions X multiplied: {}", junctions[dist.0].x * junctions[dist.1].x);
            break;
        }

        if i1 == i2 {
            continue;
        }
        let mut next_set = HashSet::new();

        next_set.extend(circuit_sets[i1].clone());
        next_set.extend(circuit_sets[i2].clone());
        circuit_sets.push(next_set);
        circuit_sets.remove(i1.max(i2));
        circuit_sets.remove(i1.min(i2));

    }

    Ok(())
}