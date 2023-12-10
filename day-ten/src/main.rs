use std::cmp::min;
use std::collections::HashMap;
use std::{fs, thread};

type Matrix = Vec<Vec<char>>;
type Coordinate = (usize, usize);
fn find_start(matrix: &Matrix) -> Coordinate {
    for (y, row) in matrix.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val == 'S' {
                return (y, x);
            }
        }
    }
    panic!();
}

fn get_paths(position: Coordinate, matrix: &Matrix, mut visited: &mut Vec<Coordinate>, mut steps: &mut HashMap<Coordinate, usize>, count: usize) {
    let binding = 'X';
    let vec_binding = vec![binding.clone()];

    if visited.contains(&position) {
        return;
    }

    visited.push(position);

    let curr = matrix.get(position.0 as usize);
    if curr.is_none() {return};
    let curr = curr.unwrap().get(position.1 as usize);
    if curr.is_none() {return};
    let curr = *curr.unwrap();
    let mut west = (position.0, (position.1.checked_sub(1)));
    let east = (position.0, (position.1 + 1));
    let mut north = ((position.0.checked_sub(1)), position.1);
    let south = ((position.0 + 1), position.1);


    // Ground
    if curr == '.' {
        return
    }
    steps.entry(position).or_insert(count);
    steps.entry(position).and_modify(|c| *c = min(*c, count));
    if curr == 'S' && visited.len() == 1 {
        let mut a = Vec::new();
        if north.0.is_some() {
            get_paths((north.0.unwrap(), north.1), &matrix, &mut a, &mut steps,count+1);
        }
        // get_paths(south, &matrix, &mut a, &mut steps,count+1);
        // if west.1.is_some() {
        //     get_paths((west.0, west.1.unwrap()), &matrix, &mut visited, &mut steps,count+1);
        // }
        let mut b = Vec::new();
        get_paths(east, &matrix, &mut b, &mut steps,count+1);
    }

    // North and South
    if curr == '|' {
        if north.0.is_some() {
            get_paths((north.0.unwrap(), north.1), &matrix, &mut visited, &mut steps,count+1);
        }
        get_paths(south, &matrix, &mut visited, &mut steps,count+1);
    }

    // East and West
    if curr == '-' {
        if west.1.is_some() {
            get_paths((west.0, west.1.unwrap()), &matrix, &mut visited, &mut steps,count+1);
        }
        get_paths(east, &matrix, &mut visited, &mut steps,count+1);

    }

    // North and East
    if curr == 'L' {
        if north.0.is_some() {
            get_paths((north.0.unwrap(), north.1), &matrix, &mut visited, &mut steps,count+1);
        }
        get_paths(east, &matrix, &mut visited, &mut steps,count+1);
    }

    // North and West
    if curr == 'J' {
        if north.0.is_some() {
            get_paths((north.0.unwrap(), north.1), &matrix, &mut visited, &mut steps,count+1);
        }
        if west.1.is_some() {
            get_paths((west.0, west.1.unwrap()), &matrix, &mut visited, &mut steps,count+1);
        }
    }

    // South and West
    if curr == '7' {
        if west.1.is_some() {
            get_paths((west.0, west.1.unwrap()), &matrix, &mut visited, &mut steps,count+1);
        }
        get_paths(south, &matrix, &mut visited, &mut steps,count+1);

    }

    // South and East
    if curr == 'F' {
        get_paths(south, &matrix, &mut visited, &mut steps,count+1);
        get_paths(east, &matrix, &mut visited, &mut steps,count+1);
    }
}

fn main() {
    let t = thread::Builder::new().stack_size(100_000 as usize * 0xFF).spawn(move || {
        let input = fs::read_to_string("input.txt").unwrap();
        let mut matrix = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Matrix>();
        let map = &mut HashMap::new();
        let mut matrix_copy = matrix.clone().iter().map(|c| c.iter().map(|cc| String::from(*cc)).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();

        get_paths(find_start(&matrix), &matrix, &mut Vec::new(), map, 0);
        // println!("{:?}", map);
        for (y, row) in matrix.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if map.contains_key(&(y, x)) {
                    matrix_copy[y][x] = "X".parse().unwrap();
                    // matrix_copy[y][x] = (*map.get(&(y, x)).unwrap().to_string()).parse().unwrap();
                }
            }
        }

        let matrix_copy = matrix_copy.iter().map(|c| c.iter().map(|c| String::from(c)).collect::<String>()).collect::<Vec<String>>().join("\n");
        let mut vals = map.values().collect::<Vec<&usize>>();
        println!("{:?}", matrix_copy);
        vals.sort();
        println!("{:?}", vals.iter().last());

    }).expect("TODO: panic message");
    t.join();
}