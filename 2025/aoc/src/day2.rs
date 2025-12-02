use std::fs::read_to_string;

fn is_invalid(x: u64) -> bool {
    let str = x.to_string();
    let len = str.len();
    str[..len/2] == str[len/2..]
}

pub fn day2(filename: &str) -> u64 {
    let mut sum: u64 = 0;

    let binding = read_to_string(filename).unwrap();
    let line = binding.lines().nth(0).unwrap();

    for part in line.split(',') {
        let mut range = part.split('-');
        let left = range.next().unwrap().parse::<u64>().unwrap();
        let right = range.next().unwrap().parse::<u64>().unwrap();
        for x in left..=right {
            if is_invalid(x) {
                sum += x;
            }
        }
    }

    sum
}