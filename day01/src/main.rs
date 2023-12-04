use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let numbers: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut final_ = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(txt) = line {
                let mut found = false;
                let mut first: u32 = 0;
                let mut last: u32 = 0;
                for (i, c) in txt.chars().enumerate() {
                    for (j, number) in numbers.iter().enumerate() {
                        let subs: String =
                            txt.chars().skip(i).take(number.chars().count()).collect();
                        if subs == number.to_string() {
                            if !found {
                                found = true;
                                first = j as u32 + 1;
                            }
                            last = j as u32 + 1;
                        }
                    }
                    if c.is_digit(10) {
                        if !found {
                            first = c.to_digit(10).unwrap();
                            found = true;
                        }
                        last = c.to_digit(10).unwrap();
                    }
                }
                let control_number = first * 10 + last;
                final_ += control_number;
            }
        }
    }
    println!("{}", final_);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
