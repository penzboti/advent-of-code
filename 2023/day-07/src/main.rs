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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone)]
struct Hand {
    hand: String,
    r#type: Type,
    bid: u32,
    rank: u32,
}

impl Hand {
    fn into_numbers(hand: String, strengthtxt: &str) -> [usize; 5] {
        let mut strength = [0, 0, 0, 0, 0];
        for i in 0..hand.len() {
            let c = hand.chars().nth(i).unwrap();
            // index_of
            // https://stackoverflow.com/a/30558370/12706133
            strength[i] = strengthtxt.chars().position(|x| x == c).unwrap();
        }
        strength
    }
}

fn part1() -> u32 {
    let file = split_at_newline(read_file("input.txt".to_string()));
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

    // sorting by enums
    // https://users.rust-lang.org/t/how-to-sort-vec-enum/58694
    let mut sortedhands: Vec<Hand> = Vec::new();
    hands.sort_by(|a, b| a.r#type.cmp(&b.r#type));
    let mut index_by_type: Vec<usize> = vec![];
    for i in 0..hands.len() { if i != hands.len() - 1 {
        index_by_type.push(i);
        if hands[i+1].r#type != hands[i].r#type {
            let immuthands = hands.clone();
            let firstindex = index_by_type[0];
            let mut typehands = immuthands.iter().filter(|x| x.r#type == hands[firstindex].r#type).collect::<Vec<&Hand>>();
            typehands.sort_by(|a, b| Hand::into_numbers(a.hand.clone(), STRENGTH).cmp(&Hand::into_numbers(b.hand.clone(), STRENGTH)));
            for h in typehands {
                sortedhands.push(h.clone());
            }
            index_by_type = vec![];
        }
    } else {
        // we probably could've done this in one part, rather than copying the code like 2 lines below
        // oh well, thats for another time (or not)
        index_by_type.push(i);
        let immuthands = hands.clone();
        let firstindex = index_by_type[0];
        let mut typehands = immuthands.iter().filter(|x| x.r#type == hands[firstindex].r#type).collect::<Vec<&Hand>>();
        typehands.sort_by(|a, b| Hand::into_numbers(a.hand.clone(), STRENGTH).cmp(&Hand::into_numbers(b.hand.clone(), STRENGTH)));
        for h in typehands {
            sortedhands.push(h.clone());
        }
    }}

    let length = sortedhands.len();
    for i in 0..length {
        sortedhands[i].rank = (length - i) as u32;
    }

    let mut sum = 0;
    for hand in &sortedhands {
        sum += hand.bid * hand.rank;
    }
    sum
}


const STRENGTH2: &str = "AKQT98765432J";

fn part2() -> u32 {
    let file = split_at_newline(read_file("input.txt".to_string()));
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
                let n = h.hand.matches(c).count();
                chars.push(c);
                numbers.push(n as u32);
            }
        }
        if chars.contains(&'J') {
            let index = chars.iter().position(|&x| x == 'J').unwrap();
            let n = numbers[index];
            numbers[index] = 0;
            // maximum value in a vector
            // https://stackoverflow.com/questions/58669865/how-to-get-the-minimum-value-within-a-vector-in-rust
            *numbers.iter_mut().max().unwrap() += n;
            // the remove() fn
            // https://stackoverflow.com/questions/26243025/how-to-remove-an-element-from-a-vector-given-the-element
            if numbers.len() != 1 { numbers.remove(index); chars.remove(index); }
        }
        if numbers.len() == 1 { h.r#type = Type::FiveOfAKind; } else
        if numbers.contains(&4) { h.r#type = Type::FourOfAKind; } else
        if numbers.contains(&3) && numbers.contains(&2) { h.r#type = Type::FullHouse; } else
        if numbers.contains(&3) { h.r#type = Type::ThreeOfAKind; } else
        if numbers.contains(&2) && numbers.len() == 3 { h.r#type = Type::TwoPair; } else
        if numbers.contains(&2) { h.r#type = Type::OnePair; }
        else { h.r#type = Type::HighCard; }
    }

    let mut sortedhands: Vec<Hand> = Vec::new();
    hands.sort_by(|a, b| a.r#type.cmp(&b.r#type));
    let mut index_by_type: Vec<usize> = vec![];
    for i in 0..hands.len() { if i != hands.len() - 1 {
        index_by_type.push(i);
        if hands[i+1].r#type != hands[i].r#type {
            let immuthands = hands.clone();
            let firstindex = index_by_type[0];
            let mut typehands = immuthands.iter().filter(|x| x.r#type == hands[firstindex].r#type).collect::<Vec<&Hand>>();
            typehands.sort_by(|a, b| Hand::into_numbers(a.hand.clone(), STRENGTH2).cmp(&Hand::into_numbers(b.hand.clone(), STRENGTH2)));
            for h in typehands {
                sortedhands.push(h.clone());
            }
            index_by_type = vec![];
        }
    } else {
        index_by_type.push(i);
        let immuthands = hands.clone();
        let firstindex = index_by_type[0];
        let mut typehands = immuthands.iter().filter(|x| x.r#type == hands[firstindex].r#type).collect::<Vec<&Hand>>();
        typehands.sort_by(|a, b| Hand::into_numbers(a.hand.clone(), STRENGTH2).cmp(&Hand::into_numbers(b.hand.clone(), STRENGTH2)));
        for h in typehands {
            sortedhands.push(h.clone());
        }
    }}

    let length = sortedhands.len();
    for i in 0..length {
        sortedhands[i].rank = (length - i) as u32;
    }

    let mut sum = 0;
    for hand in &sortedhands {
        sum += hand.bid * hand.rank;
    }
    sum
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}