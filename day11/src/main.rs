use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap().to_string();
    let mut empty_rows = vec![];
    let mut empty_cols = vec![];
    let data: Vec<Vec<bool>> = input
        .lines()
        .map(|x| x.chars().map(|y| y == '#').collect())
        .collect();
    dbg!(data.clone());
    let height = data.len();
    let width = data.get(0).unwrap().len();
    data.iter().enumerate().for_each(|(idx, row)| {
        if row.iter().filter(|x| **x).count() == 0 {
            empty_rows.push(idx)
        }
    });

    for x in 0..width {
        if (0..height).into_iter().filter(|y| data[*y][x]).count() == 0 {
            empty_cols.push(x);
        }
    }
    let mut galaxies = vec![];
    for y in 0..height {
        for x in 0..width {
            if data[y][x] {
                galaxies.push((y, x));
            }
        }
    }
    let mut sum = 0;
    for galaxy in &galaxies {
        for galaxy2 in &galaxies {
            let y_range = galaxy.0..galaxy2.0;
            let x_range = galaxy.1..galaxy2.1;

            let x_dist = x_range
                .into_iter()
                .map(|x| {
                    if empty_cols.contains(&x) {
                        return 1000000;
                    }
                    1
                })
                .sum::<i64>();

            let y_dist = y_range
                .into_iter()
                .map(|x| {
                    if empty_rows.contains(&x) {
                        return 1000000;
                    }
                    1
                })
                .sum::<i64>();

            sum += x_dist + y_dist;
        }
    }
    println!("answer {sum}");
}
