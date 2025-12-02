use std::env::args;

mod day1;

fn main() {
    let filename = args().nth(1).unwrap();
    let res = day1::day1(&filename);
    print!("res {}", res);
}
