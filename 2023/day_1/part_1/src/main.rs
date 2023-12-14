use aoc_utils::init_challenge;
use regex::Regex;

fn main() {
    let input = init_challenge();

    let mut count = 0;
    let re = Regex::new(r"\D*(\d?).*(\d)\D*").unwrap();

    for (_, [mut first, second]) in re.captures_iter(&input).map(|c| c.extract()) {
        if first.is_empty() {
            first = &second;
        }
        let line = &format!("{first}{second}");
        count += line.parse::<i64>().expect("Failed to parse value");
    }

    println!("Result: {count}");
}
