use std::collections::HashMap;
use std::fs;
fn hash(data: String) -> i64 {
    let mut current_value = 0i64;
    data.chars().for_each(|x| {
        current_value += x as i64;
        current_value *= 17;
        current_value %= 256;
    });
    current_value
}
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap().to_string();
    let data: Vec<String> = input
        .split(",")
        .map(|x| x.to_string().trim().chars().collect::<String>())
        .collect();
    let answer: i64 = data.iter().map(|x| hash(x.to_string())).sum();
    println!("answer {answer}");

    let mut hm: HashMap<i64, Vec<(String, i64)>> = HashMap::new();
    data.iter().for_each(|x| {
        if x.contains("=") {
            let mut new_entry = x.split("=");
            let key = new_entry.nth(0).unwrap();
            let box_nr = hash(key.to_string());
            let val: i64 = new_entry.nth(0).unwrap().to_string().parse().unwrap();

            let entry = hm.entry(box_nr).or_insert(Vec::new());
            let exists = entry.iter().position(|r| r.0 == key);
            if let Some(x) = exists {
                entry[x].1 = val;
            } else {
                entry.push((key.to_string(), val));
            }
        }
        if x.contains("-") {
            let mut new_entry = x.split("-");
            let key = new_entry.nth(0).unwrap();

            let box_nr = hash(key.to_string());
            if hm.contains_key(&box_nr) {
                let vec_data = hm.get_mut(&box_nr).unwrap();
                let exists = vec_data.iter().position(|r| r.0 == key);

                if let Some(x) = exists {
                    vec_data.remove(x);
                    if vec_data.len() == 0 {
                        hm.remove(&box_nr);
                    }
                }
            }
        }
    });
    let answer2: i64 = hm
        .iter()
        .map(|(k, v)| {
            v.iter()
                .enumerate()
                .map(|(i, (x, y))| (1 + k) * (i + 1) as i64 * y)
                .sum::<i64>()
        })
        .sum();

    println!("answer2 {answer2}");
}
