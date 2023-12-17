use aoc_utils::init_challenge;

fn main() {
    let input = init_challenge();
    let mut lines = input.lines();

    let times: Vec<&str> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .collect();
    let distances: Vec<&str> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .collect();

    let mut count: Vec<usize> = Vec::new();
    for (time, distance) in times.iter().zip(distances.iter()) {
        let time = time.parse::<usize>().unwrap();
        let distance = distance.parse::<usize>().unwrap();
        count.push(count_better_distances(time, distance))
    }

    let count = count.iter().fold(1, |acc, x| acc * x);
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
