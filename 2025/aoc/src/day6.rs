use std::fs::read_to_string;

fn find_num(str: &Vec<char>, offset: usize) -> usize {
    let mut new_offest = offset;
    // skip whitespaces
    while new_offest < str.len() && str[new_offest] == ' ' {
        new_offest += 1;
    }
    while new_offest < str.len() && str[new_offest].is_digit(10) {
        new_offest += 1;
    }
    new_offest
}

pub fn day6(filename: &str) -> u64 {
    let mut ans = 0;

    let mut grid: Vec<Vec<char>> = Vec::new();
    let binding = read_to_string(filename).unwrap();
    let mut row_len = 0;
    for row in binding.lines() {
        let row: Vec<char> = row.chars().collect();
        row_len = row_len.max(row.len());
        grid.push(row);
    }

    for row in &mut grid {
        if row.len() < row_len {
            row.extend(vec![' '; row_len - row.len()]);
        }
    }

    let mut offset = 0;
    while offset < grid[0].len() {
        let mut next_offest = offset;
        for i in 0..grid.len()-1 {
            next_offest = next_offest.max(find_num(&grid[i], offset));
        }

        let op = grid.last().unwrap()[offset];
        let mut num: u64 = if op == '+' {
            0
        } else {
            1
        };
        for j in offset..next_offest {
            let mut tmp = 0;
            let mut skip_blank = 0;
            while grid[skip_blank][j] == ' ' {
                skip_blank += 1;
            }
            for i in skip_blank..grid.len()-1 {
                if grid[i][j] == ' ' {
                    break;
                }
                tmp = tmp * 10 + grid[i][j].to_digit(10).unwrap();
            }
            //println!("{}", tmp);
            if op == '+' {
                num += tmp as u64;
            } else {
                num *= tmp as u64;
            }
        }
        ans += num;
        //println!();

        if next_offest == grid[0].len() {
            break;
        }

        assert_eq!(grid[0][next_offest], ' ');
        offset = next_offest + 1;
    }
    ans
}