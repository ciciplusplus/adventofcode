use std::fs::read_to_string;

fn get_max(str: &str, start_idx: usize, end_idx: usize) -> (usize, u64) {
    let mut max_idx = start_idx;
    let mut max_val = str.chars().nth(start_idx).unwrap();
    for i in start_idx..=end_idx {
        let curr = str.chars().nth(i).unwrap();
        if curr > max_val {
            max_idx = i;
            max_val = curr;
        }
    }
    (max_idx, max_val.to_digit(10).unwrap().into())
}

pub fn day3(filename: &str) -> u64 {
    let mut ans = 0;
    for x in read_to_string(filename).unwrap().lines() {
        let (first_idx, first) = get_max(x, 0, x.len()-1);
        let max = if first_idx == x.len()-1 {
            let (_, before) = get_max(x, 0, first_idx-1);
            before * 10 + first
        } else {
            let (_, second) = get_max(x, first_idx+1, x.len()-1);
            first * 10 + second
        };
        ans += max;
    }
    ans
}