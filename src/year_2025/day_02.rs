use itertools::Itertools;
use crate::define_solver;
use crate::utils::parse::{AocParseExt, IntParser};
use crate::utils::math;

#[derive(Debug, Clone, Copy)]
struct Range { start: u64, end: u64 }

define_solver!(
    Day2Solver,
    "2025",
    "02",
    Vec<Range>,
    preprocess,
    part_one,
    part_two
);

fn preprocess(input: &str) -> Vec<Range> {
    let mut unsigned_parser: IntParser<u64> = input.as_unsigned_iter();
    let mut ranges: Vec<Range> = vec![];
    loop {
        match (unsigned_parser.next(), unsigned_parser.next()) {
            (Some(start), Some(end)) => ranges.push(Range { start, end }),
            (None, None) => break,
            _ => panic!("each start of range sould have a corresponding end")
        };
    }
    ranges
}

fn part_one(input: &Vec<Range>) -> String {
    let get_repeater_lengths_part_one: GetRepeaterLengths =
        Box::new(|digit_count: u64| {
            if digit_count % 2 == 0 {
                vec![digit_count/2]
            } else {
                vec![]
            }
        });
    get_invalid_ids_total(input, &get_repeater_lengths_part_one).to_string()
}

fn part_two(input: &Vec<Range>) -> String {
    let get_repeater_lengths_part_two: GetRepeaterLengths =
        Box::new(get_repeater_lengths_part_two);
    get_invalid_ids_total(input, &get_repeater_lengths_part_two).to_string()
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

fn get_invalid_ids_total(ranges: &Vec<Range>, get_repeater_lengths: &GetRepeaterLengths) -> u64 {
    ranges
        .iter()
        .fold(0u64, |acc, range| {
            acc + get_invalid_ids(range, get_repeater_lengths).iter().sum::<u64>()
        })

}

fn get_invalid_ids(Range { start, end}: &Range, get_repeater_lengths: &GetRepeaterLengths) -> Vec<u64> {
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
            let range_start = if digit_count == start_len { *start } else { 10u64.pow((digit_count - 1) as u32) };
            let range_end = if digit_count == end_len { *end } else { 10u64.pow(digit_count as u32) - 1 };
            let range = Range { start: range_start, end: range_end};
            repeater_lengths
                .iter()
                .flat_map(|&f| find_invalid_in_range(range, f))
                .collect::<Vec<u64>>()
        })
        .collect();

    invalids.sort();
    invalids.dedup();

    invalids
}

fn find_invalid_in_range(Range { start, end }: Range, repitition_length: u64) -> Vec<u64> {
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
        let res = find_invalid_in_range(Range { start: 10, end: 99 }, 1);
        assert_eq!(res, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }

    #[test]
    fn find_invalid_in_range_2_digits_start_end_respected() {
        let res = find_invalid_in_range(Range { start: 20, end: 40 }, 1);
        assert_eq!(res, vec![22, 33]);
    }

    #[test]
    fn find_invalid_in_range_2_digits_start_end_exact_inclusive() {
        let res = find_invalid_in_range(Range { start: 44, end: 55 }, 1);
        assert_eq!(res, vec![44, 55]);
    }

    #[test]
    fn find_invalid_in_range_6_digits() {
        let res = find_invalid_in_range(Range { start: 10101111, end: 20000000 }, 2);
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