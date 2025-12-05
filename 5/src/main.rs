use std::fs;
use std::collections::HashSet;
use std::str::Lines;

#[derive(Debug, Clone)]
struct Range {
    start: u64,
    end: u64,
}


fn main() {
  let res = fs::read_to_string("./input.txt");
    match res {
        Ok(input) => {
            let lines = input
                .lines()
                .into_iter();

            total_fresh_ingredients(lines);
            // total_fresh_ids(lines);
        },
        Err(e) => {
            panic!("{}", e)
        }
    }
}

fn total_fresh_ingredients(mut lines: Lines<'_>) {
    let mut fresh_id_ranges: Vec<Range> = vec![];
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let mut split_line = line.split('-');
        let start: u64 = split_line.next().unwrap().parse::<u64>().unwrap();
        let end: u64 = split_line.next().unwrap().parse::<u64>().unwrap();
        fresh_id_ranges.push(Range { start, end });
    }
    fresh_id_ranges.sort_by_key(|r| r.start);
    println!("Fresh IDs: {:?}", fresh_id_ranges);

}


fn total_fresh_ids(mut lines: Lines<'_>) {
    let mut fresh_id_ranges: Vec<Range> = vec![];
        while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let mut split_line = line.split('-');
        let start: u64 = split_line.next().unwrap().parse::<u64>().unwrap();
        let end: u64 = split_line.next().unwrap().parse::<u64>().unwrap();
        fresh_id_ranges.push(Range { start, end });
    }
    fresh_id_ranges.sort_by_key(|r| r.start);

    let mut aggregate_fresh_id_ranges: Vec<Range> = vec![];

    let mut agg_range: Option<Range> = None;

    for curr in &fresh_id_ranges {

        match agg_range {
            Some(agg) => {
                if agg.end >= curr.start - 1 {
                    agg_range = Some(Range { 
                        start: agg.start, 
                        end: std::cmp::max(curr.end, agg.end) }
                    );
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


    let total = (&aggregate_fresh_id_ranges)
        .into_iter()
        .fold(0, |acc, Range{start, end}| acc + end + 1 - start);

    println!("Total fresh ids: {}", total)
}
