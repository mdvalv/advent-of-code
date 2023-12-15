use aoc_utils::init_challenge;
use std::collections::HashMap;

const SYMBOL: usize = 0;
const DOT: usize = 1;

#[derive(Debug)]
struct NumberTrack {
    number: String,
    found: bool,
}

// Map the number id to the symbols that are adjacent to it
type PartNumbers = HashMap<usize, NumberTrack>;

type Schematic = Vec<Vec<usize>>;

fn main() {
    let input = init_challenge();
    process_input(input)
}

fn process_input(input: String) {
    let mut schematic: Schematic = Vec::new();
    let mut part_numbers: PartNumbers = HashMap::new();
    let mut number_id = 2;

    for (curr_y, line) in input.lines().enumerate() {
        schematic.push(Vec::new());

        let mut number = String::new();
        let mut number_x_0 = 0;
        for (curr_x, c) in line.char_indices() {
            if c.is_ascii_digit() {
                process_digit(
                    &mut number,
                    number_id,
                    line,
                    curr_x,
                    curr_y,
                    &mut number_x_0,
                    &schematic,
                    &mut part_numbers,
                );
                schematic[curr_y].push(number_id);
                continue;
            }

            number_id += 1;
            number.clear();
            if c != '.' {
                schematic[curr_y].push(SYMBOL);
                process_symbol(curr_x, curr_y, &schematic, &mut part_numbers);
            } else {
                schematic[curr_y].push(DOT);
            }
        }
    }

    for l in schematic {
        println!("{:?}", l);
    }
    println!("==================================================");
    // println!("{:?}", part_numbers);
    dbg!(&part_numbers);
    println!("==================================================");
    let count = part_numbers.iter().fold(0, |acc, (_, x)| {
        let mut tmp = 0;
        if x.found {
            tmp = x.number.parse::<i32>().unwrap();
        }
        return acc + tmp;
    });
    println!("Result = {count}");
}

fn process_digit(
    number: &mut String,
    number_id: usize,
    line: &str,
    curr_x: usize,
    curr_y: usize,
    number_x_0: &mut usize,
    schematic: &Schematic,
    part_numbers: &mut PartNumbers,
) {
    let is_first: bool;
    if number.is_empty() {
        *number_x_0 = curr_x;
        get_full_number(number, line, curr_x);
        is_first = true;
    } else {
        is_first = false;
    }

    if !part_numbers.contains_key(&number_id) {
        part_numbers.insert(
            number_id,
            NumberTrack {
                number: number.to_string(),
                found: false,
            },
        );
    }

    if is_any_neighbor_symbol(curr_x, curr_y, is_first, schematic) {
        part_numbers.entry(number_id).and_modify(|x| x.found = true);
    }
}

fn get_full_number(number: &mut String, line: &str, start_index: usize) {
    for c in line[start_index..].chars() {
        if c.is_ascii_digit() {
            number.push(c);
        } else {
            break;
        }
    }
}

fn is_left_neighbor_symbol(x: usize, y: usize, schematic: &Schematic) -> bool {
    return x >= 1 && schematic[y][x - 1] == SYMBOL;
}

fn is_upper_left_neighbor_symbol(x: usize, y: usize, schematic: &Schematic) -> bool {
    return y >= 1 && x >= 1 && schematic[y - 1][x - 1] == SYMBOL;
}

fn is_upper_neighbor_symbol(x: usize, y: usize, schematic: &Schematic) -> bool {
    return y >= 1 && schematic[y - 1][x] == SYMBOL;
}

fn is_upper_right_neighbor_symbol(x: usize, y: usize, schematic: &Schematic) -> bool {
    return y >= 1 && x + 1 <= schematic[y - 1].len() - 1 && schematic[y - 1][x + 1] == SYMBOL;
}

fn is_any_neighbor_symbol(x: usize, y: usize, is_first: bool, schematic: &Schematic) -> bool {
    if is_first {
        // CCC | C = check          | Checks: (x-1, y), (x-1, y-1), (x, y-1), (x+1, y-1)
        // Cn? | n = number         | Beware! x-1 should check that x >= 1; y-1 should check that y >= 1;
        //     | ? = not mapped yet |         x+1 should check that x + 1 <= max_x
        return is_left_neighbor_symbol(x, y, schematic)
            || is_upper_left_neighbor_symbol(x, y, schematic)
            || is_upper_neighbor_symbol(x, y, schematic)
            || is_upper_right_neighbor_symbol(x, y, schematic);
    } else {
        // xC | C = check                        | Checks: (x+1, y-1)
        // n? | n = number                       | Beware! y-1 should check that y >= 1;
        //    | x = already checked by the first |         x+1 should check that x + 1 <= max_x
        //    | ? = not mapped yet               |
        return is_upper_right_neighbor_symbol(x, y, schematic);
    }
}

fn is_left_neighbor_number(x: usize, y: usize, schematic: &Schematic) -> bool {
    return x >= 1 && is_number(schematic[y][x - 1]);
}

fn is_upper_left_neighbor_number(x: usize, y: usize, schematic: &Schematic) -> bool {
    return y >= 1 && x >= 1 && is_number(schematic[y - 1][x - 1]);
}

fn is_upper_neighbor_number(x: usize, y: usize, schematic: &Schematic) -> bool {
    return y >= 1 && is_number(schematic[y - 1][x]);
}

fn is_upper_right_neighbor_number(x: usize, y: usize, schematic: &Schematic) -> bool {
    return y >= 1 && x + 1 <= schematic[y - 1].len() - 1 && is_number(schematic[y - 1][x + 1]);
}

fn process_symbol(x: usize, y: usize, schematic: &Schematic, part_numbers: &mut PartNumbers) {
    // CCC | C = check          | Checks: (x-1, y), (x-1, y-1), (x, y-1), (x+1, y-1)
    // Cs? | s = symbol         | Beware! x-1 should check that x >= 1; y-1 should check that y >= 1;
    //     | ? = not mapped yet |         x+1 should check that x <= max_x
    if is_left_neighbor_number(x, y, schematic) {
        part_numbers
            .entry(schematic[y][x - 1])
            .and_modify(|x| x.found = true);
    }
    if is_upper_left_neighbor_number(x, y, schematic) {
        part_numbers
            .entry(schematic[y - 1][x - 1])
            .and_modify(|x| x.found = true);
    }
    if is_upper_neighbor_number(x, y, schematic) {
        part_numbers
            .entry(schematic[y - 1][x])
            .and_modify(|x| x.found = true);
    }
    if is_upper_right_neighbor_number(x, y, schematic) {
        part_numbers
            .entry(schematic[y - 1][x + 1])
            .and_modify(|x| x.found = true);
    }
}

fn is_number(v: usize) -> bool {
    return v != SYMBOL && v != DOT;
}
