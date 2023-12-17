use aoc_utils::init_challenge;

fn main() {
    let input = init_challenge();

    let mut final_count = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(&[':', '|'][..]).collect();
        let winning = Vec::from_iter(parts[1].trim().split(' '));
        let numbers = Vec::from_iter(parts[2].trim().split(' '));

        let mut count = 1;
        for n in numbers {
            if n != "" && winning.contains(&n) {
                count *= 2;
            }
        }

        if count != 1 {
            final_count += count / 2;
        }
    }

    println!("Result = {final_count}");
}
