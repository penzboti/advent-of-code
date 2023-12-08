extern crate util;
use util::{read_file, split_at_newline, split_at_empty_line};

#[derive(Debug)]
struct Location {
    name: String,
    connections: (String, String),
}

fn part1() -> u32 {
    let file = split_at_empty_line(read_file("input.txt".to_string()));
    let directions = &file[0];
    let rawlocs = split_at_newline(file[1].clone());
    let mut locations: Vec<Location> = vec![];

    for l in rawlocs {
        let rawsplit = l.split(" = (").collect::<Vec<&str>>();
        let rawconns = rawsplit[1].split(", ").collect::<Vec<&str>>();
        let endcon = rawconns[1].split(')').next().unwrap().to_string();
        locations.push( Location { name: rawsplit[0].to_string(), connections: (rawconns[0].to_string(), endcon) } );
    }

    let mut totali = 0;
    let mut currloc = "AAA".to_string();
    loop {
        if currloc == "ZZZ".to_string() { break; }
        totali += 1;
        let i = (totali - 1) % directions.len();
        // cloning strings, idk why tho
        // https://stackoverflow.com/questions/65677430/how-to-fix-string-field-does-not-implement-copy
        let mut localloc: String = currloc.clone();
        for l in &locations {
            if l.name == currloc {
                // indexing strings are different for sure
                // https://stackoverflow.com/a/24542502/12706133
                match directions.chars().nth(i).unwrap() {
                    'L' => { localloc = l.connections.0.clone(); },
                    'R' => { localloc = l.connections.1.clone(); },
                    _ => {},
                }
            }
        }
        currloc = localloc.clone();

    }

    totali as u32
}

// part 2 is gonna be fun!

fn main() {
    println!("Part 1: {}", part1());
}