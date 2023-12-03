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
    let width = data.get(0).unwrap().len();
    let height = data.len();
    let mut gear_ratios = vec![0; width * height];

    for (i, line) in data.iter().enumerate() {
        let mut is_gear = false;
        let mut gear_index = 0;
        let mut current_number = String::from("");
        for (i2, ch) in line.chars().enumerate() {
            for j in 0i32..3 {
                for k in 0i32..3 {
                    let i_i32 = i as i32;
                    let i2_i32 = i2 as i32;
                    let index_i = cmp::max(0, i_i32 + j - 1) as usize;

                    let neighbour_h = data.get(index_i);
                    let neighbour_h_ok = match neighbour_h {
                        None => continue,
                        Some(i) => i,
                    };

                    let index_j = cmp::max(i2_i32 + k - 1, 0) as usize;

                    let neighbour = neighbour_h_ok.chars().nth(index_j);
                    let neighbour_ok = match neighbour {
                        None => continue,
                        Some(i) => i,
                    };

                    if neighbour_ok == '*' {
                        is_gear = true;
                        gear_index = index_i * width + index_j;
                    }
                }
            }
            if ch.is_digit(10) {
                current_number.push(ch);
                if i2 == line.len() - 1 || !line.chars().nth(i2 + 1).unwrap().is_digit(10) {
                    if !current_number.is_empty() {
                        let current_number_parsed = current_number.parse::<i32>().unwrap();
                        if is_gear {
                            println!("{}", current_number);
                            let tmp = gear_ratios[gear_index];

                            match tmp {
                                0 => gear_ratios[gear_index] = current_number_parsed,
                                -1 => continue,
                                n if n < 0 => gear_ratios[gear_index] = -1,
                                _ => gear_ratios[gear_index] *= -current_number_parsed,
                            }
                        }
                    }
                    current_number.clear();
                    is_gear = false;
                }
            } else {
                is_gear = false;
            }
        }
    }
    for ratio in gear_ratios {
        print!("{} ", ratio);
        if ratio >= -1 {
            continue;
        }
        sum += -ratio;
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
