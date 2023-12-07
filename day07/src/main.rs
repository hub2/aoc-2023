use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::iter::zip;

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Copy, Clone)]
enum Hand {
    Five = 7,
    Four = 6,
    FullHouse = 5,
    Three = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn evaluate_joker_hand(hand: String) -> Hand {
    let non_joker_cards: String = hand.chars().into_iter().filter(|x| *x != 'J').collect();
    let joker_count = hand.chars().count() - non_joker_cards.chars().count();

    let cards: Vec<char> = vec![
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    cards
        .into_iter()
        .combinations_with_replacement(joker_count)
        .map(|x| {
            let gen_cards = x.iter().collect::<String>();
            let new_cards = gen_cards + &non_joker_cards;
            evaluate_hand(new_cards)
        })
        .max()
        .unwrap()
}

fn evaluate_hand(hand: String) -> Hand {
    let mut hm = HashMap::new();
    for c in hand.chars() {
        hm.entry(c).and_modify(|entry| *entry += 1).or_insert(1);
    }

    for (_entry, count) in &hm {
        if *count == 5 {
            return Hand::Five;
        }
        if *count == 4 {
            return Hand::Four;
        }
    }

    if hm.len() == 2 {
        return Hand::FullHouse;
    }

    if *hm.values().max().unwrap() == 3 {
        return Hand::Three;
    }

    if hm.values().filter(|x| **x == 2).count() == 2 {
        return Hand::TwoPair;
    }

    if hm.values().filter(|x| **x == 2).count() == 1 {
        return Hand::OnePair;
    }

    return Hand::HighCard;
}

fn cmp_card(x: char, y: char) -> Ordering {
    let mut cards: Vec<char> = vec![
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];
    cards.reverse();
    return cards
        .iter()
        .position(|&r| r == x)
        .unwrap()
        .cmp(&cards.iter().position(|&r| r == y).unwrap());
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap().to_string();
    let lines = input.lines();
    let mut data: Vec<(Hand, String, i64)> = lines
        .map(|x| {
            let mut splitted = x.split_whitespace();
            let hand_string = splitted.nth(0).unwrap().to_string();
            (
                evaluate_joker_hand(hand_string.clone()),
                hand_string,
                splitted.nth(0).unwrap().parse().unwrap(),
            )
        })
        .collect();

    data.sort_by(|x, y| {
        x.0.cmp(&y.0).then_with(|| {
            zip(x.1.chars(), y.1.chars()).fold(Ordering::Equal, |acc, f| {
                acc.then_with(|| cmp_card(f.0, f.1))
            })
        })
    });

    let answer: i64 = data
        .iter()
        .enumerate()
        .map(|(i, x)| (i + 1) as i64 * x.2)
        .sum();
    println!("{answer}");
}
