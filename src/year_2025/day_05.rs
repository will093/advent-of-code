use crate::utils::solver::Solver;

pub struct Day5Part1;
pub struct Day5Part2;

#[derive(Debug, Clone)]
struct Range { start: u64, end: u64 }

impl Solver for Day5Part1 {
    fn year(&self) -> &str { "2025" }
    fn day(&self) -> &str { "05" }
    fn label(&self) -> &str { "Day 5 Part 1" }
    fn solve(&self, input: &str) -> String {
        solve(input).0
    }
}

impl Solver for Day5Part2 {
    fn year(&self) -> &str { "2025" }
    fn day(&self) -> &str { "05" }
    fn label(&self) -> &str { "Day 5 Part 2" }
    fn solve(&self, input: &str) -> String {
        solve(input).1
    }
}

fn solve(input: &str) -> (String, String) {
    let mut fresh_id_ranges: Vec<Range> = vec![];

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line == "" { break; }
        let mut split_line = line.split('-');
        let start: u64 = split_line.next().unwrap().parse::<u64>().unwrap();
        let end: u64 = split_line.next().unwrap().parse::<u64>().unwrap();
        fresh_id_ranges.push(Range { start, end });
    }

    fresh_id_ranges.sort_by_key(|r| r.start);

    let mut ingredient_ids: Vec<u64> = vec![];
    while let Some(line) = lines.next() {
        ingredient_ids.push(line.parse().unwrap());
    }
    let fresh_id_count = ingredient_ids
        .iter()
        .filter(|&id| {
            for range in &fresh_id_ranges {
                if *id >= range.start && *id <= range.end {
                    return true;
                }
            }
            false
        })
        .count();

    let mut aggregate_fresh_id_ranges: Vec<Range> = vec![];
    let mut agg_range: Option<Range> = None;

    for curr in &fresh_id_ranges {
        match agg_range {
            Some(agg) => {
                if agg.end >= curr.start - 1 {
                    agg_range = Some(Range { start: agg.start, end: std::cmp::max(curr.end, agg.end) });
                } else {
                    (&mut aggregate_fresh_id_ranges).push(agg);
                    agg_range = Some(curr.clone());
                }
            },
            None => {
                agg_range = Some(curr.clone());
            }
        }
    }

    aggregate_fresh_id_ranges.push(agg_range.unwrap());

    let possible_fresh_id_count = (&aggregate_fresh_id_ranges)
        .into_iter()
        .fold(0, |acc, Range{start, end}| acc + end + 1 - start);

    (fresh_id_count.to_string(), possible_fresh_id_count.to_string())
}
