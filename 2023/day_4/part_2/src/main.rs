use aoc_utils::init_challenge;
use std::collections::HashMap;

type Cards = HashMap<usize, usize>;

fn main() {
    let input = init_challenge();

    let mut cards = Cards::new();
    let mut final_count = 0;

    for (l, line) in input.lines().enumerate() {
        let card_number = l + 1;
        let card_copies = cards
            .entry(card_number)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        let card_copies = *card_copies;

        let parts: Vec<&str> = line.split(&[':', '|'][..]).collect();
        let winning = Vec::from_iter(parts[1].trim().split(' '));
        let numbers = Vec::from_iter(parts[2].trim().split(' '));

        let mut matches = 0;
        for n in numbers {
            if n != "" && winning.contains(&n) {
                matches += 1;
            }
        }


        for i in 1..matches+1 {
            cards
                .entry(card_number + i)
                .and_modify(|e| *e += card_copies)
                .or_insert(card_copies);
        }
    }

    for (_, v) in cards.iter() {
        final_count += v;
    }

    println!("Cards = {:?}", cards);
    println!("Result = {final_count}");
}
