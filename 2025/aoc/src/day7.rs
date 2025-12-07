use std::fs::read_to_string;

pub fn day7(filename: &str) -> u64 {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for row in read_to_string(filename).unwrap().lines() {
        grid.push(row.chars().collect());
    }
    let mut split_count = 0;
    for row_idx in 1..grid.len() {
        for col_idx in 1..grid[row_idx].len() {
            match grid[row_idx - 1][col_idx] {
                '|' | 'S' => {
                    match grid[row_idx][col_idx] {
                        '.' | '|' => {
                            grid[row_idx][col_idx] = '|';
                        }
                        '^' => {
                            grid[row_idx][col_idx - 1] = '|';
                            grid[row_idx][col_idx + 1] = '|';
                            split_count += 1;
                        }
                        _ => unreachable!()
                    }
                }
                _ => {}
            }
        }
    }
    split_count
}