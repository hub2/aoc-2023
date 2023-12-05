use std::fs;

#[derive(Debug, Clone, Copy)]
struct Mapping {
    source: i64,
    destination: i64,
    range: i64,
}

impl FromIterator<i64> for Mapping {
    fn from_iter<I: IntoIterator<Item = i64>>(iter: I) -> Self {
        let collected: Vec<i64> = iter.into_iter().collect::<Vec<i64>>();
        Mapping {
            source: collected[1],
            destination: collected[0],
            range: collected[2],
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap().to_string();

    let mut iter = input.split("\n\n");
    let seeds = iter
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mappings: Vec<Vec<Mapping>> = iter
        .map(|x| {
            x.split("\n")
                .skip(1)
                .filter(|x| !x.is_empty())
                .map(|k| {
                    k.split_whitespace()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Mapping>()
                })
                .collect()
        })
        .collect();

    let answer: i64 = seeds
        .iter()
        .map(|current| {
            let mut x = *current;
            for v in mappings.clone() {
                for i in 0..v.len() {
                    let map = v[i];
                    if x >= map.source && x < map.source + map.range {
                        x = map.destination + x - map.source;
                        break;
                    }
                }
            }
            x
        })
        .min()
        .unwrap();

    println!("{answer}");

    let answer2 = seeds
        .as_slice()
        .chunks(2)
        .map(|x| {
            let start = x[0];
            let range = x[1];
            println!("{start}");
            (start..start + range)
                .map(|current| {
                    let mut x = current;
                    for v in mappings.clone() {
                        for i in 0..v.len() {
                            let map = v[i];
                            if x >= map.source && x < map.source + map.range {
                                x = map.destination + x - map.source;
                                break;
                            }
                        }
                    }
                    x
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap();
    println!("{answer2}");
}
