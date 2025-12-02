use std::env::args;

mod day1;
mod day2;

fn main() {
    let filename = args().nth(1).unwrap();
    //let res = day1::day1(&filename);
    let res = day2::day2(&filename);
    print!("res {}", res);
}
