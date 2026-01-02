use crate::utils::solver::Solver;

pub struct Day2Part1;
pub struct Day2Part2;

impl Solver for Day2Part1 {
    fn year(&self) -> &str { "2025" }
    fn day(&self) -> &str { "02" }
    fn label(&self) -> &str { "Day 2 Part 1" }
    fn solve(&self, input: &str) -> String {
        let get_factors: GetFactors =
            Box::new(|length: u64, n: u64| if length % n == 0 { vec![n, length/n] } else { vec![] });
        solve(input, &get_factors)
    }
}

impl Solver for Day2Part2 {
    fn year(&self) -> &str { "2025" }
    fn day(&self) -> &str { "02" }
    fn label(&self) -> &str { "Day 2 Part 2" }
    fn solve(&self, input: &str) -> String {
        let get_factors: GetFactors =
            Box::new(|l: u64, n: u64| if l % 2 == 0 { vec![l/2] } else { vec![] });
        solve(input, &get_factors)
    }
}

type GetFactors = Box<dyn Fn(u64,u64) -> Vec<u64>>;

fn solve(input: &str, get_factors: &GetFactors) -> String {
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

    let total = get_invalid_ids_total(ranges, get_factors);
    total.to_string()
}

fn get_invalid_ids_total(ranges: Vec<(u64, u64)>, get_factors: &GetFactors) -> u64 {
    ranges
        .iter()
        .fold(0u64, |acc, range| {
            acc + get_invalid_ids(*range, get_factors).iter().sum::<u64>()
        })

}

fn get_invalid_ids((start, end): (u64, u64), get_factors: &GetFactors) -> Vec<u64> {
    let start_len: u64 = start.to_string().len() as u64;
    let end_len: u64 = end.to_string().len() as u64;

    let mut invalids: Vec<u64> = (start_len..end_len + 1)
        .flat_map(|length| {
            let mut factors: Vec<u64> = (1..length.isqrt() + 1 as u64)
                .flat_map(|n| {
                    get_factors(length, n)
                })
                .filter(|x| x != &length)
                .collect::<Vec<u64>>();

            factors.sort();
            factors.dedup();

            let length_start = if length == start_len { start } else { 10u64.pow((length - 1) as u32) };
            let length_end = if length == end_len { end } else { 10u64.pow(length as u32) - 1 };

            let invalid = factors
                .iter()
                .flat_map(|f| {
                    find_invalid_in_range(length_start, length_end, *f)
                })
                .collect::<Vec<u64>>();
            return invalid
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
        .map(|i| (i.to_string().repeat(repititions as usize)).parse().unwrap())
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
}