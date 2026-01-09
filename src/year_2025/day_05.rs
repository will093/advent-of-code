use crate::define_solver;

define_solver!(
    Day5Solver,
    "2025",
    "05",
    (String, String),
    preprocess,
    part_one,
    part_two
);

fn preprocess(input: &str) -> (String, String) {
    solve(input)
}

fn part_one((one, _): &(String, String)) -> String {
    one.clone()
}

fn part_two((_, two): &(String, String)) -> String {
    two.clone()
}


#[derive(Debug, Clone)]
struct Range { start: u64, end: u64 }

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
