use std::fs;

#[derive(Clone)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn area(&self, j: &Coord) -> i64 {
        (1 + self.x - j.x).abs() * (1 + self.y - j.y).abs()
    }
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

    let mut max_area = 0;
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            max_area = max_area.max(coords[i].area(&coords[j]))
        } 
    }

    println!("Largest area {}", max_area);
    Ok(())
}