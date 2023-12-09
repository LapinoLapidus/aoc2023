use std::fs;

fn generate_subrows<'a>(line: &'a Vec<isize>, mut adder: &mut Vec<Vec<isize>>) {
    let mut inner_adder = vec![];
    let binding = adder.clone();
    let line = binding.last().clone().unwrap();
    for i in (0..adder.last().unwrap().len()-1)/*.step_by(2)*/ {
        inner_adder.push(line[i+1] - line[i]);
    }
    adder.push(inner_adder.clone());

    if !inner_adder.iter().all(|n|n == &0) {
        generate_subrows(&line, &mut adder)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let nums = input.lines().map(|line| line.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<isize>>()).collect::<Vec<Vec<isize>>>();
    // println!("{:?}", nums);
    let mut count = 0;
    nums.iter().for_each(|line| {
        let mut adder = vec![];
        adder.push(line.clone());
        generate_subrows(line, &mut adder);
        println!("{:?}", adder);

        adder.reverse();
        for i in 0..adder.len() {
            let a = adder[i].last().unwrap() + adder.get(i.saturating_sub(1)).unwrap_or(&vec![0isize]).last().unwrap();
            adder[i].push(a);
        }
        count += adder.last().unwrap().last().unwrap();
        println!("{:?}", adder);
    });
    println!("{}", count);
}
