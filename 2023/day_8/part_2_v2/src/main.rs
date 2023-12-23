/* This is a v2 of part_2, because in my opinion, part_2 is a much more general solution.
   This one was made after seeing that the steps are always the same for each starting node.
   So it means that the problem could be solved by only finding the least common multiple.
*/
use aoc_utils::init_challenge;
use rayon::prelude::*;
use std::{collections::HashMap, sync::Mutex};

const START: &str = "A";
const END: &str = "Z";

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

type Map = HashMap<String, Node>;

fn main() {
    let input = init_challenge();
    let mut lines = input.lines();

    let directions = lines.next().unwrap();
    let mut map = Map::new();
    let mut starting_nodes = Vec::new();

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

        if node.ends_with(START) {
            starting_nodes.push(node.to_string());
        }
    }
    println!("Starting nodes: {:?}", starting_nodes);

    let nodes_steps = find_steps(&map, starting_nodes, directions);
    println!("Steps: {:?}", nodes_steps);

    println!("Result = {}", find_lcm(&nodes_steps));
}

fn find_steps(map: &Map, starting_nodes: Vec<String>, directions: &str) -> Vec<usize> {
    let steps = Mutex::new(Vec::new());

    starting_nodes.par_iter().for_each(|node| {
        let mut count = 0;
        let mut next = node.clone();
        loop {
            for direction in directions.chars() {
                count += 1;
                next = get_next_node(map, next, direction);

                if next.ends_with(END) {
                    steps.lock().unwrap().push(count);
                    return;
                }
            }
        }
    });

    return steps.into_inner().unwrap();
}

fn get_next_node(map: &Map, current: String, direction: char) -> String {
    if direction == 'L' {
        return map.get(&current).unwrap().left.clone();
    } else {
        return map.get(&current).unwrap().right.clone();
    }
}

fn find_lcm(numbers: &[usize]) -> usize {
    let mut lcm = numbers[0];

    for &number in numbers.iter().skip(1) {
        lcm = lcm * number / gcd(lcm, number);
    }

    lcm
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
