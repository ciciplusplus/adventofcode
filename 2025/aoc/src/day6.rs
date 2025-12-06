use std::fs::read_to_string;

pub fn day6(filename: &str) -> u64 {
    let mut ans = 0;
    let mut grid: Vec<Vec<u64>> = Vec::new();
    let mut reached_last = false;
    for x in read_to_string(filename).unwrap().lines() {
        let mut row = Vec::new();
        for (idx, y) in x.split_whitespace().enumerate() {
            if y == "*" || y == "+" {
                match y {
                    "*" => {
                        let mut tmp: u64 = 1;
                        for i in 0..grid.len() {
                            tmp *= grid[i][idx];
                        }
                        ans += tmp;
                        reached_last = true;
                    }
                    "+" => {
                        let mut tmp: u64 = 0;
                        for i in 0..grid.len() {
                            tmp += grid[i][idx];
                        }
                        ans += tmp;
                        reached_last = true;
                    }
                    _ => unreachable!()
                }
            } else {
                row.push(y.parse::<u64>().unwrap());
            }
        }
        if reached_last {
            break;
        }
        grid.push(row);
    }
    ans
}