use std::fs;
use std::iter::zip;
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap().to_string();
    let lines = input.lines();
    let times: Vec<i32> = lines
        .clone()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<i32> = lines
        .clone()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let answer: i64 = zip(times, distances)
        .map(|(t, d)| {
            let mut counter = 0;
            for speed in 1..t + 1 {
                let dist = speed * (t - speed);
                if dist > d {
                    counter += 1;
                }
            }
            counter
        })
        .product();
    println!("{answer}");
    let input = fs::read_to_string("./input2.txt").unwrap().to_string();
    let lines = input.lines();
    let times: Vec<i64> = lines
        .clone()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<i64> = lines
        .clone()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let answer2: i64 = zip(times, distances)
        .map(|(t, d)| {
            let mut counter = 0;
            for speed in 1..t + 1 {
                let dist = speed * (t - speed);
                if dist > d {
                    counter += 1;
                }
            }
            counter
        })
        .sum();
    println!("{answer2}");
}
