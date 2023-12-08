use std::collections::HashMap;
use std::fs;
use std::ops::Index;

fn find_node(nodes: &str, node: &str) {
    let found = nodes.find(&format!("{:?}", node.split(" ").nth(0).unwrap())).unwrap();
    println!("{:?}", found);
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let instructions = lines.nth(0).unwrap().chars().map(|c| if c == 'L' { 0 } else if c == 'R' { 1 } else {panic!()}).collect::<Vec<usize>>();
    println!("{:?}", instructions);
    // let first_node = &lines.clone().skip(1).nth(0).unwrap()[0..3];
    let first_node = "AAA";
    let nodes = lines.skip(1).map(|node| (&node[0..3], vec![&node[7..10], &node[12..15]])).collect::<HashMap<_, _>>();
    // nodes.keys().filter(|node| node.ends_with("A")).for_each(|node| println!("A {:?}", node));
    let mut current_instruction = 0;
    let mut steps = 0;
    let mut current_node = first_node;
    println!("{}", first_node);
    while current_node != "ZZZ" {

        let node = nodes.get(current_node).unwrap();
        current_node = node[instructions[current_instruction]];
        current_instruction += 1;
        if current_instruction == instructions.len() {
            current_instruction = 0;
        }
        steps += 1;
    }
    println!("{}", steps);
}
