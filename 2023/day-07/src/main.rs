extern crate util;
use util::{read_file, split_at_newline};

// i tought const could only work with string, but i guess its the opposite
// https://stackoverflow.com/questions/45176403/trying-to-declare-a-string-const-results-in-expected-type-found-my-string
const STRENGTH: &str = "AKQJT98765432";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
    None,
}
use crate::Type::*;
impl Type {
    fn allTypes() -> [Type; 7] {
        [FiveOfAKind, FourOfAKind, FullHouse, ThreeOfAKind, TwoPair, OnePair, HighCard]
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone)]
struct Hand {
    hand: String,
    r#type: Type,
    bid: u32,
    rank: u32,
}

fn part1() -> u32 {
    let file = split_at_newline(read_file("demo1.txt".to_string()));
    let raw_hands = file.iter()
    .map(|x| x.split(" ").collect::<Vec<&str>>())
    .collect::<Vec<Vec<&str>>>();
    
    let mut hands: Vec<Hand> = raw_hands.iter()
    .map(|x| Hand {
        hand: x[0].to_string(),
        r#type: Type::None,
        bid: x[1].parse::<u32>().unwrap(),
        rank: 0,
    }).collect::<Vec<Hand>>();

    for h in &mut hands {
        let mut chars: Vec<char> = vec![];
        let mut numbers: Vec<u32> = vec![];
        for c in h.hand.chars() {
            if !chars.contains(&c) {
                // thats cool af
                // https://programming-idioms.org/idiom/82/count-substring-occurrences/797/rust
                let n = h.hand.matches(c).count();
                chars.push(c);
                numbers.push(n as u32);
            }
        }
        if numbers.len() == 1 { h.r#type = Type::FiveOfAKind; } else
        if numbers.contains(&4) { h.r#type = Type::FourOfAKind; } else
        if numbers.contains(&3) && numbers.contains(&2) { h.r#type = Type::FullHouse; } else
        if numbers.contains(&3) { h.r#type = Type::ThreeOfAKind; } else
        if numbers.contains(&2) && numbers.len() == 3 { h.r#type = Type::TwoPair; } else
        if numbers.contains(&2) { h.r#type = Type::OnePair; }
        else { h.r#type = Type::HighCard; }
    }

    //TODO then we compare the hands, and assign a rank to each one
    //? just use normal sort, and then work from there?
    // just so i know
    // https://users.rust-lang.org/t/how-to-sort-vec-enum/58694
    let mut currank = 1;
    for h in &mut hands {
        let h.rank = currank;
        for h2 in &mut hands {
            // idk how this gon work yet
        }
    }

    let mut sum = 0;
    //TODO then we multiply the bid by the rank

    sum
}

fn main() {
    println!("Part 1: {}", part1());
}
