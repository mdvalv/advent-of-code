use std::{collections::HashMap, ops::Index};

use aoc_utils::init_challenge;

#[derive(Debug, PartialEq, PartialOrd, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    hand_type: HandType,
    bid: usize,
}

fn main() {
    let input = init_challenge();
    let mut hands: Vec<Hand> = Vec::new();

    for line in input.lines() {
        let (cards, bid) = line.split_once(' ').unwrap();
        hands.push(Hand {
            cards: cards.chars().collect(),
            hand_type: get_hand_type(cards),
            bid: bid.parse().unwrap(),
        })
    }

    hands.sort_by(|a, b| compare_hands(b, a));

    let mut count = 0;
    for (i, hand) in hands.iter().enumerate() {
        count += hand.bid * (i + 1);
        println!("{:?} {:?}", hand.cards, hand);
    }

    println!("Result = {count}");
}

fn compare_hands(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    if a.hand_type < b.hand_type {
        return std::cmp::Ordering::Less;
    }
    if a.hand_type > b.hand_type {
        return std::cmp::Ordering::Greater;
    }

    for i in 0..a.cards.len() {
        let res = compare_cards(*b.cards.index(i), *a.cards.index(i));
        if res != std::cmp::Ordering::Equal {
            return res;
        }
    }

    return std::cmp::Ordering::Equal;
}

fn get_hand_type(cards: &str) -> HandType {
    let mut hand: HashMap<char, usize> = HashMap::new();

    for card in cards.chars() {
        *hand.entry(card).or_insert(0) += 1;
    }

    if hand.len() == 1 {
        return HandType::FiveOfAKind;
    } else if hand.len() == 2 {
        if hand.get_key_value(&'J').is_some() {
            return HandType::FiveOfAKind;
        }
        if hand.values().any(|&x| x == 1 || x == 4) {
            return HandType::FourOfAKind;
        }
        return HandType::FullHouse;
    } else if hand.len() == 3 {
        if hand.get_key_value(&'J').is_some() {
            match hand.get_key_value(&'J').unwrap().1 {
                1 => {
                    if hand.values().any(|&x| x == 2) {
                        return HandType::FullHouse;
                    }
                    return HandType::FourOfAKind;
                }
                2 | 3 => return HandType::FourOfAKind,
                _ => panic!("Unexpected value"),
            }
        }
        if hand.values().any(|&x| x == 3) {
            return HandType::ThreeOfAKind;
        }
        return HandType::TwoPair;
    } else if hand.len() == 4 {
        if hand.get_key_value(&'J').is_some() {
            return HandType::ThreeOfAKind;
        }

        return HandType::OnePair;
    }

    if hand.get_key_value(&'J').is_some() {
        return HandType::OnePair;
    }

    return HandType::HighCard;
}

fn compare_cards(a: char, b: char) -> std::cmp::Ordering {
    let cards: HashMap<char, usize> = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
    ]);

    if cards[&a] > cards[&b] {
        return std::cmp::Ordering::Greater;
    }
    if cards[&a] < cards[&b] {
        return std::cmp::Ordering::Less;
    }
    return std::cmp::Ordering::Equal;
}
