use std::fs;


fn main() {
    let res = fs::read_to_string("./input.txt");
    match res {
        Ok(input) => {
            let sum = input.lines()
                .into_iter()
                .map(|bank| { 
                    bank.chars()
                        .map(|x| x.to_digit(10).unwrap())
                        .collect()
                })
                .fold(0, |sum, bank| 
                    sum + find_max_digits(bank, 12)
                );
            println!("Totla joltage is: {}", sum)
        },
        Err(e) => {
            panic!("{}", e)
        }
    }
}

fn find_max_digits(nums: Vec<u32>, digit_count: usize) -> u64 {
    (0..digit_count)
        .fold((-1, "".to_string()), |(prev_max_index, joltage), i| {
            let start_from = (prev_max_index + 1) as usize;
            let end_at = nums.len() - (digit_count - 1) + i as usize;
            let max_index = prev_max_index 
                + 1 
                + find_max_index(nums[start_from..end_at].to_vec());
            (
                max_index,
                format!("{}{}", joltage, nums[max_index as usize])
            )
        })
        .1
        .parse()
        .unwrap()
}

fn find_max_index(nums: Vec<u32>) -> i32 {
    nums
        .iter()
        .enumerate()
        .fold((0,0), |(max, max_index), (i, &n)| {
            (
                if n > max { n } else { max },
                if n > max { i } else { max_index },
            )
        })
        .1
        as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_max_index_no_repitions() {
        let res = find_max_index(vec![3,7,1,9,8,4]);
        assert_eq!(res, 3);
    }

    #[test]
    fn find_max_index_with_repitions() {
        let res = find_max_index(vec![3,7,8,4,8]);
        assert_eq!(res, 2);
    }

    #[test]
    fn find_max_2_digits_max_not_last() {
        let res = find_max_digits(vec![3,7,8,4,5], 2);
        assert_eq!(res, 85);
    }

    #[test]
    fn find_max_5_digits() {
        let res = find_max_digits(vec![3,7,8,9,4,5,5,7,4,3,6], 5);
        assert_eq!(res, 97436);
    }
}