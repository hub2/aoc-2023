use std::fs;
use std::iter::zip;
fn vertical_one_off(input: &Vec<String>) -> i64 {
    let mut lines_above = 0i64;
    let width = input[0].len() as i64;

    let mut line_found = 0i64;
    for i in 1..width {
        line_found = i;
        let mut diff = 0;
        for j in 1..width {
            if i - j < 0 || i + j - 1 >= width {
                continue;
            }
            let left = input
                .iter()
                .map(|x| x.chars().nth((i - j).try_into().unwrap()).unwrap())
                .collect::<String>();

            let right = input
                .iter()
                .map(|x| x.chars().nth((i + j - 1).try_into().unwrap()).unwrap())
                .collect::<String>();
            diff += zip(left.chars(), right.chars())
                .filter(|(x, y)| x != y)
                .count();
        }
        if diff == 1 {
            lines_above = line_found;
            break;
        }
    }

    return lines_above;
}

fn horizontal_one_off(input: &Vec<String>) -> i64 {
    let mut lines_above = 0i64;

    let mut line_found = 0i64;
    for i in 1..input.len() {
        line_found = i as i64;
        let mut diff = 0;
        for j in 1..input.len() {
            if ((i as i64) - (j as i64) < 0) || i + j - 1 >= input.len() {
                continue;
            }
            let left = &input[i - j];
            let right = &input[i + j - 1];

            diff += zip(left.chars(), right.chars())
                .filter(|(x, y)| x != y)
                .count();
        }
        if diff == 1 {
            lines_above = line_found;
            break;
        }
    }

    return lines_above * 100;
}
fn vertical(input: &Vec<String>) -> i64 {
    let mut lines_above = 0i64;
    let width = input[0].len() as i64;

    let mut line_found = 0i64;
    for i in 1..width {
        line_found = i;
        for j in 1..width {
            if i - j < 0 || i + j - 1 >= width {
                continue;
            }
            let left = input
                .iter()
                .map(|x| x.chars().nth((i - j).try_into().unwrap()).unwrap())
                .collect::<String>();

            let right = input
                .iter()
                .map(|x| x.chars().nth((i + j - 1).try_into().unwrap()).unwrap())
                .collect::<String>();

            if left.ne(&right) {
                line_found = -1;
                break;
            }
        }
        if line_found != -1 {
            lines_above = line_found;
            break;
        }
    }

    return lines_above;
}

fn horizontal(input: &Vec<String>) -> i64 {
    let mut lines_above = 0i64;

    let mut line_found = 0i64;
    for i in 1..input.len() {
        line_found = i as i64;
        for j in 1..input.len() {
            if ((i as i64) - (j as i64) < 0) || i + j - 1 >= input.len() {
                continue;
            }

            if input[i - j].ne(&input[i + j - 1]) {
                line_found = -1;
                break;
            }
        }
        if line_found != -1 {
            lines_above = line_found;
            break;
        }
    }

    return lines_above * 100;
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap().to_string();
    let data: Vec<Vec<String>> = input
        .split("\n\n")
        .map(|x| {
            x.to_string()
                .split_whitespace()
                .map(|y| y.to_string())
                .collect()
        })
        .collect();

    let sum: i64 = data
        .iter()
        .map(|x| {
            let horizontal_val = horizontal(x);
            if horizontal_val != 0 {
                return horizontal_val;
            }
            let vertical_val = vertical(x);
            if vertical_val != 0 {
                return vertical_val;
            }
            0
        })
        .sum();

    println!("answer {sum}");
    let sum2: i64 = data
        .iter()
        .map(|x| {
            let horizontal_val = horizontal_one_off(x);
            if horizontal_val != 0 {
                return horizontal_val;
            }
            let vertical_val = vertical_one_off(x);
            if vertical_val != 0 {
                return vertical_val;
            }
            0
        })
        .sum();

    println!("answer {sum2}");
}
