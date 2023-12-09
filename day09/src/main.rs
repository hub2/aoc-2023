use std::fs;

fn step(list: Vec<i64>) -> Vec<i64> {
    list.windows(2).map(|x| x[1] - x[0]).collect()
}
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap().to_string();
    let lines: Vec<Vec<i64>> = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    let answer: i64 = lines
        .iter()
        .map(|x| {
            let mut tmp = x.clone();
            let mut all: Vec<Vec<i64>> = vec![tmp.clone()];
            while tmp.iter().filter(|x| **x != 0).count() != 0 {
                tmp = step(tmp);
                all.push(tmp.clone());
            }
            all.iter().map(|y| y.last().unwrap()).sum::<i64>()
        })
        .sum();
    println!("answer {answer}");

    let answer2: i64 = lines
        .iter()
        .map(|x| {
            let mut tmp = x.clone();
            let mut all: Vec<Vec<i64>> = vec![tmp.clone()];
            while tmp.iter().filter(|x| **x != 0).count() != 0 {
                tmp = step(tmp);
                all.push(tmp.clone());
            }
            all.iter()
                .rev()
                .map(|y| y.first().unwrap())
                .copied()
                .reduce(|acc, z| (z - acc))
                .unwrap()
        })
        .sum();
    println!("answer {answer2}");
}
