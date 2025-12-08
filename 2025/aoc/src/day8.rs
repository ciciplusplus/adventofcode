use std::fs::read_to_string;
use crate::union_find::UnionFind;

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(str: &str) -> Self {
        let (x, yz) = str.split_once(",").unwrap();
        let (y, z) = yz.split_once(",").unwrap();
        Self { x: x.parse().unwrap(), y: y.parse().unwrap(), z: z.parse().unwrap() }
    }

    fn distance(&self, p: &Point) -> u64 {
        let dx = p.x - self.x;
        let dy = p.y - self.y;
        let dz = p.z - self.z;
        (dx * dx + dy * dy + dz * dz).try_into().unwrap()
    }
}

pub fn day8(filename: &str) -> u64 {
    let mut points = Vec::new();
    for point in read_to_string(filename).unwrap().lines() {
        points.push(Point::new(point));
    }
    let mut distances = Vec::new();
    for i in 0..points.len() {
        for j in i+1..points.len() {
            distances.push((i, j, points[i].distance(&points[j])));
        }
    }
    distances.sort_by(|a, b| a.2.cmp(&b.2));
    let mut uf = UnionFind::new(points.len());
    for p in 0..1000 {
        let (i, j, _) = distances[p];
        uf.union(i, j);
    }
    let sizes = uf.sizes();
    let mut maxes: [usize; 3] = [0; 3];
    for size in sizes.values() {
        if *size >= maxes[0] {
            maxes[2] = maxes[1];
            maxes[1] = maxes[0];
            maxes[0] = *size;
        } else if *size >= maxes[1] {
            maxes[2] = maxes[1];
            maxes[1] = *size;
        } else if *size >= maxes[2] {
            maxes[2] = *size;
        }
    }
    (maxes[0] * maxes[1] * maxes[2]) as u64
}