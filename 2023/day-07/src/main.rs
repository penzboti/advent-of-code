extern crate util;
use util::{read_file, split_at_newline};

// i tought const could only work with string, but i guess its the opposite
// https://stackoverflow.com/questions/45176403/trying-to-declare-a-string-const-results-in-expected-type-found-my-string
const STRENGTH: &str = "AKQJT98765432";
// so we map every letter of the hand, according to their position in string, to abc, and then we use that to sort, and then we somehow push those back, but that is my problem. idk how to push the changes back.
const ABC: &str = "ABCDEFGHIJKLM";

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
    fn all_types() -> [Type; 7] {
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

    //* just clone them in a separate vector, compare letters or numbers (idk), then pop in the main array, and push them back in order
    //TODO then we compare the hands, and assign a rank to each one
    // it prob would've worked, if the elements were only enums
    // but at least i got to know that it works, at the link below
    // https://users.rust-lang.org/t/how-to-sort-vec-enum/58694
    hands.sort_by(|a, b| a.r#type.cmp(&b.r#type));
    println!("{:?}", hands);
    println!("---");
    let mut index_by_type: Vec<usize> = vec![];
    for i in 0..hands.len() { if i != hands.len() - 1 {
        if hands[i+1].r#type == hands[i].r#type {
            // we only gather the indexes here
            index_by_type.push(i);
        }
        else { 
            // we compare all the cards here, using the indexes we've gathered
            // watching this video gave me an idea
            // https://youtu.be/7zVAz2iMViE?si=2mini-vVeGkvrbar
            // because finding something that uses the STRENGTH string as a reference, we turn them into chars [a-z] and then sort should sort it in a way that works for us
            //* idk, my brain is fried
            //* im close tho, but this sorting is just too much for me atm
            hands[i+1].rank = hands[i].rank + 1;
            index_by_type = vec![];
        }
    }}
    println!("{:?}", hands);
    // dont forget to invert all of the ranks!

    let mut sum = 0;
    //TODO then we multiply the bid by the rank

    sum
}

fn main() {
    println!("Part 1: {}", part1());
}
