use std::collections::HashSet;

use util::{read_file, split_at_newline};

#[derive(PartialEq, Eq, Clone,Debug,Hash)]
enum Direction {
    North,
    East,
    South,
    West,
    None
}

#[derive(PartialEq,Eq,Debug,Clone,Hash)]
struct Position {
    start: (usize,usize),
    pos: (usize,usize),
    direction: Direction
}

fn part1() -> usize {
    let file = split_at_newline(read_file("input.txt".to_string()));
    let table: Vec<Vec<u32>> = file.iter().map(|x| x.split("").collect::<Vec<&str>>().iter().filter(|&y| y!=&"").map(|y| y.parse::<u32>().unwrap_or(10)).collect()).collect();

    let mut stack: Vec<Position> = vec![];

    for i in 0..table.len() {
        for j in 0..table[0].len() {
            if table[i][j] == 0 {stack.push(
                Position {
                    start: (i,j),
                    pos: (i,j),
                    direction: Direction::None
                }
            );}
        }
    }

    let mut endings: Vec<Position> = vec![];

    loop {
        if stack.len() == 0 {break;}

        // returns the removed elements
        // https://stackoverflow.com/questions/27904864/what-does-cannot-move-out-of-index-of-mean
        let curpos = stack.remove(0);
        let pos = curpos.pos; let start = curpos.start; let dir = curpos.direction;
        let y = pos.0; let x = pos.1;

        let north = (-1i32,0i32,Direction::South);
        let east = (0i32,1i32,Direction::West);
        let south = (1i32,0i32,Direction::North);
        let west = (0i32,-1i32,Direction::East);
        let excluedir = match dir {
            Direction::North => { Direction::South },
            Direction::East => {Direction::West},
            Direction::South => {Direction::North},
            Direction::West => {Direction::East}
            Direction::None => {Direction::None}
        };

        for (changey,changex,fromdir) in [north,east,south,west] {
            if fromdir == excluedir {continue;}

            let newy = y.clone() as i32 +changey;
            let newx = x.clone() as i32 + changex;
            if newy < 0 || newx < 0 {continue;}

            let row = table.iter().nth(newy as usize);
            if row.is_none() {continue;}
            let raw = row.unwrap().iter().nth(newx as usize);
            if raw.is_none() {continue;}

            let number: u32 = *raw.unwrap();
            let oldnumber = table[y][x];
            if number as i32 - oldnumber as i32 != 1 {continue;}

            if raw.unwrap() == &9u32 {endings.push(
                Position {
                    start,
                    pos: (newy as usize,newx as usize),
                    direction: Direction::None
                }
            );continue;}

            stack.push(
                Position {
                    start,
                    pos: (newy as usize, newx as usize),
                    direction: fromdir
                }
            );
        }
    }

    // https://users.rust-lang.org/t/better-way-to-find-unique-values/38966
    // luckily, requiring hashset is automatic, so this was easy
    let endingshash = endings.iter().collect::<HashSet<_>>();
    endingshash.into_iter().count()
}

fn part2() -> usize {
    let file = split_at_newline(read_file("input.txt".to_string()));
    let table: Vec<Vec<u32>> = file.iter().map(|x| x.split("").collect::<Vec<&str>>().iter().filter(|&y| y!=&"").map(|y| y.parse::<u32>().unwrap_or(10)).collect()).collect();

    let mut stack: Vec<Position> = vec![];

    for i in 0..table.len() {
        for j in 0..table[0].len() {
            if table[i][j] == 0 {stack.push(
                Position {
                    start: (i,j),
                    pos: (i,j),
                    direction: Direction::None
                }
            );}
        }
    }

    let mut endings: Vec<Position> = vec![];

    loop {
        if stack.len() == 0 {break;}
        let curpos = stack.remove(0);

        let pos = curpos.pos; let start = curpos.start; let dir = curpos.direction;
        let y = pos.0; let x = pos.1;

        let north = (-1i32,0i32,Direction::South);
        let east = (0i32,1i32,Direction::West);
        let south = (1i32,0i32,Direction::North);
        let west = (0i32,-1i32,Direction::East);
        let excluedir = match dir {
            Direction::North => { Direction::South },
            Direction::East => {Direction::West},
            Direction::South => {Direction::North},
            Direction::West => {Direction::East}
            Direction::None => {Direction::None}
        };

        for (changey,changex,fromdir) in [north,east,south,west] {
            if fromdir == excluedir {continue;}

            let newy = y.clone() as i32 +changey;
            let newx = x.clone() as i32 + changex;
            if newy < 0 || newx < 0 {continue;}

            let row = table.iter().nth(newy as usize);
            if row.is_none() {continue;}
            let raw = row.unwrap().iter().nth(newx as usize);
            if raw.is_none() {continue;}

            let number: u32 = *raw.unwrap();
            let oldnumber = table[y][x];
            if number as i32 - oldnumber as i32 != 1 {continue;}

            if raw.unwrap() == &9u32 {endings.push(
                Position {
                    // start is unnecessary, but it was easier this way
                    start,
                    pos: (newy as usize,newx as usize),
                    direction: Direction::None
                }
            );continue;}

            stack.push(
                Position {
                    start,
                    pos: (newy as usize, newx as usize),
                    direction: fromdir
                }
            );
        }
    }

    // commenting this line took 10 seconds, and solved me the puzzle.
    // im proud of myself.
    // let endingshash = endings.iter().collect::<HashSet<_>>();

    endings.into_iter().count()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
