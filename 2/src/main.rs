use std::fs;

fn main() {
    let res = fs::read_to_string("./input.txt");
    match res {
        Ok(input) => {
            let ranges: Vec<(u64, u64)> = input
                .lines()
                .next()
                .unwrap()
                .split(",")
                .map(|x| {
                    let mut parts = x.split('-');
                    (
                        parts.next().unwrap().parse().unwrap(), 
                        parts.next().unwrap().parse().unwrap()
                    )
                })
                .collect();

            let total = get_invalid_ids_total(ranges);
            println!("{}", total)
        },
        Err(e) => {
            panic!("{}", e);
        }
    }
}

fn get_invalid_ids_total(ranges: Vec<(u64, u64)>) -> u64 {
    ranges
        .iter()
        .fold(0u64, |acc, range| {
            acc + get_invalid_ids(*range).iter().sum::<u64>()
        })

}

fn get_invalid_ids((start, end): (u64, u64)) -> Vec<u64> {

    let start_len: u32 = start.to_string().len().try_into().unwrap();
    let end_len: u32 = end.to_string().len().try_into().unwrap();

    // Note that this works for the solution at hand but doesnt account fro | start_len - end_len | > 1
    match (start_len % 2, end_len % 2) {
        (0, 0) => return find_invalid_in_range(start, end),
        (1, 0) => return find_invalid_in_range(10u64.pow(start_len), end),
        (0, 1) => return find_invalid_in_range(start, 10u64.pow(end_len) - 1),
        (_, _) => return vec![],
    }
    
}

fn find_invalid_in_range(start: u64, end: u64) -> Vec<u64> {
    let repeat_length: u32 = (start.to_string().len() / 2).try_into().unwrap();


    (0..(10u64.pow(repeat_length)))
        .map(|i| (i.to_string() + &i.to_string()).parse().unwrap())
        .filter(|&n| n >= start && n <= end)
        .collect()
}

// Part 2

// find factors of length of start/end and all values in between
// eg. length 8 1,2,4
// check for repetitions of numbers of equal length to those factors

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_invalid_in_range_2_digits() {
        let res = find_invalid_in_range(10, 99);
        assert_eq!(res, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }

    #[test]
    fn find_invalid_in_range_2_digits_start_end_respected() {
        let res = find_invalid_in_range(20, 40);
        assert_eq!(res, vec![22, 33]);
    }


    #[test]
    fn find_invalid_in_range_2_digits_start_end_exact_inclusive() {
        let res = find_invalid_in_range(44, 55);
        assert_eq!(res, vec![44, 55]);
    }


    #[test]
    fn find_invalid_in_range_6_digits() {
        let res = find_invalid_in_range(10011000, 20000000);
        assert_eq!(res, vec![44, 55]);
    }

}