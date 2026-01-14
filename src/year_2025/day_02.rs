use itertools::Itertools;

use crate::define_solver;
use crate::utils::parse::{AocParseExt, IntParser};
use crate::utils::math;

define_solver!(
    Day2Solver,
    "2025",
    "02",
    String,
    preprocess,
    part_one,
    part_two
);

fn preprocess(input: &str) -> String {
    String::from(input)
}

fn part_one(input: &str) -> String {
    let get_repeater_lengths_part_one: GetRepeaterLengths =
        Box::new(|digit_count: u64| {
            if digit_count % 2 == 0 {
                vec![digit_count/2]
            } else {
                vec![]
            }
        });
    solve(input, &get_repeater_lengths_part_one)
}

fn part_two(input: &str) -> String {
    let get_repeater_lengths_part_two: GetRepeaterLengths =
        Box::new(get_repeater_lengths_part_two);
    solve(input, &get_repeater_lengths_part_two)
}

fn get_repeater_lengths_part_two(digit_count: u64) -> Vec<u64> {
    match digit_count {
        0 => vec![],
        1 => vec![],
        digit_count => {
            let vals: Vec<_> = (2..(digit_count/2)+1)
                .filter(|&n| digit_count % n == 0 && math::is_prime(n))
                .map(|n| digit_count / n)
                .sorted()
                .dedup()
                .collect();
            if vals.is_empty() { vec![1] } else { vals }
        }
    }
}

type GetRepeaterLengths = Box<dyn Fn(u64) -> Vec<u64>>;

fn solve(input: &str, get_repeater_lengths: &GetRepeaterLengths) -> String {
    let mut unsigned_parser: IntParser<u64> = input.as_unsigned_iter();
    let mut ranges: Vec<(u64, u64)> = vec![];
    loop {

        let start = match unsigned_parser.next() {
            Some(val) => val,
            None => break,
        };
        let end = match unsigned_parser.next() {
            Some(val) => val,
            None => panic!("each start of range sould have a corresponding end"),
        };
        ranges.push((start, end))
    }

    get_invalid_ids_total(ranges, get_repeater_lengths).to_string()
}

fn get_invalid_ids_total(ranges: Vec<(u64, u64)>, get_repeater_lengths: &GetRepeaterLengths) -> u64 {
    ranges
        .iter()
        .fold(0u64, |acc, range| {
            acc + get_invalid_ids(*range, get_repeater_lengths).iter().sum::<u64>()
        })

}

fn get_invalid_ids((start, end): (u64, u64), get_repeater_lengths: &GetRepeaterLengths) -> Vec<u64> {
    let start_len: u64 = start.to_string().len() as u64;
    let end_len: u64 = end.to_string().len() as u64;

    // We look at each possible number of digits that an ID in this range could be
    let mut invalids: Vec<u64> = (start_len..end_len + 1)
        .flat_map(|digit_count| {
            let repeater_lengths: Vec<u64> = get_repeater_lengths(digit_count)
                .into_iter()
                .filter(|x| x != &digit_count)
                .collect::<Vec<u64>>();

            // The range in which to search for invalid IDs
            let range_start = if digit_count == start_len { start } else { 10u64.pow((digit_count - 1) as u32) };
            let range_end = if digit_count == end_len { end } else { 10u64.pow(digit_count as u32) - 1 };

            repeater_lengths
                .iter()
                .flat_map(|&f| find_invalid_in_range(range_start, range_end, f))
                .collect::<Vec<u64>>()
        })
        .collect();

    invalids.sort();
    invalids.dedup();

    invalids
}

fn find_invalid_in_range(start: u64, end: u64, repitition_length: u64) -> Vec<u64> {
    let total_length: u64 = start.to_string().len() as u64;
    let repititions = total_length / repitition_length;

    (0..(10u64.pow(repitition_length as u32)))
        .map(|i| (i.to_string().repeat(repititions as usize)).parse().expect("'i' is an unsigned int so 'i' repeated should also be an int"))
        .filter(|&n| n >= start && n <= end)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_invalid_in_range_2_digits() {
        let res = find_invalid_in_range(10, 99, 1);
        assert_eq!(res, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }

    #[test]
    fn find_invalid_in_range_2_digits_start_end_respected() {
        let res = find_invalid_in_range(20, 40, 1);
        assert_eq!(res, vec![22, 33]);
    }

    #[test]
    fn find_invalid_in_range_2_digits_start_end_exact_inclusive() {
        let res = find_invalid_in_range(44, 55, 1);
        assert_eq!(res, vec![44, 55]);
    }

    #[test]
    fn find_invalid_in_range_6_digits() {
        let res = find_invalid_in_range(10101111, 20000000, 2);
        assert_eq!(res, vec![11111111, 12121212, 13131313, 14141414, 15151515, 16161616, 17171717, 18181818, 19191919]);
    }

    #[test]
    fn get_possible_lengths_part_two_test() {
        assert_eq!(get_repeater_lengths_part_two(1), vec![]);
        assert_eq!(get_repeater_lengths_part_two(2), vec![1]);
        assert_eq!(get_repeater_lengths_part_two(3), vec![1]);
        assert_eq!(get_repeater_lengths_part_two(4), vec![2]);
        assert_eq!(get_repeater_lengths_part_two(5), vec![1]);
        assert_eq!(get_repeater_lengths_part_two(6), vec![2,3]);
        assert_eq!(get_repeater_lengths_part_two(7), vec![1]);
        assert_eq!(get_repeater_lengths_part_two(8), vec![4]);
        assert_eq!(get_repeater_lengths_part_two(9), vec![3]);
        assert_eq!(get_repeater_lengths_part_two(10), vec![2, 5]);
        assert_eq!(get_repeater_lengths_part_two(11), vec![1]);
        assert_eq!(get_repeater_lengths_part_two(12), vec![4,6]);
        assert_eq!(get_repeater_lengths_part_two(13), vec![1]);
    }
}