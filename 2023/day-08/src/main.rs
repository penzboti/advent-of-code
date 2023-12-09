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

#[derive(Debug, Clone)]
struct Node {
    _name: String,
    current_location: String,
}

// part 2 is gonna be fun!
fn part2() -> u64 {
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

    // filters are fun, prob could've used them a couple of times before
    // https://linuxhint.com/rust-filter-method-vectors/
    let filteredlocations = locations.iter().filter(|l| l.name.chars().nth(2).unwrap() == 'A').collect::<Vec<&Location>>();
    let mut nodes: Vec<Node> = filteredlocations.iter().map(|l| Node { _name: l.name.clone(), current_location: l.name.clone() }).collect::<Vec<Node>>();
    let mut finished_nodes: Vec<(Node, u32)> = vec![];

    let mut totali = 0;
    loop {
        if nodes.len() == 0 { break; }
        if nodes.iter().filter(|n| n.current_location.chars().nth(2).unwrap() == 'Z').collect::<Vec<&Node>>().len() > 0 {
            let finished_node = nodes.iter().filter(|n| n.current_location.chars().nth(2).unwrap() == 'Z').collect::<Vec<&Node>>()[0].clone();
            finished_nodes.push((finished_node.clone(), totali));
            nodes.retain(|n| n.current_location.chars().nth(2).unwrap() != 'Z');
        }

        totali += 1;
        let i: usize = (totali - 1) as usize % directions.len();
        for n in &mut nodes {
            let mut localloc: String = n.current_location.clone();
            for l in &locations {
                if l.name == n.current_location {
                    match directions.chars().nth(i).unwrap() {
                        'L' => { localloc = l.connections.0.clone(); },
                        'R' => { localloc = l.connections.1.clone(); },
                        _ => {},
                    }
                }
            }
            n.current_location = localloc.clone();
        }
    }

    // using the lcm method on all nodes
    // https://www.hackertouch.com/least-common-multiple-in-rust.html
    let mut global_lcm: Vec<u64> = finished_nodes.iter().map(|(_, i)| *i as u64).collect::<Vec<u64>>();
    loop {
        if global_lcm.len() == 1 { break; }
        let first = global_lcm[0];
        let second = global_lcm[1];
        let mut _gcd = 0;
        let mut max = first;
        let mut min = second;
        if min > max {
            let val = max;
            max = min;
            min = val;
        }

        loop {
            let res = max % min;
            if res == 0 {
                _gcd = min;
                break;
            }

            max = min;
            min = res;
        }
        let lcm = (first * second) / _gcd;
        global_lcm[0] = lcm;
        global_lcm.remove(1);
    }

    global_lcm[0]
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}