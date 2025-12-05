use std::fs::read_to_string;
use std::ops::Range;

pub fn day5(filename: &str) -> u64 {
    let mut ranges: Vec<Range<u64>> = Vec::new();
    let binding = read_to_string(filename).unwrap();
    let mut lines = binding.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (l, r) = line.split_once("-").unwrap();
        let mut r = Range { start: l.parse::<u64>().unwrap(), end: r.parse::<u64>().unwrap() + 1 };
        let mut i = 0;
        while i < ranges.len() {
            let rr = &ranges[i];
            if rr.contains(&r.start) || r.contains(&rr.start) {
                r.start = r.start.min(rr.start);
                r.end = r.end.max(rr.end);
                ranges.remove(i);
            } else {
                i += 1;
            }
        }
        ranges.push(r);
    }
    let mut count: u64 = 0;
    for r in ranges {
        count += r.count() as u64;
    }
    count
}