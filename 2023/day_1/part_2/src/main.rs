use aoc_utils::init_challenge;

use regex::Regex;

fn as_number_str(s: &str) -> &str {
    match s {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => s,
    }
}

fn main() {
    let input = init_challenge();
    let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let mut count = 0;

    // we may have overlaps, for example:
    // eightwo, should return two
    // twone, should return one
    // so search by index

    for line in input.lines() {
        let mut first = "";
        let mut last = "";

        for cap in line
            .char_indices()
            .filter_map(|(i, _)| re.captures(&line[i..]))
        {
            if cap.len() == 1 {
                if first == "" {
                    first = as_number_str(cap.get(0).unwrap().as_str());
                }
                last = as_number_str(cap.get(0).unwrap().as_str());
            }
        }

        let value = format!("{}{}", first, last);
        if value.len() >= 1 {
            count += value.parse::<i64>().expect("Failed to parse value");
        }
    }

    println!("Result: {count}");
}
