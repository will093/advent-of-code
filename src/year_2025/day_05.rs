use crate::utils::parse::{AocParseExt, IntParser};
use crate::define_solver;

#[derive(Debug, Clone)]
struct Range { start: u64, end: u64 }

struct Ingredients {
    sorted_fresh_id_ranges: Vec<Range>,
    ingredient_ids: Vec<u64>
}

define_solver!(
    Day5Solver,
    "2025",
    "05",
    Ingredients,
    preprocess,
    part_one,
    part_two
);

fn preprocess(input: &str) -> Ingredients {
    let [input_ranges, input_ids]: [&str; 2] = input
        .split("\n\n")
        .take(2)
        .collect::<Vec<_>>()
        .try_into()
        .expect("Expected exactly 2 sections");

    let mut ranges_parser: IntParser<u64> = input_ranges.as_unsigned_iter();
    let mut ranges: Vec<Range> = vec![];
    loop {
        match (ranges_parser.next(), ranges_parser.next()) {
            (Some(start), Some(end)) => ranges.push(Range { start, end }),
            (None, None) => break,
            _ => panic!("Each start of range sould have a corresponding end"),
        };
    }
    ranges.sort_by_key(|r| r.start);

    let ingredient_ids: Vec<u64> = input_ids.as_unsigned_iter().collect();

    Ingredients { 
        sorted_fresh_id_ranges: ranges, 
        ingredient_ids 
    }
}

fn part_one(ingredients: &Ingredients) -> String {
    get_fresh_ingredient_count(ingredients).to_string()
}

fn part_two(ingredients: &Ingredients) -> String {
    get_fresh_range_total(ingredients).to_string()
}

fn get_fresh_ingredient_count(Ingredients{ sorted_fresh_id_ranges, ingredient_ids }: &Ingredients) -> usize {
    ingredient_ids
        .iter()
        .filter(|&id| {
            for range in sorted_fresh_id_ranges {
                if *id >= range.start && *id <= range.end {
                    return true;
                }
            }
            false
        })
        .count()
}

fn get_fresh_range_total(Ingredients{ sorted_fresh_id_ranges, ingredient_ids: _ }: &Ingredients) -> u64 {
    let mut aggregate_fresh_id_ranges: Vec<Range> = vec![];
    let mut agg_range: Option<Range> = None;

    // Aggregate any groups of multiple ranges which overlap into a single range
    for curr in sorted_fresh_id_ranges {
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
    aggregate_fresh_id_ranges.push(agg_range.expect("agg range always not None after aggregation loop"));

    let possible_fresh_id_count = (&aggregate_fresh_id_ranges)
        .into_iter()
        .fold(0, |acc, Range{start, end}| acc + end + 1 - start);

    possible_fresh_id_count
}
