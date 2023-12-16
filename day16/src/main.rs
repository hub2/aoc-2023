use std::cmp::max;
use std::fs;
fn beam(
    data: &Vec<Vec<char>>,
    current: (i32, i32),
    direction: (i32, i32),
    visited: &mut Vec<Vec<(i32, i32, i32, i32)>>,
) {
    if !(current.0 >= 0
        && current.1 >= 0
        && (current.0 as usize) < data[0].len()
        && (current.1 as usize) < data.len())
    {
        return;
    }

    let current_x = current.0 as usize;
    let current_y = current.1 as usize;
    match visited[current_y][current_x] {
        (1, _, _, _) => {
            if direction.0 == -1 {
                return;
            }
        }
        (_, 1, _, _) => {
            if direction.0 == 1 {
                return;
            }
        }
        (_, _, 1, _) => {
            if direction.1 == -1 {
                return;
            }
        }
        (_, _, _, 1) => {
            if direction.1 == 1 {
                return;
            }
        }
        _ => {}
    }
    match direction {
        (-1, 0) => visited[current_y][current_x].0 += 1,
        (1, 0) => visited[current_y][current_x].1 += 1,
        (0, -1) => visited[current_y][current_x].2 += 1,
        (0, 1) => visited[current_y][current_x].3 += 1,
        _ => {}
    }

    let direction_x = direction.0;
    let direction_y = direction.1;

    match data[current_y][current_x] {
        '.' => beam(
            data,
            (current.0 + direction_x, current.1 + direction_y),
            direction,
            visited,
        ),
        '|' => {
            if direction_x != 0 {
                beam(data, (current.0, current.1 - 1), (0, -1), visited);
                beam(data, (current.0, current.1 + 1), (0, 1), visited)
            } else {
                beam(
                    data,
                    (current.0 + direction_x, current.1 + direction_y),
                    direction,
                    visited,
                )
            }
        }
        '-' => {
            if direction_y != 0 {
                beam(data, (current.0 - 1, current.1), (-1, 0), visited);
                beam(data, (current.0 + 1, current.1), (1, 0), visited)
            } else {
                beam(
                    data,
                    (current.0 + direction_x, current.1 + direction_y),
                    direction,
                    visited,
                )
            }
        }
        '\\' => {
            if direction_y != 0 {
                beam(
                    data,
                    (current.0 + direction_y, current.1),
                    (direction_y, 0),
                    visited,
                )
            } else {
                beam(
                    data,
                    (current.0, current.1 + direction_x),
                    (0, direction_x),
                    visited,
                )
            }
        }
        '/' => {
            if direction_y != 0 {
                beam(
                    data,
                    (current.0 - direction_y, current.1),
                    (-direction_y, 0),
                    visited,
                )
            } else {
                beam(
                    data,
                    (current.0, current.1 - direction_x),
                    (0, -direction_x),
                    visited,
                )
            }
        }
        _ => {}
    }
}
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap().to_string();
    let data: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let width = data[0].len();
    let height = data.len();
    let mut visited = vec![vec![(0, 0, 0, 0); width]; height];

    beam(&data, (0, 0), (1, 0), &mut visited);
    let answer = visited
        .iter()
        .flatten()
        .filter(|x| x.0 > 0 || x.1 > 0 || x.2 > 0 || x.3 > 0)
        .count();
    //dbg!(visited);

    let mut max_val = 0;

    let mut visited2 = vec![vec![(0, 0, 0, 0); width]; height];
    for x in 0..width {
        visited2 = vec![vec![(0, 0, 0, 0); width]; height];

        beam(&data, (x as i32, 0), (0, 1), &mut visited2);
        max_val = max(
            visited2
                .iter()
                .flatten()
                .filter(|x| x.0 > 0 || x.1 > 0 || x.2 > 0 || x.3 > 0)
                .count(),
            max_val,
        );
    }

    for x in 0..width {
        visited2 = vec![vec![(0, 0, 0, 0); width]; height];
        beam(
            &data,
            (x as i32, (height - 1) as i32),
            (0, -1),
            &mut visited2,
        );
        max_val = max(
            visited2
                .iter()
                .flatten()
                .filter(|x| x.0 > 0 || x.1 > 0 || x.2 > 0 || x.3 > 0)
                .count(),
            max_val,
        );
    }

    for y in 0..height {
        visited2 = vec![vec![(0, 0, 0, 0); width]; height];
        beam(&data, (0, y as i32), (1, 0), &mut visited2);
        max_val = max(
            visited2
                .iter()
                .flatten()
                .filter(|x| x.0 > 0 || x.1 > 0 || x.2 > 0 || x.3 > 0)
                .count(),
            max_val,
        );
    }

    for y in 0..height {
        visited2 = vec![vec![(0, 0, 0, 0); width]; height];
        beam(
            &data,
            ((width - 1) as i32, y as i32),
            (-1, 0),
            &mut visited2,
        );
        max_val = max(
            visited2
                .iter()
                .flatten()
                .filter(|x| x.0 > 0 || x.1 > 0 || x.2 > 0 || x.3 > 0)
                .count(),
            max_val,
        );
    }
    println!("answer {answer}");
    println!("answer2 {max_val}");
}
