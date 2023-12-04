use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;
fn main() {
    let lines = read_lines("./input.txt").unwrap();
    let cards: Vec<String> = lines
        .map(|x| x.unwrap().split(": ").nth(1).unwrap().into())
        .collect();

    let mut copies = vec![1i32; cards.len()];

    let winning_numbers: Vec<Vec<i32>> = cards
        .iter()
        .map(|x| {
            x.split(" | ")
                .nth(0)
                .unwrap()
                .split_whitespace()
                .map(|k| k.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let drawn_numbers: Vec<Vec<i32>> = cards
        .iter()
        .map(|x| {
            x.split(" | ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|k| k.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let answer: i32 = zip(drawn_numbers.clone(), winning_numbers.clone())
        .map(|(x, y)| {
            let power: i32 = x
                .iter()
                .filter(|k| y.contains(k))
                .count()
                .try_into()
                .unwrap();
            if power > 0 {
                2i32.pow((power - 1).try_into().unwrap())
            } else {
                0
            }
        })
        .sum();

    print!("{}\n", answer);

    let answer2: i32 = zip(drawn_numbers.clone(), winning_numbers.clone())
        .enumerate()
        .map(|(i, (x, y))| {
            let power: usize = x.iter().filter(|k| y.contains(k)).count();
            let current_copies = copies[i];
            copies
                .iter_mut()
                .skip(i + 1)
                .take(power.try_into().unwrap())
                .for_each(|x| *x += current_copies);
            copies[i]
        })
        .sum();
    print!("{}\n", answer2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
