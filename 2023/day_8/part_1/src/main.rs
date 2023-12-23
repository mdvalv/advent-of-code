use aoc_utils::init_challenge;
use std::{collections::HashMap, str::Chars};

const START: &str = "AAA";
const END: &str = "ZZZ";

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

type Map = HashMap<String, Node>;

fn main() {
    let input = init_challenge();
    let mut lines = input.lines();

    let directions = lines.next().unwrap().chars();
    let mut map = Map::new();

    // skip empty line
    lines.next();

    for line in lines {
        // line format: AAA = (BBB, CCC)
        let (node, parts) = line.split_once(" = ").unwrap();
        let x: &[_] = &['(', ')'];
        let (left, right) = parts.trim_matches(x).split_once(", ").unwrap();

        map.insert(
            node.to_string(),
            Node {
                left: left.to_string(),
                right: right.to_string(),
            },
        );
    }

    let steps = 0;
    let current = START.to_string();

    walk_map(map, directions, current, steps);
}

fn walk_map(map: Map, directions: Chars, mut current: String, mut steps: usize) {
    for direction in directions.clone() {
        steps += 1;
        if direction == 'L' {
            current = map.get(&current).unwrap().left.clone();
        } else {
            current = map.get(&current).unwrap().right.clone();
        }
        if current == END {
            println!("Result = {}", steps);
            return;
        }
    }
    walk_map(map, directions, current, steps)
}
