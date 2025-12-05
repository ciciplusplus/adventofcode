use std::fs::read_to_string;
use std::ops::Range;

pub fn day5(filename: &str) -> u64 {
    let mut ranges = Vec::new();
    let binding = read_to_string(filename).unwrap();
    let mut lines = binding.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (l, r) = line.split_once("-").unwrap();
        let r = Range { start: l.parse::<u64>().unwrap(), end: r.parse::<u64>().unwrap() + 1 };
        ranges.push(r);
    }
    let mut count = 0;
    while let Some(line) = lines.next() {
        let num = line.parse::<u64>().unwrap();
        for r in ranges.clone() { // DING! DING! DING! clone alert!
            if r.contains(&num) {
                count += 1;
                break
            }
        }
    }
    count
}