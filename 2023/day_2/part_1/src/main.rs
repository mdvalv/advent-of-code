use aoc_utils::init_challenge;

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

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
        let (game, draws) = &line.split_once(":").unwrap();
        let (_, game_id) = game.split_once(" ").unwrap();

        let mut count_game = true;
        for draw in draws.split(";") {
            for cubes in draw.split(",") {
                let (number, color) = cubes.trim().split_once(" ").unwrap();
                let number: i32 = number.parse().unwrap();
                let max;
                match color {
                    "red" => {
                        max = RED;
                    }
                    "green" => {
                        max = GREEN;
                    }
                    "blue" => {
                        max = BLUE;
                    }
                    _ => panic!("Unknown color: {color}"),
                }
                if number > max {
                    count_game = false;
                    break;
                }
            }
            if !count_game {
                break;
            }
        }
        if count_game {
            count += game_id.parse::<i32>().unwrap();
        }
    }

    println!("Result: {count}");
}
