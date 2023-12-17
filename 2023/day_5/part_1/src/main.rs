use std::collections::HashSet;

use aoc_utils::init_challenge;

#[derive(Debug)]
struct Range {
    destination: usize,
    source: usize,
    length: usize,
}

fn main() {
    let input = init_challenge();

    let mut lines = input.lines();

    let seeds_str: Vec<&str> = lines.next().unwrap().split(':').collect();
    let seeds: Vec<&str> = seeds_str[1].trim().split(' ').collect();
    let seeds: Vec<usize> = seeds.iter().map(|x| x.parse().unwrap()).collect();

    println!("Seeds = {seeds:?}");

    let mut maps: Vec<Vec<Range>> = Vec::new();
    let mut maps_index = 0;

    for line in lines {
        if line == "" {
            continue;
        }

        if line.ends_with(":") {
            maps.push(Vec::new());
            maps_index = maps.len() - 1;
            continue;
        }

        let parts: Vec<&str> = line.split(' ').collect();
        maps[maps_index].push(Range {
            destination: parts[0].parse().unwrap(),
            source: parts[1].parse().unwrap(),
            length: parts[2].parse().unwrap(),
        })
    }

    for map in &maps {
        println!("Map = {map:?}");
    }

    let mut locations: HashSet<usize> = HashSet::new();
    for seed in seeds {
        let mut curr_value = seed;
        for map in &maps {
            curr_value = find_value(curr_value, map);
        }
        locations.insert(curr_value);
    }

    println!("Locations = {locations:?}");

    let min = *locations.iter().min().unwrap();
    println!("Result = {min}");
}

fn find_value(from: usize, map: &Vec<Range>) -> usize {
    for range in map {
        if range.source <= from && from <= range.source + range.length {
            return range.destination + (from - range.source);
        }
    }
    return from;
}
