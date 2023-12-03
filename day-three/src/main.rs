use std::fs;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUM_REGEX: Regex = Regex::new("[0-9]*").unwrap();
}

fn is_symbol(s: &String) -> bool {
    if s == "-1" {return false}
    let s = s.chars().collect::<Vec<char>>();
    let s = s.first().unwrap();
    let ret = !(s.is_numeric() || *s == '.');
    5+5;
    return ret
}

fn get_adjacent(position: (isize, isize), matrix: &Vec<Vec<String>>) -> bool {
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

    // println!("{}: {} {} {} {} {} {} {} {}", curr, left, right, top, bottom, top_left, top_right, btm_left, btm_right);
    // println!("{}", is_symbol(left) || is_symbol(right) || is_symbol(top) || is_symbol(bottom) || is_symbol(top_left) || is_symbol(top_right) || is_symbol(btm_left) || is_symbol(btm_right));
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

    for (i, line) in matrix.iter().enumerate() {
        let line_str = line.join("");
        for capture in NUM_REGEX.find_iter(&line_str) {
            if capture.len() == 0 {continue};
            println!("{:?}", capture);
            let mut valid = false;
            for j in (capture.start()..capture.end()) {
                if valid {continue}
                valid = get_adjacent((i as isize, j as isize), &matrix);
            }
            println!("{} is {}", capture.as_str(), valid);
            if valid {count += i32::from_str(capture.as_str()).unwrap()}
        }
    }
    println!("{:?}", count);
}
