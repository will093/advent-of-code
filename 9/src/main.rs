use std::{fs};
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn area(&self, j: &Coord) -> i64 {
        (1 + self.x - j.x).abs() * (1 + self.y - j.y).abs()
    }
}

struct Rectangle {
    start: Coord,
    end: Coord,
}

fn main() -> Result<(), std::io::Error> {
    let coords: Vec<Coord> = fs::read_to_string("./input.txt")?
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            let x = parts[0].parse::<i64>().unwrap();
            let y = parts[1].parse::<i64>().unwrap();
            return Coord { x, y };
        })
        .collect();

    let area = largest_area(&coords);
    println!("Largest area: {}", area);

    let interior_area = largest_interior_area(&coords);
    println!("Largest  interior area: {}", interior_area);

    Ok(())
}

fn largest_area(coords: &Vec<Coord>) -> i64 {
    let mut max_area = 0;
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            max_area = max_area.max(coords[i].area(&coords[j]))
        } 
    }
    max_area
}


fn largest_interior_area(coords: &Vec<Coord>) -> i64 {
    let loop_boundary = get_boundary_coords(coords);

    println!("Boundary calculated");

    let mut max_area = 0;

    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            let min_x = coords[i].x.min(coords[j].x);
            let max_x = coords[i].x.max(coords[j].x);
            let min_y = coords[i].y.min(coords[j].y);
            let max_y = coords[i].y.max(coords[j].y);
            let rectangle_vertices= vec![
                Coord { x: min_x + 1, y: min_y + 1 },
                Coord { x: min_x + 1, y: max_y - 1 },
                Coord { x: max_x - 1, y: max_y - 1 },
                Coord { x: max_x - 1, y: min_y + 1 },
            ];
            if is_inside_boundary(&loop_boundary, &rectangle_vertices) {
                max_area = max_area.max(coords[i].area(&coords[j]))
            }
        } 
    }
    max_area
}

fn is_inside_boundary(outer_boundary: &Vec<Coord>, polygon_vertices: &Vec<Coord>) -> bool {

    let mut polygon_vertices_cycled = polygon_vertices[1..].to_vec();
    polygon_vertices_cycled.push(polygon_vertices[0].clone());
    
    let is_inside: Vec<bool> = polygon_vertices
        .iter()
        .zip(polygon_vertices_cycled.iter())
        .map(|(c1, c2)| {
            match (c1, c2) {
                (c1, c2) if c1.x == c2.x => {
                    let max_y = c1.y.max(c2.y);
                    let min_y = c1.y.min(c2.y);
                    let mut intersection_points_edge: Vec<_> = outer_boundary
                        .iter()
                        .filter(|c| c.x == c1.x && c1.y >= min_y && c1.y <= max_y)
                        .collect();
                    intersection_points_edge.sort_by_key(|c| c.y);

                    let mut prev = 0;
                    for c in intersection_points_edge {
                        if prev != 0 && c.y > prev + 1 {
                            return false;
                        }
                        prev = c.y;
                    }

                    // let intersection_points_ray: Vec<_> = outer_boundary
                    //     .iter()
                    //     .filter(|c| c.x == c1.x && c1.y > max_y)
                    //     .collect();
                    
                    // if intersection_points_ray.len() % 2 == 0 {
                    //     return false;
                    // }
                    true
                },
                (c1, c2) if c1.y == c2.y => {
                    let max_x = c1.x.max(c2.x);
                    let min_x = c1.x.min(c2.x);
                    let mut intersection_points_edge: Vec<_> = outer_boundary
                        .iter()
                        .filter(|c| c.y == c1.y && c1.x >= min_x && c1.x <= max_x)
                        .collect();
                    intersection_points_edge.sort_by_key(|c| c.x);

                    let mut prev = min_x;
                    for c in intersection_points_edge {
                        if c.x > prev + 1 {
                            return false;
                        }
                        prev = c.x;
                    }
                    if prev != min_x && prev != max_x {
                        return false;
                    }

                    // let intersection_points_ray: Vec<_> = outer_boundary
                    //     .iter()
                    //     .filter(|c| c.x == c1.x && c1.y > max_x)
                    //     .collect();
                    
                    // if intersection_points_ray.len() % 2 == 0 {
                    //     return false;
                    // }
                    true
                },
                _ => { 
                    panic!("Expected boundary point to share x or y coord with adjacent coords") 
                },
            }
        })
        .collect();

    is_inside.iter().all(|v| !!v)
}

fn get_boundary_coords(coords: &Vec<Coord>) -> Vec<Coord> {

    let mut coords_cycled = coords[1..].to_vec();
    coords_cycled.push(coords[0].clone());

    let boundary: Vec<_> = coords
        .iter()
        .zip(coords_cycled.iter())
        .flat_map(|(c1, c2)| {
            match (c1, c2) {
                (c1, c2) if c1.x == c2.x => {
                    let max_y = c1.y.max(c2.y);
                    let min_y = c1.y.min(c2.y);

                    let edge_coords: Vec<_> = (min_y..=max_y).map(|y| Coord { x: c1.x, y }).collect();
                    edge_coords
                },
                (c1, c2) if c1.y == c2.y => {
                    let max_x = c1.x.max(c2.x);
                    let min_x = c1.x.min(c2.x);

                    let edge_coords: Vec<_> = (min_x..=max_x).map(|x| Coord { x, y: c1.y }).collect();
                    edge_coords
                },
                _ => { 
                    panic!("Expected boundary point to share x or y coord with adjacent coords") 
                },
            }
        })
        .unique()
        
        .collect();

    boundary
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_boundary_coords_adjacent_share_x_or_y() {
        let input_coords = vec![
            Coord { x: 10, y: 4 },
            Coord { x: 10, y: 2 },
            Coord { x: 7, y: 2 },
            Coord { x: 7, y: 4 },
        ];
        let output_coords = get_boundary_coords(&input_coords);
        
        let expected_coords = vec![
            Coord { x: 10, y: 4 },
            Coord { x: 10, y: 3 },
            Coord { x: 10, y: 2 },
            Coord { x: 9, y: 2 },
            Coord { x: 8, y: 2 },
            Coord { x: 7, y: 2 },
            Coord { x: 7, y: 3 },
            Coord { x: 7, y: 4 },
            Coord { x: 8, y: 4 },
            Coord { x: 7, y: 4 },
        ];

        assert_eq!(output_coords.len(), expected_coords.len());
        
        for c in expected_coords {
            assert_eq!(output_coords.iter().find(|&o| o == &c), Some(&c));
        }
    }


    #[test]
    fn largest_interior_area_example() {
        let boundary = vec![
            Coord { x: 7, y: 1 },
            Coord { x: 11, y: 1 },
            Coord { x: 11, y: 7 },
            Coord { x: 9, y: 7 },
            Coord { x: 9, y: 5 },
            Coord { x: 2, y: 5 },
            Coord { x: 2, y: 3 },
            Coord { x: 7, y: 3 },
        ];
        let output_area = largest_interior_area(&boundary);
    
        assert_eq!(output_area, 24);
    }
}
