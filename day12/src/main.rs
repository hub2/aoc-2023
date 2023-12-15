use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;

use std::sync::Mutex;

lazy_static! {
    static ref CACHE: Mutex<HashMap<(Vec<char>, Vec<usize>), i64>> = {
        let map = HashMap::new();
        Mutex::new(map)
    };
}
fn analyze(pattern: &[char], lens: &[usize]) -> i64 {
    let mut sum: i64 = 0;
    let key = (pattern.to_vec(), lens.to_vec());
    {
        let map = CACHE.lock().unwrap();
        if map.contains_key(&key) {
            return *map.get(&key).unwrap();
        }
    }

    if lens.is_empty() {
        if let Some(x) = pattern.iter().position(|x| *x == '#') {
            return 0;
        } else {
            return 1;
        }
    }

    let current_lens = lens[0];
    if pattern.len() < current_lens
        || pattern.is_empty()
        || pattern.len() < lens.iter().sum()
        || lens.len() > pattern.split(|x| *x == '?' || *x == '.').count()
    {
        return 0;
    }

    let can_fit_lens = pattern
        .iter()
        .take(current_lens)
        .filter(|x| **x != '?' && **x != '#')
        .count()
        == 0;

    if can_fit_lens {
        if pattern.len() > current_lens as usize
            && (pattern[current_lens] == '?' || pattern[current_lens] == '.')
        {
            sum += analyze(&pattern[current_lens + 1..], &lens[1..]);
        }
        if pattern.len() == current_lens as usize {
            sum += analyze(&[], &lens[1..]);
        }
    }
    if *pattern.get(0).unwrap() != '#' {
        sum += analyze(&pattern[1..], lens);
    }
    {
        let mut map = CACHE.lock().unwrap();
        map.insert(key, sum);
    }
    sum
}
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap().to_string();
    let sum: i64 = input
        .lines()
        .map(|x| {
            let mut i = x.split_whitespace();
            let left = i.nth(0).unwrap().chars().collect::<Vec<char>>();
            let left_slice = left.as_slice();
            let right = i
                .nth(0)
                .unwrap()
                .split(",")
                .map(|y| y.parse().unwrap())
                .collect::<Vec<usize>>();
            analyze(&left_slice, &right)
        })
        .sum();

    println!("answer {sum}");
    let sum2: i64 = input
        .lines()
        .map(|x| {
            let mut i = x.split_whitespace();
            let left: Vec<char> = i.nth(0).unwrap().chars().collect::<Vec<char>>();
            let mut new_left = vec![];
            new_left.extend(left.clone());
            for k in 0..4 {
                new_left.extend(vec!['?']);
                new_left.extend(left.clone());
            }

            let right: Vec<usize> = i
                .nth(0)
                .unwrap()
                .split(",")
                .map(|y| y.parse().unwrap())
                .collect::<Vec<usize>>();

            let mut new_right = vec![];
            for k in 0..5 {
                new_right.extend(right.clone());
            }
            println!("{}", x);
            analyze(&new_left, &new_right)
        })
        .sum();

    println!("answer2 {sum2}");
}
