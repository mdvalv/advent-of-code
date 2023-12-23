use aoc_utils::init_challenge;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    str::Chars,
    sync::atomic::{AtomicUsize, Ordering},
};

const START: &str = "A";
const END: &str = "Z";

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

type Paths = HashMap<String, HashMap<String, String>>;
type Map = HashMap<String, Node>;

fn main() {
    let input = init_challenge();
    let mut lines = input.lines();

    let directions = lines.next().unwrap().chars();
    let mut map = Map::new();
    let mut paths = Paths::new();

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
        paths.insert(node.to_string(), HashMap::new());
    }

    build_paths(&map, directions.clone(), &mut paths);

    let steps = 0;
    let current_nodes = get_starting_nodes(&map);
    println!("Starting nodes: {:?}", current_nodes);
    walk_map_first_node(
        directions,
        current_nodes[0].clone(),
        current_nodes[1..].to_vec(),
        steps,
        paths,
    );
}

fn build_paths(map: &Map, directions: Chars, paths: &mut Paths) {
    map.keys().for_each(|k| {
        let mut direction_str = String::new();
        let mut current = k.clone();
        for direction in directions.clone() {
            let next = get_next_node(&map, current, direction);
            direction_str += direction.to_string().as_str();
            paths.entry(k.clone()).and_modify(|e| {
                e.insert(direction_str.clone(), next.clone());
            });
            current = next;
        }
    });
}

fn get_starting_nodes(map: &Map) -> Vec<String> {
    let mut nodes: Vec<String> = Vec::new();
    for key in map.keys() {
        if key.ends_with(START) {
            nodes.push(key.clone());
        }
    }
    return nodes;
}

fn walk_map_first_node(
    directions: Chars,
    mut first_node: String,
    mut current_nodes: Vec<String>,
    mut steps: usize,
    paths: Paths,
) {
    let mut all_finished: bool;
    let mut next = String::from("");

    println!("First node: {}", first_node);

    loop {
        let mut direction_str = String::new();

        for direction in directions.clone() {
            steps += 1;
            direction_str += direction.to_string().as_str();
            next = paths[&first_node][&direction_str].clone();
            print!("\r{:?} steps = {}", current_nodes, steps);

            if next.ends_with(END) {
                (_, all_finished) = walk_others(&current_nodes, &paths, direction_str.clone());
                if all_finished {
                    println!("\nResult = {}", steps);
                    return;
                }
            }
        }

        let (next_nodes, _) = walk_others(&current_nodes, &paths, direction_str.clone());
        current_nodes = next_nodes;
        first_node = next.clone();
    }
}

fn walk_others(
    current_nodes: &Vec<String>,
    paths: &Paths,
    direction_str: String,
) -> (Vec<String>, bool) {
    let all_finished = AtomicUsize::new(current_nodes.len());

    let next_nodes: Vec<String> = current_nodes
        .par_iter()
        .fold(Vec::new, |mut acc, current| {
            let next = paths[current][&direction_str].clone();
            if next.ends_with(END) {
                all_finished.fetch_sub(1, Ordering::Relaxed);
            }
            acc.push(next);
            return acc;
        })
        .reduce(Vec::new, |mut acc, next| {
            acc.extend(next);
            return acc;
        });

    return (next_nodes, all_finished.load(Ordering::Relaxed) == 0);
}

fn get_next_node(map: &Map, current: String, direction: char) -> String {
    if direction == 'L' {
        return map.get(&current).unwrap().left.clone();
    } else {
        return map.get(&current).unwrap().right.clone();
    }
}
