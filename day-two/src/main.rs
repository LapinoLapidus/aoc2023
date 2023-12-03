use std::fs;
use std::io::Read;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RED_REGEX: Regex = Regex::new("[0-9][0-9]? red").unwrap();
    static ref GREEN_REGEX: Regex = Regex::new("[0-9][0-9]? green").unwrap();
    static ref BLUE_REGEX: Regex = Regex::new("[0-9][0-9]? blue").unwrap();
    static ref REPLACE_GAME_REGEX: Regex = Regex::new("Game [1-9][0-9]?: ").unwrap();
}

const RED_CUBES: u8 = 12;
const GREEN_CUBES: u8 = 13;
const BLUE_CUBES: u8 = 14;

fn read_lines() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[derive(Debug)]
struct Set(u8, u8, u8);

struct Game(Vec<Set>);

fn get_sets_from_game(game: &str) -> Vec<Set> {
    let mut set_vec = Vec::new();
    let game = REPLACE_GAME_REGEX.replace_all(game, "").to_string();
    let sets = game.split(";");
    for set in sets {
        let red = match RED_REGEX.find(set) {
            None => { 0u8 }
            Some(m) => { u8::from_str(&m.as_str()[0..2].trim()).unwrap() }
        };
        let green = match GREEN_REGEX.find(set) {
            None => { 0u8 }
            Some(m) => { u8::from_str(&m.as_str()[0..2].trim()).unwrap() }
        };
        let blue = match BLUE_REGEX.find(set) {
            None => { 0u8 }
            Some(m) => { u8::from_str(&m.as_str()[0..2].trim()).unwrap() }
        };
        set_vec.push(Set(red, green, blue));
    }
    set_vec
}

/* part one
// fn main() {
//     let mut failed_games: Vec<u64> = vec![];
//     for mut game in read_lines().lines() {
//         for set in get_sets_from_game(game) {
//             let game_id = u64::from_str(&*game[5..8].replace(":", "").trim()).unwrap();
//             if set.0 > RED_CUBES {
//                 failed_games.push(game_id);
//                 break;
//             }
//             if set.1 > GREEN_CUBES {
//                 failed_games.push(game_id);
//                 break;
//             }
//             if set.2 > BLUE_CUBES {
//                 failed_games.push(game_id);
//                 break;
//             }
//         }
//     }
//     let success_games: Vec<u64> = (1..101).filter(|num| !failed_games.contains(num)).collect();
//     println!("{:?}", success_games);
//     println!("{:?}", success_games.iter().sum::<u64>())
/ }*/
fn main() {
    let mut failed_games: Vec<u64> = vec![];
    let mut counter = 0usize;
    for mut game in read_lines().lines() {
        let mut r = vec![];
        let mut g = vec![];
        let mut b = vec![];

        for set in get_sets_from_game(game) {
            let game_id = u64::from_str(&*game[5..8].replace(":", "").trim()).unwrap();
            r.push(set.0);
            g.push(set.1);
            b.push(set.2);
        }
        r.sort();
        g.sort();
        b.sort();
        let highest_r: usize = (*r.last().unwrap()).into();
        let highest_g: usize = (*g.last().unwrap()).into();
        let highest_b: usize = (*b.last().unwrap()).into();
        counter += (highest_r * highest_g * highest_b);
    }

    println!("{}", counter)
}
