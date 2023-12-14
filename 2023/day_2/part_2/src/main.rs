use std::collections::HashMap;

use aoc_utils::init_challenge;

fn main() {
    let input = init_challenge();

    let mut count = 0;

    for line in input.lines() {
        if line == "" {
            continue;
        }

        // format is:
        // Game <id>: <1st draw>; <2nd draw>; ...
        // where each draw is of the format:
        // <number for color 1> <color1>, <number for color 2> <color2>, ...
        let (_, draws) = &line.split_once(":").unwrap();
        let mut colors = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        for draw in draws.split(";") {
            for cubes in draw.split(",") {
                let (number, color) = cubes.trim().split_once(" ").unwrap();
                let number: i32 = number.parse().unwrap();

                colors.entry(color).and_modify(|e| {
                    if number > *e {
                        *e = number;
                    }
                });
            }
        }
        count += colors.values().fold(1, |acc, x| acc * x);
    }

    println!("Result: {count}");
}
