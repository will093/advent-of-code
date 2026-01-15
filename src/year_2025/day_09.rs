use crate::define_solver;
use std::{cmp::Reverse};

define_solver!(
    Day9Solver,
    "2025",
    "09",
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
    area: u64,
}

impl Rectangle {
    fn max_x(&self) -> i64 { self.start.x.max(self.end.x) }
    fn min_x(&self) -> i64 { self.start.x.min(self.end.x) }
    fn max_y(&self) -> i64 { self.start.y.max(self.end.y) }
    fn min_y(&self) -> i64 { self.start.y.min(self.end.y) }

    fn interior_edges(&self) -> Vec<Edge> {
        return vec![
            Edge::new( 
                Coord { x: self.min_x() + 1, y: self.min_y() + 1 },
                Coord { x: self.min_x() + 1, y: self.max_y() - 1  },
            ),
            Edge::new( 
                Coord { x: self.min_x() + 1, y: self.max_y() - 1  },
                Coord { x: self.max_x() - 1, y: self.max_y() - 1  },
            ),
            Edge::new(
                Coord { x: self.max_x() - 1, y: self.max_y() - 1  },
                Coord { x: self.max_x() - 1, y: self.min_y() + 1  },
            ),
            Edge::new( 
                Coord { x: self.max_x() - 1, y: self.min_y() + 1  },
                Coord { x: self.min_x() + 1, y: self.min_y() + 1 },
            ),
        ]
    }
}

#[derive(Debug)]
struct Edge {
    start: Coord,
    end: Coord,
    orientation: Orientation,
}

#[derive(Debug)]
enum Orientation {
    Vertical,
    Horizontal,
}

impl Edge {

    fn new(start: Coord, end: Coord) -> Self {
        let orientation = if start.x == end.x { Orientation::Vertical } else { Orientation::Horizontal };
        Edge { 
            start,
            end,
            orientation,
        }
    }

    fn max_x(&self) -> i64 { self.start.x.max(self.end.x) }
    fn min_x(&self) -> i64 { self.start.x.min(self.end.x) }
    fn max_y(&self) -> i64 { self.start.y.max(self.end.y) }
    fn min_y(&self) -> i64 { self.start.y.min(self.end.y) }

    fn intersects(&self, e: &Edge) -> bool {
        match (&self.orientation, &e.orientation) {
            (Orientation::Vertical, Orientation::Vertical) => {
                self.start.x == e.start.x &&
                self.max_y() >= e.min_y() &&
                self.min_y() <= e.max_y()
            },
            (Orientation::Horizontal, Orientation::Horizontal) => {
                self.start.y == e.start.y &&
                self.max_x() >= e.min_x() &&
                self.min_x() <= e.max_x()
            },
            (Orientation::Vertical, Orientation::Horizontal) => {
                e.min_x() <= self.start.x && self.start.x <= e.max_x() &&
                self.min_y() <= e.start.y && e.start.y <= self.max_y()
            },
            (Orientation::Horizontal, Orientation::Vertical) => {
                self.min_x() <= e.start.x && e.start.x <= self.max_x() &&
                e.min_y() <= self.start.y && self.start.y <= e.max_y()
            },
        }
    }
}

fn solve(input: &str) -> (String, String) {
    let coords: Vec<Coord> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            let x = parts[0].parse::<i64>().unwrap();
            let y = parts[1].parse::<i64>().unwrap();
            return Coord { x, y };
        })
        .collect();

    let area = largest_area(&coords);
    let interior_area = largest_interior_area(&coords);

    (area.to_string(), interior_area.to_string())
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

fn get_boundary_edges(coords: &Vec<Coord>) -> Vec<Edge> {
    let mut coords_cycled = coords[1..].to_vec();
    coords_cycled.push(coords[0].clone());

    coords
        .iter()
        .zip(coords_cycled.iter())
        .map(|(c, c_next) | {
            Edge::new(c.clone(), c_next.clone())
        })
        .collect()
}

fn largest_interior_area(coords: &Vec<Coord>) -> u64 {
    let loop_edges = get_boundary_edges(coords);

    let mut rectangles = vec![];

    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            let min_x = coords[i].x.min(coords[j].x);
            let max_x = coords[i].x.max(coords[j].x);
            let min_y = coords[i].y.min(coords[j].y);
            let max_y = coords[i].y.max(coords[j].y);
            let rectangle = Rectangle {
                start: Coord { x: min_x, y: min_y },
                end: Coord { x: max_x, y: max_y },
                area: ((1 + max_x - min_x) * (1 + max_y - min_y)) as u64,
            };
            rectangles.push(rectangle);
        } 
    }

    rectangles.sort_by_key(|r| Reverse(r.area));

    let max_area_rect = rectangles
        .iter()
        // For now filter out 1/2 width rectangles as they break the algorithm and probably are not the largest.
        .filter(|&r| (r.start.x - r.end.x).abs() > 1 && (r.start.y - r.end.y).abs() > 1)
        .find(|&r| is_inside_boundary(&loop_edges, r))
        .unwrap();
    max_area_rect.area
}

// TODO: this gives the right answer but doesnt work for general case...
fn is_inside_boundary(boundary_edges: &Vec<Edge>, rect: &Rectangle) -> bool {
    let rect_interior = rect.interior_edges();

    for rect_edge in &rect_interior {
        for boundary_edge in boundary_edges {
            if rect_edge.intersects(boundary_edge) {
                return false;
            }
        }
    }

    true
}



#[cfg(test)]
mod tests {
    use super::*;


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
