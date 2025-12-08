use std::env::args;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod union_find;
mod day8;

fn main() {
    let filename = args().nth(1).unwrap();
    //let res = day1::day1(&filename);
    //let res = day2::day2(&filename);
    //let res = day3::day3(&filename);
    //let res = day4::day4(&filename);
    //let res = day5::day5(&filename);
    //let res = day6::day6(&filename);
    //let res = day7::day7(&filename);
    let res = day8::day8(&filename);
    print!("res {}", res);
}
