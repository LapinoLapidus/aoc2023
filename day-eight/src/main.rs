use std::collections::HashMap;
use std::fs;
use std::ops::Index;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn find_node(nodes: &str, node: &str) {
    let found = nodes.find(&format!("{:?}", node.split(" ").nth(0).unwrap())).unwrap();
    println!("{:?}", found);
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let instructions = lines.nth(0).unwrap().chars().map(|c| if c == 'L' { 0 } else if c == 'R' { 1 } else {panic!()}).collect::<Vec<usize>>();
    let first_node = "AAA";
    let nodes = lines.skip(1).map(|node| (&node[0..3], vec![&node[7..10], &node[12..15]])).collect::<HashMap<_, _>>();
    let mut starting_nodes = nodes.keys().filter(|node| node.ends_with("A")).collect::<Vec<&&str>>();
    let mut current_instruction = 0;
    let mut steps = 0;
    let mut mults = vec![];
    while !starting_nodes.iter().all(|node| node.ends_with("Z")) {
        let mut to_remove = vec![];
        for i in 0..starting_nodes.len() {
            starting_nodes[i] = &nodes.get(starting_nodes[i]).unwrap()[instructions[current_instruction]];
        }
        current_instruction += 1;
        if current_instruction == instructions.len() {
            current_instruction = 0;
        }
        for i in to_remove {
            starting_nodes.remove(i);
        }
        steps += 1;
    }
    println!("{}", mults.iter().product::<usize>());
}
