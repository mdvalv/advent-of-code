use aoc_utils::init_challenge;

fn main() {
    let input = init_challenge();
    let mut lines = input.lines();

    let time: usize = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();
    let distance: usize = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();

    println!("time = {time}");
    println!("distance = {distance}");

    let count = count_better_distances(time, distance);
    println!("Result = {count}");
}

fn count_better_distances(time: usize, distance: usize) -> usize {
    let mut better = 0;
    for time_button in 1..time {
        if calc_distance(time, time_button) > distance {
            better += 1
        }
    }
    return better;
}

fn calc_distance(time: usize, time_button: usize) -> usize {
    return time_button * (time - time_button);
}
