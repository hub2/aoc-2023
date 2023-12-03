use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input.txt").unwrap();
    let mut data: Vec<String> = Vec::new();

    for line_opt in lines {
        let line = line_opt.unwrap();
        data.push(line);
    }

    let mut sum = 0;

    for (i, line) in data.iter().enumerate() {
        let mut is_current_relevant = false;
        let mut current_number = String::from("");
        for (i2, ch) in line.chars().enumerate() {
            for j in 0i32..3 {
                for k in 0i32..3 {
                    let i_i32 = i as i32;
                    let i2_i32 = i2 as i32;

                    let neighbour_h = data.get(cmp::max(0, i_i32 + j - 1) as usize);
                    let neighbour_h_ok = match neighbour_h {
                        None => continue,
                        Some(i) => i,
                    };

                    let neighbour = neighbour_h_ok
                        .chars()
                        .nth(cmp::max(i2_i32 + k - 1, 0) as usize);
                    let neighbour_ok = match neighbour {
                        None => continue,
                        Some(i) => i,
                    };

                    if !neighbour_ok.is_digit(10) && neighbour_ok != '.' {
                        is_current_relevant = true;
                    }
                }
            }
            if ch.is_digit(10) {
                current_number.push(ch);
                if i2 == line.len() - 1 || !line.chars().nth(i2 + 1).unwrap().is_digit(10) {
                    if !current_number.is_empty() {
                        let current_number_parsed = current_number.parse::<i32>().unwrap();
                        if is_current_relevant {
                            println!("{}", current_number);
                            sum += current_number_parsed;
                        }
                    }
                    current_number.clear();
                    is_current_relevant = false;
                }
            } else {
                is_current_relevant = false;
            }
        }
    }
    print!("{}\n", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
