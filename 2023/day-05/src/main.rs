extern crate util;
use util::{read_file, split_at_empty_line, split_at_newline};

#[derive(Ord, Eq, PartialOrd, PartialEq, Clone, Copy)]
#[derive(Debug)]
struct Map {
    destination: u64,
    source: u64,
    range: u64,
}
#[derive(Debug)]
struct Seed {
    seed: u64,
    maps: Vec<u64>,
}

fn part1() -> u64 {
    let file = split_at_empty_line(read_file("input.txt".to_string()));

    let initseeds = file[0].split(": ")
    .collect::<Vec<&str>>()[1]
    .split(" ")
    .map(|x| x.parse::<u64>().unwrap_or(0))
    .collect::<Vec<u64>>();
    let mut seeds = initseeds.iter().map(|x| Seed { seed: *x, maps: vec![*x] }).collect::<Vec<Seed>>();

    for i in 1..file.len() {
        let initline = file[i].split(":\r\n")
        .collect::<Vec<&str>>()[1].to_string();
        let initlines = split_at_newline(initline);
        let initnums = initlines.iter()
        .map(|x| x.split(' ')
            .map(|y| y.parse::<u64>().unwrap_or(0))
            .collect::<Vec<u64>>()
        )
        // i could've done this if i give it more time, honestly
        // .map(|x| Map { destination: x[0], source: x[1], range: x[2] } ).collect::<Vec<Map>>()
        // .collect::<Vec<Map>>();
        .collect::<Vec<Vec<u64>>>();
    
        let mut ranges = initnums.iter().map(|x| Map { destination: x[0], source: x[1], range: x[2] } ).collect::<Vec<Map>>();
        // sorting, so we use the lowest number first
        // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html
        ranges.sort_by(|a, b| a.source.cmp(&b.source));

        for s in &mut seeds {
            let mut found = false;
            for r in &ranges {
                if r.source <= s.maps[i-1] && r.source + r.range-1 >= s.maps[i-1] {
                    s.maps.push((s.maps[i-1] as i64 + (r.destination as i64 - r.source as i64)) as u64);
                    found = true;
                }
            }
            if !found {
                s.maps.push(s.maps[i-1]);
            }
        }

        // tried to store maps, rather than numbers, but i got lost
        // FIND THIS SNIPPET IN alt_solution.rs
    }

    seeds.sort_by(|a, b| a.maps[a.maps.len()-1].cmp(&b.maps[b.maps.len()-1]));

    seeds[0].maps[seeds[0].maps.len()-1]
}

fn part2() -> u64 {
    let file = split_at_empty_line(read_file("input.txt".to_string()));

    let initseeds = file[0].split(": ")
    .collect::<Vec<&str>>()[1]
    .split(" ")
    .map(|x| x.parse::<u64>().unwrap_or(0))
    .collect::<Vec<u64>>();
    // basically change this
    let mut seeds: Vec<Seed> = vec![];
    let mut step = 0;

    // looping with steps by two seemed hard in rust, so just used an outer variable
    // https://stackoverflow.com/questions/43823042/is-it-possible-to-step-by-a-different-amount-each-iteration-without-creating-a-s
    //* sadly its fucking big. 
    for i in 0..(initseeds.len()/2) {
        for j in 0..initseeds[i+step+1]+1 {
            seeds.push(Seed { seed: initseeds[i+step]+j, maps: vec![initseeds[i+step]+j] });
        }
        step += 1;
    }
    // we NEVER reach this point.
    //TODO try to use ranges ig, the *easier* solution doesn't work 

    for i in 1..file.len() {
        let initline = file[i].split(":\r\n")
        .collect::<Vec<&str>>()[1].to_string();
        let initlines = split_at_newline(initline);
        let initnums = initlines.iter()
        .map(|x| x.split(' ')
            .map(|y| y.parse::<u64>().unwrap_or(0))
            .collect::<Vec<u64>>()
        )
        .collect::<Vec<Vec<u64>>>();
    
        let mut ranges = initnums.iter().map(|x| Map { destination: x[0], source: x[1], range: x[2] } ).collect::<Vec<Map>>();
        ranges.sort_by(|a, b| a.source.cmp(&b.source));

        for s in &mut seeds {
            let mut found = false;
            for r in &ranges {
                if r.source <= s.maps[i-1] && r.source + r.range-1 >= s.maps[i-1] {
                    s.maps.push((s.maps[i-1] as i64 + (r.destination as i64 - r.source as i64)) as u64);
                    found = true;
                }
            }
            if !found {
                s.maps.push(s.maps[i-1]);
            }
        }
    }

    seeds.sort_by(|a, b| a.maps[a.maps.len()-1].cmp(&b.maps[b.maps.len()-1]));

    seeds[0].maps[seeds[0].maps.len()-1]
}

fn main() {
    println!("Part 1: {}", part1());
    // println!("Part 2: {}", part2());
}
