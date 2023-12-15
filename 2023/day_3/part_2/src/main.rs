use aoc_utils::init_challenge;

const GEAR: char = '*';
const NOT_NUMBER: &str = "-";

type Schematic = Vec<Vec<String>>;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

type Gears = Vec<Point>;

fn main() {
    let input = init_challenge();

    let mut schematic: Schematic = Vec::new();
    let mut gears: Gears = Vec::new();
    process_input(input, &mut schematic, &mut gears);
    process_gears(&schematic, &gears);
}

fn process_input(input: String, schematic: &mut Schematic, gears: &mut Gears) {
    for (curr_y, line) in input.lines().enumerate() {
        schematic.push(Vec::new());

        let mut number = String::new();
        for (curr_x, c) in line.char_indices() {
            if c.is_ascii_digit() {
                if number.is_empty() {
                    number = get_full_number(line, curr_x);
                }
                schematic[curr_y].push(number.clone());
                continue;
            }
            number.clear();
            schematic[curr_y].push(NOT_NUMBER.to_string());
            if c == GEAR {
                gears.push(Point {
                    x: curr_x,
                    y: curr_y,
                });
            }
        }
    }

    for l in schematic {
        println!("{:?}", l);
    }
    println!("{:?}", gears);
}

fn get_full_number(line: &str, start_index: usize) -> String {
    let mut number: String = String::new();
    for c in line[start_index..].chars() {
        if c.is_ascii_digit() {
            number.push(c);
        } else {
            break;
        }
    }
    return number;
}

fn process_gears(schematic: &Schematic, gears: &Gears) {
    let mut count = 0;

    for gear in gears {
        let neighbors = find_neighbors(gear.x, gear.y, schematic);
        println!("Neighbors = {:?}", neighbors);
        if neighbors.len() == 2 {
            count += neighbors
                .iter()
                .fold(1, |acc, x| acc * x.parse::<usize>().unwrap())
        }
    }

    println!("Result = {count}");
}

type Neighbor = Vec<String>;

fn find_neighbors(x: usize, y: usize, schematic: &Schematic) -> Neighbor {
    let mut neighbors = Neighbor::new();

    // CCC | C = check  | Checks: (x-1, y), (x+1, y), (x-1, y-1), (x, y-1), (x+1, y-1), (x-1, y+1), (x, y+1), (x+1, y+1)
    // CsC | s = symbol | Beware! x-1 should check that x >= 1; y-1 should check that y >= 1;
    // CCC | s = symbol |         x+1 should check that x + 1 <= max_x; y+1 should check that y + 1 <= max_y

    if is_left_neighbor_number(x, y, schematic) {
        neighbors.push(schematic[y][x - 1].clone())
    }
    if is_right_neighbor_number(x, y, schematic) {
        neighbors.push(schematic[y][x + 1].clone())
    }

    if is_upper_neighbor_number(x, y, schematic) {
        neighbors.push(schematic[y - 1][x].clone())
    } else {
        if is_upper_right_neighbor_number(x, y, schematic) {
            neighbors.push(schematic[y - 1][x + 1].clone())
        }
        if is_upper_left_neighbor_number(x, y, schematic) {
            neighbors.push(schematic[y - 1][x - 1].clone())
        }
    }

    if is_bottom_neighbor_number(x, y, schematic) {
        neighbors.push(schematic[y + 1][x].clone())
    } else {
        if is_bottom_right_neighbor_number(x, y, schematic) {
            neighbors.push(schematic[y + 1][x + 1].clone());
        }
        if is_bottom_left_neighbor_number(x, y, schematic) {
            neighbors.push(schematic[y + 1][x - 1].clone())
        }
    }

    return neighbors;
}

fn is_left_neighbor_number(x: usize, y: usize, schematic: &Schematic) -> bool {
    return x >= 1 && schematic[y][x - 1] != NOT_NUMBER;
}

fn is_upper_left_neighbor_number(x: usize, y: usize, schematic: &Schematic) -> bool {
    return y >= 1 && x >= 1 && schematic[y - 1][x - 1] != NOT_NUMBER;
}

fn is_upper_neighbor_number(x: usize, y: usize, schematic: &Schematic) -> bool {
    return y >= 1 && schematic[y - 1][x] != NOT_NUMBER;
}

fn is_upper_right_neighbor_number(x: usize, y: usize, schematic: &Schematic) -> bool {
    return y >= 1 && x + 1 <= schematic[y - 1].len() - 1 && schematic[y - 1][x + 1] != NOT_NUMBER;
}

fn is_right_neighbor_number(x: usize, y: usize, schematic: &Schematic) -> bool {
    return x + 1 <= schematic[y].len() - 1 && schematic[y][x + 1] != NOT_NUMBER;
}

fn is_bottom_right_neighbor_number(x: usize, y: usize, schematic: &Schematic) -> bool {
    return y + 1 <= schematic.len() - 1
        && x + 1 <= schematic[y + 1].len() - 1
        && schematic[y + 1][x + 1] != NOT_NUMBER;
}

fn is_bottom_neighbor_number(x: usize, y: usize, schematic: &Schematic) -> bool {
    return y + 1 <= schematic.len() - 1 && schematic[y + 1][x] != NOT_NUMBER;
}

fn is_bottom_left_neighbor_number(x: usize, y: usize, schematic: &Schematic) -> bool {
    return y + 1 <= schematic.len() - 1 && x >= 1 && schematic[y + 1][x - 1] != NOT_NUMBER;
}
