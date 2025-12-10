use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;
use std::iter::Map;

fn flip(state: &Vec<char>, toggle: &Vec<usize>) -> Vec<char> {
    let mut new_state = state.clone();
    for i in toggle {
        new_state[*i] = match new_state[*i] {
            '.' => '#',
            '#' => '.',
            _ => unreachable!(),
        }
    }
    new_state
}

pub fn day10(filename: &str) -> u64 {
    let mut states = Vec::new();
    let mut toggles = Vec::new();
    for str in read_to_string(filename).unwrap().lines() {
        let (left, rest) = str.split_once("] (").unwrap();
        let state = left[1..].chars().collect::<Vec<char>>();
        states.push(state);
        let (toggles_str, _) = rest.split_once(") {").unwrap();
        let mut toggle = Vec::new();
        for toggle_str in toggles_str.split(") (") {
            let toggle_positions = toggle_str.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            toggle.push(toggle_positions);
        }
        toggles.push(toggle);
    }

    let mut ans = 0;
    for i in 0..states.len() {
        let mut q: VecDeque<Vec<char>> = VecDeque::new();
        let mut init_state = Vec::new();
        for _ in 0..states[i].len() {
            init_state.push('.');
        }
        q.push_back(init_state.clone());

        let mut found: HashMap<Vec<char>, usize> = HashMap::new();
        found.insert(init_state, 0);

        let desired_state = &states[i];
        while !q.is_empty() {
            let state = q.pop_front().unwrap();
            let count = found.get(&state).unwrap().clone();

            if let Some(desired_count) = found.get(desired_state) && count+1 > *desired_count {
                continue;
            }

            for toggle in &toggles[i] {
                let new_state = flip(&state, toggle);
                if !found.contains_key(&new_state) || found[&new_state] > count+1 {
                    found.insert(new_state.clone(), count + 1);
                    q.push_back(new_state);
                }
            }
        }
        ans += found.get(desired_state).unwrap();
    }
    ans as u64
}