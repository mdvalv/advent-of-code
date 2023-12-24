use aoc_utils::init_challenge;
use rayon::prelude::*;

fn main() {
    let input = init_challenge();

    let mut part_1 = 0;
    let mut part_2 = 0;
    for line in input.lines() {
        let history: Vec<isize> = line
            .split(" ")
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        let (part_1_tmp, part_2_tmp) = process_history(history);
        part_1 += part_1_tmp;
        part_2 += part_2_tmp;
    }
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn process_history(history: Vec<isize>) -> (isize, isize) {
    if history.par_iter().all(|x| x == &history[0]) {
        return (history[0], history[0]);
    }

    let (part_1, part_2) = process_history(history.windows(2).map(|x| x[1] - x[0]).collect());
    return (
        history.last().unwrap() + part_1,
        history.first().unwrap() - part_2,
    );
}
