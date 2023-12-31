use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUM_REGEX: Regex = Regex::new("[0-9]*").unwrap();
}

fn is_symbol(s: &String) -> bool {
    if s == "-1" { return false; }
    let s = s.chars().collect::<Vec<char>>();
    let s = s.first().unwrap();
    !(s.is_numeric() || *s == '.')
}

fn get_adjacent(position: (isize, isize), matrix: &Vec<Vec<String>>, current_number: usize, mut adjacent_gears: &mut HashMap<(isize, isize), Vec<usize>>) -> bool {
    let binding = String::from("-1");
    let vec_binding = vec![binding.clone()];

    let curr = matrix.get(position.0 as usize).unwrap().get(position.1 as usize).unwrap();
    let left = matrix.get(position.0 as usize).unwrap_or(&vec_binding).get((position.1 - 1) as usize).unwrap_or(&binding);
    let right = matrix.get(position.0 as usize).unwrap_or(&vec_binding).get((position.1 + 1) as usize).unwrap_or(&binding);
    let top = matrix.get((position.0 - 1) as usize).unwrap_or(&vec_binding).get(position.1 as usize).unwrap_or(&binding);
    let bottom = matrix.get((position.0 + 1) as usize).unwrap_or(&vec_binding).get(position.1 as usize).unwrap_or(&binding);
    let top_left = matrix.get((position.0 - 1) as usize).unwrap_or(&vec_binding).get((position.1 - 1) as usize).unwrap_or(&binding);
    let top_right = matrix.get((position.0 - 1) as usize).unwrap_or(&vec_binding).get((position.1 + 1) as usize).unwrap_or(&binding);
    let btm_left = matrix.get((position.0 + 1) as usize).unwrap_or(&vec_binding).get((position.1 - 1) as usize).unwrap_or(&binding);
    let btm_right = matrix.get((position.0 + 1) as usize).unwrap_or(&vec_binding).get((position.1 + 1) as usize).unwrap_or(&binding);


    if left == "*" {
        adjacent_gears.entry((position.0, position.1 - 1)).or_insert(vec![]).push(current_number);
    }
    if right == "*" {
        adjacent_gears.entry((position.0, position.1 + 1)).or_insert(vec![]).push(current_number);
    }
    if top == "*" {
        adjacent_gears.entry((position.0 - 1, position.1)).or_insert(vec![]).push(current_number);
    }
    if bottom == "*" {
        adjacent_gears.entry((position.0 + 1, position.1)).or_insert(vec![]).push(current_number);
    }
    if top_left == "*" {
        adjacent_gears.entry((position.0 - 1, position.1 - 1)).or_insert(vec![]).push(current_number);
    }
    if top_right == "*" {
        adjacent_gears.entry((position.0 - 1, position.1 + 1)).or_insert(vec![]).push(current_number);
    }
    if btm_left == "*" {
        adjacent_gears.entry((position.0 + 1, position.1 - 1)).or_insert(vec![]).push(current_number);
    }
    if btm_right == "*" {
        adjacent_gears.entry((position.0 + 1, position.1 + 1)).or_insert(vec![]).push(current_number);
    }

    is_symbol(left) || is_symbol(right) || is_symbol(top) || is_symbol(bottom) || is_symbol(top_left) || is_symbol(top_right) || is_symbol(btm_left) || is_symbol(btm_right)
}

fn main() {
    let lines = fs::read_to_string("input.txt").unwrap();
    let mut matrix: Vec<Vec<String>> = vec![];

    for line in lines.lines() {
        let mut matrix_line = vec![];

        for char in line.chars() {
            matrix_line.push(char.to_string());
        }
        matrix.push(matrix_line);
    }

    let mut count = 0;
    let mut gears: HashMap<(isize, isize), Vec<usize>> = HashMap::new();
    for (i, line) in matrix.iter().enumerate() {
        let line_str = line.join("");
        for capture in NUM_REGEX.find_iter(&line_str) {
            if capture.len() == 0 { continue; };
            println!("{:?}", capture);
            let mut valid = false;
            for j in (capture.start()..capture.end()) {
                if valid { continue; }
                valid = get_adjacent((i as isize, j as isize), &matrix, usize::from_str(capture.as_str()).unwrap(), &mut gears);
            }
            println!("{} is {}", capture.as_str(), valid);
            if valid { count += i32::from_str(capture.as_str()).unwrap() }
        }
    }
    let mut count = 0;
    for (k, v) in gears.iter() {
        if v.len() == 2 { count += (v.first().unwrap() * v.last().unwrap()) }
    }
    println!("{:?}", count);
}
