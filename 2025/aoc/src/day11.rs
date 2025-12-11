use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

pub fn day11(filename: &str) -> u64 {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let binding = read_to_string(filename).unwrap();
    for str in binding.lines() {
        let (node, neighbours_str) = str.split_once(": ").unwrap();
        let mut neighbours = Vec::new();
        for n in neighbours_str.split(' ') {
            neighbours.push(n);
        }
        graph.insert(node, neighbours);
    }
    let mut ans = 0;
    let mut q = VecDeque::new();
    q.push_back("you");
    while !q.is_empty() {
        let next = q.pop_front().unwrap();
        if next == "out" {
            ans += 1;
            continue;
        }
        for n in &graph[&next] {
            q.push_back(n);
        }
    }
    ans
}