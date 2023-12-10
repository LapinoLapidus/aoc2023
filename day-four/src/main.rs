use std::fs;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CARD_REGEX: Regex = Regex::new("Card[ ]*[0-9]*: ").unwrap();
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut points = 0u32;
    let mut copies: Vec<u32> = vec![];

    for (i, line) in input.lines().enumerate() {
        let line = CARD_REGEX.replace(line, " ").to_string();
        let divide = line.split(" | ").collect::<Vec<&str>>();
        let (winning, mine) = (divide[0], divide[1]);
        let winning = winning.split(" ").filter(|n| n.len() != 0).map(|n| u32::from_str(n).unwrap()).collect::<Vec<u32>>();
        let mine = mine.split(" ").filter(|n| n.len() != 0).map(|n| u32::from_str(n).unwrap()).collect::<Vec<u32>>();

        copies.push(i as u32);
        let mut copies_temp: Vec<u32> = vec![];
        for _ in copies.iter().filter(|n| **n == i as u32) {
            let mut i_incr = i as u32;
            for n in &mine {
                if winning.contains(&n) {
                    i_incr += 1;
                    copies_temp.push(i_incr);
                }
            }
        }
        copies.append(&mut copies_temp)
    }
    println!("Scratchcards: {}", copies.len());
}
