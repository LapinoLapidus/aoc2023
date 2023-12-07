use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use crate::Card::{A, Eight, Five, Four, J, K, Nine, One, Q, Seven, Six, T, Three, Two};
use crate::Hand::{FIVE_OF_A_KIND, FOUR_OF_A_KIND, FULL_HOUSE, HIGH_CARD, ONE_PAIR, THREE_OF_A_KIND, TWO_PAIR};

#[derive(Debug, Eq, PartialEq, Hash, Ord)]
struct HandBid<'a>(&'a str, usize);

impl PartialOrd for HandBid<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        println!("Ordering {:?} with {:?}", &self, other);
        for (i, c) in self.0.chars().enumerate() {
            let card = Card::from(c);
            let other_card = Card::from(other.0.chars().nth(i).unwrap());
            if card < other_card {
                println!("{:?} < {:?}", card, other_card);
                return Option::from(Ordering::Greater);
                }
                if card > other_card {
                    return Option::from(Ordering::Less);
                }
                continue
            }
        Option::from(Ordering::Equal)
    }

}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => A,
            'K' => K,
            'Q' => Q,
            'J' => J,
            'T' => T,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            '1' => One,
            _ => A
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Hand {
    FIVE_OF_A_KIND,
    FOUR_OF_A_KIND,
    FULL_HOUSE,
    THREE_OF_A_KIND,
    TWO_PAIR,
    ONE_PAIR,
    HIGH_CARD,
}

fn identify_hand(hand: &str) -> Hand {
    let mut map = HashMap::new();

    hand.chars().for_each(|ch| *map.entry(ch).or_insert(0) += 1);
    if map.values().filter(|k| k == &&4).count() == 1 { return FOUR_OF_A_KIND };
    if map.values().filter(|k| k == &&5).count() == 1 { return FIVE_OF_A_KIND };

    // Full house or three of a kind
    let three = map.values().filter(|k| k == &&3).collect::<Vec<_>>();
    if three.len() == 1 {
        if map.iter().filter(|c| c.1 != &3).collect::<Vec<_>>().iter().all(|i| i.1 > &1) { return FULL_HOUSE } else { return THREE_OF_A_KIND };
    }

    // Two pair
    let pair_count = map.iter().filter(|e| e.1 == &2).count();
    if  pair_count == 2 && map.iter().filter(|e| e.1 == &1).count() == 1 {
        return TWO_PAIR
    }

    if pair_count == 1 && map.iter().filter(|e| e.1 == &1).count() == 3 {
        return ONE_PAIR
    }

    HIGH_CARD

}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let handbids = input.lines().map(|line| HandBid(line.split_ascii_whitespace().nth(0).unwrap(), line.split_ascii_whitespace().nth(1).unwrap().parse().unwrap())).collect::<Vec<HandBid>>();

    let mut map = HashMap::new();
    let mut sorted: Vec<&HandBid> = vec![];
    handbids.iter().for_each(|handbid| {
        let hand = handbid.0;
        let identified = identify_hand(hand);
        println!("{}: {:?}", handbid.0, &identified);
        map.insert(handbid, identified);
    });

    let mut high_cards = map.iter().filter(|hbh| hbh.1 == &HIGH_CARD).map(|hbh| *hbh.0).collect::<Vec<_>>();
    let mut one_pairs = map.iter().filter(|hbh| hbh.1 == &ONE_PAIR).map(|hbh| *hbh.0).collect::<Vec<_>>();
    let mut two_pairs = map.iter().filter(|hbh| hbh.1 == &TWO_PAIR).map(|hbh| *hbh.0).collect::<Vec<_>>();
    let mut three_of_a_kinds = map.iter().filter(|hbh| hbh.1 == &THREE_OF_A_KIND).map(|hbh| *hbh.0).collect::<Vec<_>>();
    let mut full_houses = map.iter().filter(|hbh| hbh.1 == &FULL_HOUSE).map(|hbh| *hbh.0).collect::<Vec<_>>();
    let mut four_of_a_kinds = map.iter().filter(|hbh| hbh.1 == &FOUR_OF_A_KIND).map(|hbh| *hbh.0).collect::<Vec<_>>();
    let mut five_of_a_kinds = map.iter().filter(|hbh| hbh.1 == &FIVE_OF_A_KIND).map(|hbh| *hbh.0).collect::<Vec<_>>();
    high_cards.sort();
    one_pairs.sort();
    two_pairs.sort();
    three_of_a_kinds.sort();
    full_houses.sort();
    four_of_a_kinds.sort();
    five_of_a_kinds.sort();
    sorted.append(&mut high_cards);
    sorted.append(&mut one_pairs);
    sorted.append(&mut two_pairs);
    sorted.append(&mut three_of_a_kinds);
    sorted.append(&mut full_houses);
    sorted.append(&mut four_of_a_kinds);
    sorted.append(&mut five_of_a_kinds);
    println!("Sorted: {:?}", sorted);

    let mut count = 0;
    for (i, x) in sorted.iter().enumerate() {
        count += x.1 * (i+1);
    }

    println!("{:?}", count);
}
