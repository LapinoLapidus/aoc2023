use std::fs;

#[derive(Debug)]
struct Race(usize, usize);

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();

    let time: Vec<usize> = lines[0].split_ascii_whitespace().filter_map(|n| n.parse().ok()).collect();
    let distance: Vec<usize> = lines[1].split_ascii_whitespace().filter_map(|n| n.parse().ok()).collect();

    let races: Vec<Race> = (0..time.len()).map(|n| Race(time[n], distance[n])).collect();

    let mut fast_enough: Vec<usize> = vec![];
    for race in races {
        fast_enough.push((0..race.0).filter(|x| (race.0 - x) * x > race.1).count());
    }
    println!("{:?}", fast_enough.iter().product::<usize>());
}
