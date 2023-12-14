use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};

use std::iter::zip;

fn north(data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = data[0].len();
    let height = data.len();
    let mut out = vec![vec!['.'; width]; height];

    for x in 0..width {
        for y in 0..height {
            match data[y][x] {
                '#' => {
                    out[y][x] = '#';
                }
                'O' => {
                    for j in 1..height + 1 {
                        if (y as i64) - (j as i64) < 0 {
                            out[0][x] = 'O';
                            break;
                        }
                        if out[y - j][x] != '.' {
                            out[y - j + 1][x] = 'O';
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    out
}
fn south(data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = data[0].len();
    let height = data.len();
    let mut out = vec![vec!['.'; width]; height];

    for x in 0..width {
        for y in (0..height).rev() {
            match data[y][x] {
                '#' => {
                    out[y][x] = '#';
                }
                'O' => {
                    for j in 1..height {
                        if y + j >= height {
                            out[height - 1][x] = 'O';
                            break;
                        }
                        if out[y + j][x] != '.' {
                            out[y + j - 1][x] = 'O';
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    out
}

fn east(data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = data[0].len();
    let height = data.len();
    let mut out = vec![vec!['.'; width]; height];

    for y in 0..height {
        for x in (0..width).rev() {
            match data[y][x] {
                '#' => {
                    out[y][x] = '#';
                }
                'O' => {
                    for j in 1..width {
                        if x + j >= width {
                            out[y][width - 1] = 'O';
                            break;
                        }
                        if out[y][x + j] != '.' {
                            out[y][x + j - 1] = 'O';
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    out
}

fn west(data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = data[0].len();
    let height = data.len();
    let mut out = vec![vec!['.'; width]; height];

    for y in 0..height {
        for x in 0..width {
            match data[y][x] {
                '#' => {
                    out[y][x] = '#';
                }
                'O' => {
                    for j in 1..width + 1 {
                        if (x as i64) - (j as i64) < 0 {
                            out[y][0] = 'O';
                            break;
                        }
                        if out[y][x - j] != '.' {
                            out[y][x - j + 1] = 'O';
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    out
}
fn calc(data: &Vec<Vec<char>>) -> usize {
    let width = data[0].len();
    let height = data.len();

    let mut acc = 0;
    for x in 0..width {
        let mut stones_count = 0;
        let mut last_row = 0;
        for y in 0..height {
            match data[y][x] {
                '#' => {
                    acc += (height - last_row - stones_count + 1..height - last_row + 1)
                        .sum::<usize>();
                    last_row = y + 1;
                    stones_count = 0;
                }
                'O' => {
                    stones_count += 1;
                }
                _ => {}
            }
        }
        acc += (height - last_row - stones_count + 1..height - last_row + 1).sum::<usize>();
    }

    acc
}
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap().to_string();
    let data: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.to_string().trim().chars().collect::<Vec<char>>())
        .collect();

    let width = data[0].len();
    let height = data.len();
    let acc = calc(&data);

    println!("answer {acc}");

    let mut step = data;
    let mut prev: HashMap<String, Vec<i64>> = HashMap::new();

    for i in 1..1000000000 {
        step = north(&step);
        //step.clone()
        //    .iter()
        //    .for_each(|x| println!("{}", x.iter().collect::<String>()));
        //println!("");
        step = west(&step);
        //step.clone()
        //    .iter()
        //    .for_each(|x| println!("{}", x.iter().collect::<String>()));
        //println!("");
        step = south(&step);
        //step.clone()
        //    .iter()
        //    .for_each(|x| println!("{}", x.iter().collect::<String>()));
        //println!("");
        step = east(&step);

        if prev.contains_key(&step.iter().flatten().collect::<String>()) {
            let cycle: Vec<i64> = prev
                .get(&step.iter().flatten().collect::<String>())
                .unwrap()
                .to_vec();
            dbg!(cycle.clone());
            if cycle.len() > 5 {
                let diff = cycle[3] - cycle[2];
                if (1000000000 - cycle[2]) % diff == 0 {
                    println!("Found it ! {i}");
                    break;
                }
            }
        }
        //step.iter().flatten().collect::<String>().hash(&mut s);
        //out = s.finish();
        prev.entry(step.iter().flatten().collect::<String>())
            .or_insert(Vec::new())
            .push(i);

        if i % 1 == 0 {
            println!("wow {i}");
            step.clone()
                .iter()
                .for_each(|x| println!("{}", x.iter().collect::<String>()));
            println!("");
        }
    }
    //let acc2 = calc(&step);
    let acc2: usize = step
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let row_val = height - i;
            x.iter()
                .map(|y| match y {
                    'O' => row_val,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum();

    println!("answer2 {acc2}");
}
