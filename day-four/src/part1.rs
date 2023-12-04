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

    for line in input.lines() {
        let line = CARD_REGEX.replace_all(line, "").to_string();
        let divide = line.split(" | ").collect::<Vec<&str>>();
        let (winning, mine) = (divide[0], divide[1]);
        let winning = winning.split(" ").filter(|n| n.len() != 0).map(|n| u32::from_str(n).unwrap()).collect::<Vec<u32>>();
        let mine = mine.split(" ").filter(|n| n.len() != 0).map(|n| u32::from_str(n).unwrap()).collect::<Vec<u32>>();

        let mut mult = 0u32;

        for n in &mine {
            if winning.contains(&n) {
                if mult == 0 {
                    mult = 1;
                } else {
                    mult *= 2;
                }

            }
        }
        points += mult;
    }
    println!("Points: {}", points);
}
