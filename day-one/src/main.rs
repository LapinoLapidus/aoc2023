use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use regex::Regex;

fn read_lines() -> String {
    fs::read_to_string("input.txt").unwrap()
}

fn main() {
    let mut number_words = HashMap::new();
    number_words.insert("one", 1);
    number_words.insert("two", 2);
    number_words.insert("three", 3);
    number_words.insert("four", 4);
    number_words.insert("five", 5);
    number_words.insert("six", 6);
    number_words.insert("seven", 7);
    number_words.insert("eight", 8);
    number_words.insert("nine", 9);
    number_words.insert("eighthree", 83);
    number_words.insert("sevenine", 79);
    number_words.insert("oneight", 18);
    number_words.insert("eightwo", 82);
    number_words.insert("twone", 21);

    let first_regex = Regex::new("sevenine|eighthree|oneight|eightwo|twone").unwrap();
    let regex = Regex::new("(one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let mut count = 0;
    for mut _line in read_lines().lines() {

        let mut line = _line.to_string();
        line = line.replace("1", "one1one");
        let first_matches = first_regex.find_iter(_line);
        for m in first_matches {
            line = line.replace(m.as_str(), &*number_words.get(m.as_str().trim()).unwrap().to_string())
        }
        let matches = regex.find_iter(_line);
        for m in matches {
            println!("Match: {}", m.as_str());
            line = line.replace(m.as_str(), &*number_words.get(m.as_str().trim()).unwrap().to_string())
        }
        let mut first = None;
        let mut last = None;
        for char in line.chars() {
            if !char.is_numeric() { continue; }
            if first == None {
                first = Some(char)
            }
            break;
        }
        for char in line.chars().rev() {
            if !char.is_numeric() { continue; }
            if last == None {
                last = Some(char)
            }
            break;
        }
        let num = u64::from_str(&*format!("{}{}", first.unwrap(), last.unwrap())).unwrap();
        count += num;
    }
    println!("Solution: {}", count);
}
