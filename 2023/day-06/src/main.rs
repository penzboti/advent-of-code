extern crate util;
use util::{read_file, split_at_newline};

#[derive(Debug)]
struct Race {
    duration: u32,
    record: u32,
    win_with: Vec<u32>,
}

fn part1() -> u32 {
    let file = split_at_newline(read_file("input.txt".to_string()));
    let onlynums = file.iter().map(|x| x.split(':').next_back().unwrap_or(&"")).collect::<Vec<&str>>();
    // split at whitespace
    // https://www.educative.io/answers/what-is-the-stringsplitwhitespace-method-in-rust
    let rawnums = onlynums.iter().map(|x| x.split_whitespace()
    .map(|x| x.parse::<u32>().unwrap_or(0)).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
    let mut races: Vec<Race> = vec![];
    
    let mut sum = 1;
    for i in 0..rawnums[0].len() {
        races.push(Race { duration: rawnums[0][i], record: rawnums[1][i], win_with: vec![] });
    }

    for race in &mut races {
        let duration = race.duration;
        let record = race.record;
        let mut win_with: Vec<u32> = vec![];
        for i in 1..(race.duration+1) {
            let ourdistance = i*(duration-i);
            if ourdistance > record {
                win_with.push(i);
            }
        }
        race.win_with = win_with;
    }

    races.iter().for_each(|x| sum *= x.win_with.len() as u32);

    sum
}


#[derive(Debug)]
struct RaceSmol {
    duration: u32,
    record: u64,
}

fn part2() -> u32 {
    let file = split_at_newline(read_file("input.txt".to_string()));
    let onlynums = file.iter().map(|x| x.split(':').next_back().unwrap_or(&"")).collect::<Vec<&str>>();
    let racevec = onlynums.iter()
    .map(|x| x.split_whitespace()
        // join, just like in javascript i guess
        // https://stackoverflow.com/questions/28311868/what-is-the-equivalent-of-the-join-operator-over-a-vector-of-strings
        .collect::<Vec<&str>>().join("")
        .parse::<u64>().unwrap_or(0))
    .collect::<Vec<u64>>();
    let race = RaceSmol { duration: racevec[0] as u32, record: racevec[1] };
    
    let duration = race.duration;
    let record = race.record;
    let mut win_with: Vec<u32> = vec![];
    for i in 1..(duration+1) {
        let ourdistance: u64 = (i as u64*(duration-i) as u64) as u64;
        if ourdistance > record {
            win_with.push(i);
        }
    }

    let sum = win_with.len() as u32;
    sum
}

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}
