use std::{fs, vec};

fn x_y_to_i(x: i64, y: i64, width: usize) -> usize {
    x as usize + y as usize * width
}
fn add_edge(
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    width: usize,
    height: usize,
    graph: &mut Vec<Vec<bool>>,
) {
    if 0 <= x1
        && x1 < width as i64
        && 0 <= x2
        && x2 < width as i64
        && 0 <= y1
        && y1 < height as i64
        && 0 <= y2
        && y2 < height as i64
    {
        let i1 = x_y_to_i(x1, y1, width);
        let i2 = x_y_to_i(x2, y2, width);
        graph[i1][i2] = true;
    }
}

fn neighbours(v: usize, graph: &Vec<Vec<bool>>) -> Vec<usize> {
    graph[v]
        .iter()
        .enumerate()
        .filter(|(_, x)| **x)
        .map(|(i, _)| i)
        .collect()
}
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap().to_string();
    let width = input.lines().nth(0).unwrap().chars().count();
    let height = input.lines().count();
    let n = height * width;
    let mut graph: Vec<Vec<bool>> = vec![vec![false; n]; n];
    let mut start: usize = 0;
    let parsed: String = input.chars().filter(|x| !x.is_whitespace()).collect();

    input
        .chars()
        .filter(|x| !x.is_whitespace())
        .enumerate()
        .for_each(|(i, ch)| {
            let y: i64 = (i / width).try_into().unwrap();
            let x: i64 = (i % width).try_into().unwrap();
            match ch {
                'F' => {
                    add_edge(x, y, x + 1, y, width, height, &mut graph);
                    add_edge(x, y, x, y + 1, width, height, &mut graph);
                }
                'J' => {
                    add_edge(x, y, x, y - 1, width, height, &mut graph);
                    add_edge(x, y, x - 1, y, width, height, &mut graph);
                }
                '-' => {
                    add_edge(x, y, x + 1, y, width, height, &mut graph);
                    add_edge(x, y, x - 1, y, width, height, &mut graph);
                }
                '|' => {
                    add_edge(x, y, x, y + 1, width, height, &mut graph);
                    add_edge(x, y, x, y - 1, width, height, &mut graph);
                }
                'L' => {
                    add_edge(x, y, x, y - 1, width, height, &mut graph);
                    add_edge(x, y, x + 1, y, width, height, &mut graph);
                }
                '7' => {
                    add_edge(x, y, x, y + 1, width, height, &mut graph);
                    add_edge(x, y, x - 1, y, width, height, &mut graph);
                }
                'S' => {
                    add_edge(x, y, x, y + 1, width, height, &mut graph);
                    add_edge(x, y, x, y - 1, width, height, &mut graph);
                    add_edge(x, y, x + 1, y, width, height, &mut graph);
                    add_edge(x, y, x - 1, y, width, height, &mut graph);
                    start = i
                }
                _ => {}
            }
        });
    for y in 0..n {
        for x in 0..n {
            if graph[y][x] != graph[x][y] {
                graph[y][x] = false;
                graph[x][y] = false;
            }
        }
    }

    let mut dist: Vec<usize> = vec![usize::MAX; n];
    let mut prev: Vec<i64> = vec![-1; n];
    let mut q: Vec<usize> = (0..n).collect();
    let mut s: Vec<usize> = vec![];
    dist[start] = 0;
    while !q.is_empty() {
        let out = q.iter().enumerate().min_by_key(|x| dist[*x.1]).unwrap();
        let u = q.remove(out.0);
        if dist[u] == usize::MAX {
            break;
        }
        s.push(u);
        neighbours(u, &graph).iter().for_each(|w| {
            if q.contains(w) {
                if dist[*w] > dist[u] + 1 {
                    dist[*w] = dist[u] + 1;
                    prev[*w] = u as i64;
                }
            }
        })
    }
    let answer = dist.iter().filter(|x| **x != usize::MAX).max().unwrap();

    println!("answer1 {answer}");
    let mut is_in: bool = false;
    let mut count_in = 0;

    let mut is_in_list: Vec<bool> = vec![false; n];
    let mut last = "";
    for y in 0..height {
        for x in 0..width {
            let i = &x_y_to_i(x as i64, y as i64, width);
            let curr = parsed.chars().nth(*i).unwrap();
            if s.contains(i) {
                match curr {
                    '|' => is_in = !is_in,
                    'F' | 'S' => {
                        last = "F";
                    }
                    'L' => {
                        last = "L";
                    }
                    '7' => {
                        if last == "L" {
                            is_in = !is_in;
                        }
                        last = "";
                    }
                    'J' => {
                        if last == "F" {
                            is_in = !is_in;
                        }
                        last = "";
                    }
                    _ => {}
                }
                continue;
            }
            if is_in {
                is_in_list[*i] = true;
                count_in += 1;
            }
        }
        is_in = false;
    }
    println!("answer2 {count_in}");
}
