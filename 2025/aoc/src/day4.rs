use std::fs::read_to_string;

pub fn day4(filename: &str) -> u64 {
    let mut ans = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();
    for x in read_to_string(filename).unwrap().lines() {
        grid.push(x.chars().collect());
    }

    let mut grid1 = grid.clone();
    let n: i32 = grid.len().try_into().unwrap();
    let m: i32 = grid[0].len().try_into().unwrap();

    let mut prev_ans = 0;
    while ans == 0 || prev_ans != ans {
        prev_ans = ans;
        for i in 0..n {
            for j in 0..m {
                if grid[i as usize][j as usize] != '@' {
                    continue;
                }
                let mut count = 0;
                for p in -1..=1 {
                    if !(0 <= i + p && i + p < n) {
                        continue;
                    }
                    for q in -1..=1 {
                        if p == 0 && q == 0 {
                            continue;
                        }
                        if !(0 <= j + q && j + q < m) {
                            continue;
                        }
                        if grid[(i + p) as usize][(j + q) as usize] == '@' {
                            count += 1;
                        }
                    }
                }
                if count < 4 {
                    ans += 1;
                    grid1[i as usize][j as usize] = 'x';
                }
            }
        }
        println!("Removed {} roll of paper", ans - prev_ans);
        grid = grid1.clone(); // not efficient!
        //std::mem::swap(&mut grid1, &mut grid);
    }

    ans
}