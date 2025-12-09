use std::fs::read_to_string;

pub fn day9(filename: &str) -> u64 {
    let mut points = Vec::new();
    for str in read_to_string(filename).unwrap().lines() {
        let (x, y) = str.split_once(',').unwrap();
        points.push((x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()));
    }
    let mut max = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dist_x: u64 = (points[i].0 - points[j].0).abs() as u64 + 1;
            let dist_y: u64 = (points[i].1 - points[j].1).abs() as u64 + 1;
            max = max.max(dist_x * dist_y);
        }
    }
    max
}