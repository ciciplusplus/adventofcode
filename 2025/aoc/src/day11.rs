use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

fn count<'a>(curr: &'a str, to: &str, graph: &HashMap<&str, Vec<&'a str>>, mem: &mut HashMap<&'a str, u64>) -> u64 {
    if mem.contains_key(curr) {
        return mem[curr];
    }
    let res = if curr == to {
        1
    } else {
        let mut total = 0;
        if let Some(x) = graph.get(curr) {
            for y in x {
                total += count(y, to, graph, mem);
            }
        }
        total
    };
    mem.insert(curr, res);
    res
}

pub fn day11(filename: &str) -> u64 {
    let mut reverse_graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let binding = read_to_string(filename).unwrap();
    for str in binding.lines() {
        let (node, neighbours_str) = str.split_once(": ").unwrap();
        for n in neighbours_str.split(' ') {
            reverse_graph.entry(n).or_default().push(node);
        }
    }
    let mut res = 1;
    let mut mem = HashMap::new();
    res *= count("fft", "svr", &reverse_graph, &mut mem);
    mem = HashMap::new();
    res *= count("dac", "fft", &reverse_graph, &mut mem);
    mem = HashMap::new();
    res *= count("out", "dac", &reverse_graph, &mut mem);
    res
}