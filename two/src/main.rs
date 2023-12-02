use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    const LIMIT_BLUE: i32 = 14;
    const LIMIT_GREEN: i32 = 13;
    const LIMIT_RED: i32 = 12;
    let mut ids_sum = 0;
    let mut power_sum = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for (i, line) in lines.enumerate() {
            if let Ok(txt) = line {
                let first: String = txt.split(": ").skip(1).take(1).collect::<String>();
                let info = first
                    .split("; ")
                    .map(|x| x.split(", ").collect())
                    .collect::<Vec<Vec<&str>>>();
                let mut max_blue = 0;
                let mut max_green = 0;
                let mut max_red = 0;

                for grab in info {
                    for colour in grab {
                        let mut colour_iter = colour.split(" ");
                        let number = colour_iter.next().unwrap().parse::<i32>().unwrap();
                        let colour = colour_iter.next().unwrap();
                        match colour {
                            "blue" => max_blue = cmp::max(max_blue, number),
                            "green" => max_green = cmp::max(max_green, number),
                            "red" => max_red = cmp::max(max_red, number),
                            _ => panic!("{}", colour),
                        }
                    }
                }
                power_sum += max_blue * max_green * max_red;
                if max_blue <= LIMIT_BLUE && max_green <= LIMIT_GREEN && max_red <= LIMIT_RED {
                    ids_sum += i + 1;
                }
            }
        }
    }
    println!("first: {}", ids_sum);
    println!("second: {}", power_sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
