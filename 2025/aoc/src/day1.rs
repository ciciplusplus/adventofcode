use std::fs::read_to_string;

pub fn day1(filename: &str) -> i32 {
    let mut start = 50;
    let mut turns_count = 0;
    
    for line in read_to_string(filename).unwrap().lines() {
        let amount = line[1..].parse::<i32>().unwrap();
        turns_count += amount / 100;
        let amount = amount % 100;
        let to_turn = match line.chars().nth(0).unwrap() {
            'R' => amount,
            'L' => -amount,
            _ => unreachable!()
        };
        let mut current = start + to_turn;
        if current < 0 {
            current = current + 100;
            turns_count += 1;
        } else if current > 99 {
            current = current % 100;
            turns_count += 1;
        }
        start = current;
    }
    
    turns_count
}