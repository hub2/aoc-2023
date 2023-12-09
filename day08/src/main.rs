use num::integer::lcm;
use std::collections::HashMap;
use std::{fs, vec};

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn navigate_once(node: String, hm: &HashMap<String, Node>, direction: char) -> String {
    match direction {
        'R' => hm.get(&node).unwrap().right.to_owned(),
        'L' => hm.get(&node).unwrap().left.to_owned(),
        _ => panic!("what"),
    }
}
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap().to_string();
    let mut lines = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string());
    let directions: String = lines.nth(0).unwrap().trim().to_string();
    dbg!(directions.clone());

    let mut hm: HashMap<String, Node> = HashMap::new();

    lines.for_each(|x| {
        let splitted: Vec<String> = x.split(" = ").map(|x| x.to_string()).collect();
        let node = splitted.get(0).unwrap().to_string();
        dbg!(node.to_string());
        let cleaned = splitted.get(1).unwrap().replace("(", "").replace(")", "");
        let dest: Vec<String> = cleaned.split(",").map(|x| x.to_string()).collect();
        let left = dest.get(0).unwrap().trim().to_string();
        let right = dest.get(1).unwrap().trim().to_string();
        hm.insert(node, Node { left, right });
    });

    //let mut i = 0;
    //let mut current_node: &str = &"AAA";
    //while current_node != "ZZZ" {
    //    dbg!(current_node);
    //    match directions
    //        .chars()
    //        .nth(i % directions.chars().count())
    //        .unwrap()
    //    {
    //        'R' => current_node = &hm.get(current_node).unwrap().right,
    //        'L' => current_node = &hm.get(current_node).unwrap().left,
    //        _ => panic!("what"),
    //    }
    //    i += 1;
    //}

    //println!("answer: {i}");

    let current_nodes: Vec<String> = hm
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| x.to_string())
        .collect();
    dbg!(current_nodes.clone());
    let mut each: Vec<usize> = vec![];
    for current_node in current_nodes {
        let mut found = false;
        let mut i: usize = 0;
        let mut tmp_node = current_node.clone();
        while !found {
            let direction = directions
                .chars()
                .nth(i % directions.chars().count())
                .unwrap();

            i += 1;
            tmp_node = navigate_once(tmp_node, &hm, direction);
            found = tmp_node.ends_with('Z');
        }
        each.push(i);
    }
    let out: usize = each.into_iter().reduce(|x, y| lcm(x, y)).unwrap();
    println!("answer: {out}");
}
