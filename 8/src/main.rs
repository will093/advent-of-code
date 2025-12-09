use std::fs;
use std::collections::{HashSet, HashMap, BTreeSet};
use std::hash::Hash;
use std::rc::Rc;
use std::cell::RefCell;

// struct Junction {
//     x: i64,
//     y: i64,
//     z: i64,
// }

// impl Junction {
//     fn distance(&self, j: &Junction) -> i64 {
//         ((self.x - j.x).pow(2) + (self.y - j.y).pow(2) + (self.z - j.z).pow(2)).isqrt()
//     }
// }

// fn main() -> Result<(), std::io::Error> {
//    let junctions: Vec<Junction> = fs::read_to_string("./input.txt")?
//         .lines()
//         .map(|line| {
//             let parts: Vec<&str> = line.split(',').collect();
//             let x = parts[0].parse::<i64>().unwrap();
//             let y = parts[1].parse::<i64>().unwrap();
//             let z = parts[2].parse::<i64>().unwrap();
//             return Junction { x, y, z }
//         })
//         .collect();

//     let mut distances: Vec<(usize, usize, i64)> = vec![];
//     for i in 0..junctions.len() {
//         for j in i+1..junctions.len() {
//             distances.push((i,j, junctions[i].distance(&junctions[j])))
//         } 
//     }
//     distances.sort_by_key(|t| t.2);

//     let closest_pairs: Vec<(usize, usize, i64)> = distances.into_iter().take(1000).collect();

//     let mut circuits: HashMap<usize, Rc<RefCell<HashSet<usize>>>> = HashMap::new();
//     let mut all_sets: Vec<Rc<RefCell<HashSet<usize>>>> = Vec::new();

//     for (j1, j2, _) in closest_pairs {
//         let set = circuits.get(&j1)
//             .or_else(|| circuits.get(&j2))
//             .cloned()
//             .unwrap_or_else(|| {
//                 let new_set = Rc::new(RefCell::new(HashSet::new()));
//                 all_sets.push(new_set.clone());
//                 new_set
//             });

//         circuits.insert(j1, set.clone());
//         circuits.insert(j2, set.clone());

//         set.borrow_mut().extend([j1, j2]);
//     }

//     println!("circuits: {:?}, {}", all_sets, all_sets.len());

//     all_sets
//         .sort_by_key(|rc_set| -(rc_set.borrow().len() as isize));
    
//     let largest_3: Vec<_> = all_sets.iter().take(3).map(|rc| rc.borrow().clone()).collect();

//     let multiplied = largest_3[0].len() * largest_3[1].len() * largest_3[2].len();

//     println!("Mult {:?}, {}", largest_3, multiplied);


//     Ok(())
// }

// struct Junction {
//     id: i64,
//     x: i64,
//     y: i64,
//     z: i64,

//     circuit: Rc<RefCell<HashSet<i64>>>,


// }

// impl Junction {
//     fn distance(&self, j: &Junction) -> i64 {
//         ((self.x - j.x).pow(2) + (self.y - j.y).pow(2) + (self.z - j.z).pow(2)).isqrt()
//     }

//     pub fn new(id: i64, x: i64,y: i64,z: i64) -> Self {
//         let circuit = Rc::new(RefCell::new(HashSet::new()));
//         circuit.borrow_mut().insert(id);
//         Self { id, x, y, z, circuit }
//     }
// }

// fn main() -> Result<(), std::io::Error> {
//    let mut junctions: Vec<Junction> = fs::read_to_string("./input.txt")?
//         .lines()
//         .enumerate()
//         .map(|(i, line)| {
//             let parts: Vec<&str> = line.split(',').collect();
//             let x = parts[0].parse::<i64>().unwrap();
//             let y = parts[1].parse::<i64>().unwrap();
//             let z = parts[2].parse::<i64>().unwrap();
//             return Junction::new(i as i64,x,y,z);
//         })
//         .collect();



//     let mut distances: Vec<(usize, usize, i64)> = vec![];
//     for i in 0..junctions.len() {
//         for j in i+1..junctions.len() {
//             distances.push((i,j, junctions[i].distance(&junctions[j])))
//         } 
//     }
//     distances.sort_by_key(|t| t.2);

//     let mut distance_iter = distances.iter();

//     let mut connection_cables = 1000;
//     while connection_cables > 0 {
//         let d = distance_iter.next().unwrap();
//         let j1 = &junctions[d.0];
//         let j2 = &junctions[d.1];

//         let intersects = j1.circuit.borrow().is_disjoint(&j2.circuit.borrow());
//         if !intersects {
//             connection_cables -= 1;
//         }

//         j1.circuit.borrow_mut().extend(j2.circuit.borrow().clone());
//         j2.circuit.replace(j1.circuit.borrow().clone());
//     }

//     let mut circuits: Vec<Rc<RefCell<HashSet<i64>>>> = junctions
//         .iter()
//         .map(|j| j.circuit.clone())
//         .collect();


//     let set_clones: Vec<BTreeSet<i64>> = circuits
//         .iter()
//         .map(|rc| rc.borrow().iter().cloned().collect::<BTreeSet<_>>())
//         .collect();

//     let unique_sets: HashSet<BTreeSet<i64>> = set_clones.into_iter().collect();

//     let mut unique_sets_vec: Vec<BTreeSet<i64>> = unique_sets.into_iter().collect();

//     unique_sets_vec.sort_by_key(|rc_set| -(rc_set.len() as isize));

//     let largest_3: Vec<_> = unique_sets_vec.iter().take(3).collect();

//     let multiplied = largest_3[0].len() * largest_3[1].len() * largest_3[2].len();


//     println!("Mult {:?}, {}", largest_3, multiplied);


//     Ok(())
// }








#[derive(Clone)]
struct Junction {
    x: i64,
    y: i64,
    z: i64,


}

impl Junction {
    fn distance(&self, j: &Junction) -> i64 {
        ((self.x - j.x).pow(2) + (self.y - j.y).pow(2) + (self.z - j.z).pow(2)).isqrt()
    }
}

fn main() -> Result<(), std::io::Error> {
   let mut junctions: Vec<Junction> = fs::read_to_string("./input.txt")?
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

    let mut distance_iter = distances.iter();

    let mut sets: Vec<HashSet<i64>> = junctions.clone().into_iter().enumerate().map(|(i,_)| {
        let mut s = HashSet::new();
        s.insert(i as i64);
        s
    }).collect();

    loop {
        let d = distance_iter.next().unwrap();

        let j1_i = sets.iter().position(|set| set.contains(&(d.0 as i64))).unwrap();
        let j2_i = sets.iter().position(|set| set.contains(&(d.1 as i64))).unwrap();

        if sets.len() == 2 && j1_i != j2_i {
            println!("{}", junctions[d.0].x * junctions[d.1].x);
            break;
        }

        if j1_i == j2_i {
            continue;
        }
        let mut next_set = HashSet::new();

        next_set.extend(sets[j1_i].clone());
        next_set.extend(sets[j2_i].clone());
        sets.push(next_set);
        sets.remove(j1_i.max(j2_i));
        sets.remove(j1_i.min(j2_i));

    }

    Ok(())
}