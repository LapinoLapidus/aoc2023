#![feature(iter_collect_into)]

use std::fs;
use std::str::FromStr;

struct XToY {
    dest_start: usize,
    src_start: usize,
    range: usize
}

fn x_to_y(a: &str, x: &mut Vec<XToY>) {
    let mut a = a.split("\n").collect::<Vec<&str>>();
    for i in 1..a.len() {
        let s = a[i].split_ascii_whitespace().map(|n| usize::from_str(n).unwrap()).collect::<Vec<usize>>();
        if s.len() == 0 {continue}
        x.push(XToY {
            dest_start: s[0],
            src_start: s[1],
            range: s[2]
        });
    }
}

fn a_to_b(num: usize, vec: &Vec<XToY>) -> usize {
    let mut val = num;
    for v in vec {
        if num >= v.src_start && num < v.src_start + v.range {
            val = v.dest_start + (num - v.src_start)
        }
    }
    val
}

fn main() {
    let lines = fs::read_to_string("input.txt").unwrap();
    let divided = lines.split("\n\n").collect::<Vec<&str>>();

    let mut seeds: Vec<usize> = vec![];

    let mut seed_to_soil: Vec<XToY> = Vec::new();
    let mut soil_to_fertilizer: Vec<XToY> = Vec::new();
    let mut fertilizer_to_water: Vec<XToY> = Vec::new();
    let mut water_to_light: Vec<XToY> = Vec::new();
    let mut light_to_temperature: Vec<XToY> = Vec::new();
    let mut temperature_to_humidity: Vec<XToY> = Vec::new();
    let mut humidity_to_location: Vec<XToY> = Vec::new();

    for category in divided {
        if category.starts_with("seeds:") {
            let category = category.replace("seeds: ", "");
            let seeds_and_lengths = category.split(" ").map(|n| usize::from_str(n).unwrap()).collect::<Vec<usize>>();
            for x in (0..seeds_and_lengths.len()).step_by(2) {
                (seeds_and_lengths[x]..seeds_and_lengths[x]+ seeds_and_lengths[x+1]).collect_into(&mut seeds);
            }
        } else if category.starts_with("seed-to-soil map:") {
            x_to_y(category, &mut seed_to_soil);
        } else if category.starts_with("soil-to-fertilizer map:") {
            x_to_y(category, &mut soil_to_fertilizer);
        } else if category.starts_with("fertilizer-to-water map:") {
            x_to_y(category, &mut fertilizer_to_water);
        } else if category.starts_with("water-to-light map:") {
            x_to_y(category, &mut water_to_light);
        } else if category.starts_with("light-to-temperature map:") {
            x_to_y(category, &mut light_to_temperature);
        } else if category.starts_with("temperature-to-humidity map:") {
            x_to_y(category, &mut temperature_to_humidity);
        } else if category.starts_with("humidity-to-location map:") {
            x_to_y(category, &mut humidity_to_location);
        }
    }
    let mut smallest = 9999999999usize;
    for seed in seeds {
        let soil = a_to_b(seed, &seed_to_soil);
        let fertilizer = a_to_b(soil, &soil_to_fertilizer);
        let water = a_to_b(fertilizer, &fertilizer_to_water);
        let light = a_to_b(water, &water_to_light);
        let temperature = a_to_b(light, &light_to_temperature);
        let humidity = a_to_b(temperature, &temperature_to_humidity);
        let location = a_to_b(humidity, &humidity_to_location);
        smallest = smallest.min(location);
        // println!("Seed: {}, Soil: {}, Fertilizer: {}, Water: {}, Light: {}, Temperature: {}, Humidity: {}, Location: {}", seed, soil, fertilizer, water, light, temperature, humidity, location)
    }
    println!("{}", smallest);
}
