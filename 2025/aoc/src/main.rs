use std::env::args;
use std::fs::read_to_string;

fn main() {
    let mut start = 50;
    let mut turns_count = 0;

    let filename = args().nth(1).unwrap();
    for line in read_to_string(filename).unwrap().lines() {
        let amount = line[1..].parse::<i32>().unwrap();
        let amount = amount % 100;
        let to_turn = match line.chars().nth(0).unwrap() {
            'R' => amount,
            'L' => -amount,
            _ => unreachable!()
        };
        let mut current = start + to_turn;
        if current < 0 {
            current = current + 100;
        } else if current > 99 {
            current = current % 100;
        }
        start = current;
        if start == 0 {
            turns_count += 1;
        }
    }

    print!("pass {}", turns_count);
}
