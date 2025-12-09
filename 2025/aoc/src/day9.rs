use std::fs::read_to_string;

pub fn day9(filename: &str) -> u64 {
    let mut points = Vec::new();
    for str in read_to_string(filename).unwrap().lines() {
        let (x, y) = str.split_once(',').unwrap();
        points.push((x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()));
    }
    let mut max = 0;
    // bottom-left quadrant
    let (px, py) = (94926, 48373);
    let mut maxx = 0;
    let mut maxy = 0;
    for i in 0..points.len() {
        let (x, y) = points[i];
        if x >= 50000 || y >= 50000 {
            continue;
        }
        if y > 48373 || y < 34839 {
            continue;
        }
        let dist_x = (x - px).abs() as u64 + 1;
        let dist_y = (y - py).abs() as u64 + 1;
        if max < dist_x * dist_y {
            max = dist_x * dist_y;
            maxx = x;
            maxy = y;
        }
    }
    println!("x: {}, y: {} max {}", maxx, maxy, max);
    // upper-left quadrant
    let (qx, qy) = (94926, 50372);
    for i in 0..points.len() {
        let (x, y) = points[i];
        if x >= 50000 || y <= 50000 {
            continue;
        }
        if y > 65764 || y < 50372 {
            continue;
        }
        let dist_x = (x - qx).abs() as u64 + 1;
        let dist_y = (y - qy).abs() as u64 + 1;
        if max < dist_x * dist_y {
            max = dist_x * dist_y;
            maxx = x;
            maxy = y;
        }
    }
    println!("x: {}, y: {} max {}", maxx, maxy, max);
    max
}