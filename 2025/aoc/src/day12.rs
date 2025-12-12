use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Present {
    figure: Vec<Vec<char>>,
    letter: char,
    filled: usize,
}

impl Present {
    fn new(figure: Vec<Vec<char>>, letter: char) -> Present {
        let filled= figure.iter().flatten().filter(|&&c| c == '#').count();
        Present { figure, letter, filled }
    }

    fn generate(present: Present) -> Vec<Present> {
        let mut presents = Vec::new();
        presents.push(present);
        for i in 0..3 {
            let mut next = presents.last().unwrap().clone();
            next.rotate();
            presents.push(next);
        }
        presents
    }

    fn rotate(&mut self) {
        let mut new_figure = self.figure.clone();
        new_figure[0][0] = self.figure[2][0];
        new_figure[0][1] = self.figure[1][0];
        new_figure[0][2] = self.figure[0][0];
        new_figure[1][0] = self.figure[2][1];
        new_figure[1][1] = self.figure[1][1];
        new_figure[1][2] = self.figure[0][1];
        new_figure[2][0] = self.figure[2][2];
        new_figure[2][1] = self.figure[1][2];
        new_figure[2][2] = self.figure[0][2];
        self.figure = new_figure;
    }

    fn can_fit(&self, i: usize, j: usize, grid: &mut Vec<Vec<char>>) -> bool {
        if i + 2 >= grid.len() || j + 2 >= grid[0].len() {
            return false;
        }
        for p in 0..3 {
            for q in 0..3 {
                if self.figure[p][q] == '#' && grid[i + p][j + q] != '.' {
                    return false;
                }
            }
        }
        true
    }

    fn fit(&self, i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
        for p in 0..3 {
            for q in 0..3 {
                if self.figure[p][q] == '#' {
                    grid[i + p][j + q] = self.letter;
                }
            }
        }
    }

    fn unfit(&self, i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
        for p in 0..3 {
            for q in 0..3 {
                if self.figure[p][q] == '#' {
                    grid[i + p][j + q] = '.';
                }
            }
        }
    }
}

fn solve(grid: &mut Vec<Vec<char>>, vacant: usize, presents: &Vec<Vec<Present>>, counts: &mut Vec<usize>, to_fill: usize) -> bool {
    if counts.iter().sum::<usize>() == 0 {
        // for i in 0..grid.len() {
        //     for j in 0..grid[0].len() {
        //         print!("{}", grid[i][j]);
        //     }
        //     println!();
        // }
        return true;
    }

    if vacant < to_fill {
        return false;
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for idx in 0..counts.len() {
                let count = counts[idx];
                if count == 0 {
                    continue
                }
                let variations = &presents[idx];
                for var in variations {
                    if grid[i][j] == '#' && var.figure[0][0] != '.' {
                        continue;
                    }
                    if var.can_fit(i, j, grid) {
                        var.fit(i, j, grid);
                        counts[idx] -= 1;
                        if solve(grid, vacant - var.filled, presents, counts, to_fill - var.filled) {
                            return true;
                        }
                        counts[idx] += 1;
                        var.unfit(i, j, grid);
                    }
                }
            }
        }
    }

    false
}

pub fn day12(filename: &str) -> u64 {
    let binding = read_to_string(filename).unwrap();
    let mut lines = binding.lines().peekable();
    let mut building = false;
    let mut figure = Vec::new();
    let mut presents = Vec::new();
    let mut letter = 'A';
    while let Some(line) = lines.peek() && !line.contains('x') {
        let line = lines.next().unwrap();
        if line.is_empty() {
            building = false;
            let present = Present::new(figure, letter);
            presents.push(Present::generate(present));
            figure = Vec::new();
            letter = char::from_u32(letter as u32 + 1).unwrap();
            continue
        }
        if line.contains(':') {
            building = true;
            continue;
        }
        if building {
            figure.push(line.chars().collect::<Vec<_>>());
        }
    }
    //println!("presents {:?}", presents);

    let mut sizes = Vec::new();
    let mut counts = Vec::new();
    while let Some(line) = lines.next() {
        let (sizes_str, rest) = line.split_once(": ").unwrap();
        let (x, y) = sizes_str.split_once('x').unwrap();
        sizes.push((y.parse::<usize>().unwrap(), x.parse::<usize>().unwrap()));
        let count: Vec<usize> = rest.split(' ').map(|x| x.parse::<usize>().unwrap()).collect();
        counts.push(count);
    }
    let mut total = 0;
    for idx in 0..sizes.len() {
        let (n, m) = sizes[idx];
        let mut count = &mut counts[idx];
        //println!("n: {}, m: {}, counts: {:?}", n, m, count);
        let mut grid = Vec::new();
        for _ in 0..n {
            let mut row = Vec::new();
            for _ in 0..m {
                row.push('.');
            }
            grid.push(row);
        }
        let mut to_fill = 0;
        for (idx, &c) in count.iter().enumerate() {
            to_fill += presents[idx][0].filled * c;
        }
        let res = solve(&mut grid, n*m, &presents, &mut count, to_fill);
        //println!("res {}", res);
        if res {
            total += 1;
        }
    }
    total
}