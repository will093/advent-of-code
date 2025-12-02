use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let res = fs::read_to_string("./input.txt");
    match res {
        Ok(input) => {
            let rotations = input.lines().collect();
            let zero_count = count_zero_clicks(rotations, 50);
            // let zero_count = count_zero_landings(rotations, 50);

            println!("{}", zero_count)
        },
        Err(e) => {
            panic!("{}", e);
        }
    }

    Ok(())
}


fn count_zero_landings(rotations: Vec<&str>, start: i32) -> i32 {
    let dial_size = 100;
    let mut current = start;
    let mut zero_count = 0;
    for r in rotations {
        let num: i32 = rotation_to_num(r);

        current = (current + num).rem_euclid(dial_size);

        if current == 0 {
            zero_count += 1;
        }
    }
    zero_count
}

fn count_zero_clicks(rotations: Vec<&str>, start: i32) -> i32 {
    let dial_size = 100;
    let mut current = start;
    let mut zeros_total = 0;
    for r in rotations {
        let num: i32 = rotation_to_num(r);

        let full_rotations = (num / dial_size).abs();
        zeros_total += full_rotations;

        let partial_rotation = current + (num % dial_size);  

        if current != 0 && (partial_rotation <= 0 || partial_rotation >= dial_size) {
            zeros_total += 1;
        }

        current = (current + num).rem_euclid(dial_size);
    }
    zeros_total
}

fn rotation_to_num(val: &str) -> i32 {
    let mut num: i32 = val[1..].parse().unwrap();
    if val.starts_with('L') {
        num *= -1;
    }
    num
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_zero_landings_no_rotation() {
        let rotations: Vec<&str> = vec![];
        let result: i32 = count_zero_landings(rotations, 3);
        assert_eq!(result, 0);
    }

    #[test]
    fn count_zero_landings_single_rotation() {
        let rotations: Vec<&str> = vec!["R10"];
        let result: i32 = count_zero_landings(rotations, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn count_zero_landings_multi_rotation() {
        let rotations: Vec<&str> = vec!["R3", "L5", "R2"];
        let result: i32 = count_zero_landings(rotations, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn count_zero_landings_many_zeros() {
        let rotations: Vec<&str> = vec!["R10", "L15", "R1", "L1", "L20", "R10", "R10"];
        let result: i32 = count_zero_landings(rotations,5);
        assert_eq!(result, 3);
    }

    #[test]
    fn count_zero_landings_large_nums() {
        let rotations: Vec<&str> = vec!["R150", "R145"];
        let result: i32 = count_zero_landings(rotations, 5);
        assert_eq!(result, 1);
    }


    #[test]
    fn count_zero_landings_large_nums_multi() {
        let rotations: Vec<&str> = vec!["L100", "L1", "L2", "R99", "L199", "L3"];
        let result: i32 = count_zero_landings(rotations, 3);
        assert_eq!(result, 2);
    }
    
    #[test]
    fn count_zero_clicks_no_rotation() {
        let rotations: Vec<&str> = vec![];
        let result: i32 = count_zero_clicks(rotations, 3);
        assert_eq!(result, 0);
    }

    #[test]
    fn count_zero_clicks_right_rotation_no_clicks() {
        let rotations: Vec<&str> = vec!["R10"];
        let result: i32 = count_zero_clicks(rotations, 3);
        assert_eq!(result, 0);
    }

    #[test]
    fn count_zero_clicks_right_rotation_one_click() {
        let rotations: Vec<&str> = vec!["R98"];
        let result: i32 = count_zero_clicks(rotations, 3);
        assert_eq!(result, 1);
    }

    #[test]
    fn count_zero_clicks_right_rotation_multi_click() {
        let rotations: Vec<&str> = vec!["R900"];
        let result: i32 = count_zero_clicks(rotations, 1);
        assert_eq!(result, 9);
    }

        #[test]
    fn count_zero_clicks_left_rotation_no_clicks() {
        let rotations: Vec<&str> = vec!["L1"];
        let result: i32 = count_zero_clicks(rotations, 3);
        assert_eq!(result, 0);
    }

    #[test]
    fn count_zero_clicks_left_rotation_one_click() {
        let rotations: Vec<&str> = vec!["L10"];
        let result: i32 = count_zero_clicks(rotations, 3);
        assert_eq!(result, 1);
    }

    #[test]
    fn count_zero_clicks_left_rotation_multi_click() {
        let rotations: Vec<&str> = vec!["L900"];
        let result: i32 = count_zero_clicks(rotations, 1);
        assert_eq!(result, 9);
    }

    #[test]
    fn count_zero_clicks_multi_rotations() {
        let rotations: Vec<&str> = vec!["L100", "L100", "R200"];
        let result: i32 = count_zero_clicks(rotations, 1);
        assert_eq!(result, 4);
    }

    #[test]
    fn count_zero_clicks_left_zero_landing() {
        let rotations: Vec<&str> = vec!["L101", "L100", "L200"];
        let result: i32 = count_zero_clicks(rotations, 1);
        assert_eq!(result, 5);
    }

        #[test]
    fn count_zero_clicks_right_zero_landing() {
        let rotations: Vec<&str> = vec!["R100"];
        let result: i32 = count_zero_clicks(rotations, 0);
        assert_eq!(result, 1);
    }

    #[test]
    fn count_zero_clicks_close_to_zero() {
        let rotations: Vec<&str> = vec!["R98", "L98"];
        let result: i32 = count_zero_clicks(rotations, 1);
        assert_eq!(result, 0);
    }


    #[test]
    fn count_zero_clicks_left_large_nums() {
        let rotations: Vec<&str> = vec!["L499", "L500"];
        let result: i32 = count_zero_clicks(rotations, 1);
        assert_eq!(result, 10);
    }

    #[test]
    fn count_zero_clicks_example() {
        let rotations: Vec<&str> = vec!["L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"];
        let result: i32 = count_zero_clicks(rotations, 50);
        assert_eq!(result, 6);
    }
    

}