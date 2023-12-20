extern crate util;
use util::{read_file, split_at_newline};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use crate::Direction::*;

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: u16,
    _color: String,
}

fn part1() -> u32 {
    let file = split_at_newline(read_file("input.txt".to_string()));

    let instructions: Vec<Instruction> = file.iter()
        .map(|line| {
            let split = line.split(" ").collect::<Vec<&str>>();
            let direction = match split[0] {
                "U" => Up,
                "D" => Down,
                "L" => Left,
                "R" => Right,
                _ => panic!("Invalid direction"),
            };
            Instruction {
                direction,
                distance: split[1].parse::<u16>().unwrap(),
                _color: split[2].replace(&['(', ')'][..], "").to_string(),
            }
        })
    .collect::<Vec<Instruction>>();

    // getting map size
    let mut x: i16 = 0; let mut y: i16 = 0;
    let mut max_x = 0; let mut max_y = 0;
    let mut min_x = 0; let mut min_y = 0;
    for instruction in &instructions {
        let dist = instruction.distance as i16;
        match instruction.direction {
            Right => {x+=dist; if x > max_x {max_x = x;}},
            Left => {x-=dist; if x < min_x {min_x = x;}},
            Down => {y+=dist; if y > max_y {max_y = y;}},
            Up => {y-=dist; if y < min_y {min_y = y;}},
        }
    }

    println!("x: {} {} y: {} {}", min_x, max_x, min_y, max_y);

    let mut lagoon: Vec<Vec<String>> = Vec::new();
    for _ in 0..(max_y-min_y+1) {
        let mut row: Vec<String> = Vec::new();
        for _ in 0..(max_x-min_x+1) {
            row.push(".".to_string());
        }
        lagoon.push(row);
    }

    for instruction in instructions {
        match instruction.direction {
            Right => {
                for i in 0..instruction.distance+1 {
                    lagoon[(y-min_y) as usize][(x-min_x+i as i16) as usize] = '#'.to_string();
                }
                x += instruction.distance as i16;
            },
            Left => {
                x -= instruction.distance as i16;
                for j in 0..instruction.distance {
                    let i = instruction.distance-j-1;
                    lagoon[(y-min_y) as usize][(x-min_x+i as i16) as usize] = '#'.to_string();
                }
            },
            Down => {
                for i in 0..instruction.distance+1 {
                    lagoon[(y-min_y+i as i16) as usize][(x-min_x) as usize] = '#'.to_string();
                }
                y += instruction.distance as i16;
            },
            Up => {
                y -= instruction.distance as i16;
                for j in 0..instruction.distance {
                    let i = instruction.distance-j-1;
                    lagoon[(y-min_y+i as i16) as usize][(x-min_x) as usize] = '#'.to_string();
                }
            },
        }
    }


    //* I tried 2 algorithms, but they both failed
    // first, i've tried to go line by line, checking if we have passed a wall, and then updating a variable if we're in the trench or not.
    // this fails, because what if we move along the trench wall. we only see one line, so it is impossible to know if we're in the trench or not.
    // second, i've tried to go char by char, checking if the char has walls onn all sides. 
    // this fails because there are some caves that follow this rule, but also have an opening we can't see.
    // i might watch some youtube videos on how to solve this, but i'm not sure.
    // no star today
    // btw it's been 3.5 hours. this melted my brain multiple times.
    for y in 0..lagoon.len() {
        for x in 0..lagoon[0].len() {
            let vertical = lagoon.iter().map(|row| row[x].clone()).collect::<Vec<String>>();
            let horizontal = &lagoon[y];
            let mut topcount: i16 = -1;
            for i in 0..y+1 {
                if vertical[i] == "#" {
                    topcount = i as i16;
                }
            }
            let mut botcount: i16 = -1;
            for i in y..vertical.len() {
                if vertical[i] == "#" {
                    botcount = x as i16 + i as i16;
                }
            }
            let mut leftcount: i16 = -1;
            for i in 0..x+1 {
                if horizontal[i] == "#" {
                    leftcount = i as i16;
                }
            }
            let mut rightcount: i16 = -1;
            for i in x..horizontal.len() {
                if horizontal[i] == "#" {
                    rightcount = x as i16 + i as i16;
                }
            }
            if botcount != -1 && topcount != -1 && leftcount != -1 && rightcount != -1 {
                // if botcount - topcount > 1 && rightcount - leftcount > 1 {
                //     for i in topcount+1..botcount {
                //         for j in leftcount+1..rightcount {
                            lagoon[y][x] = "#".to_string();
                //         }
                //     }
                // }
            } else {
                // println!("{} {} {} {}", topcount, botcount, leftcount, rightcount);
            }
        }
    }

    // print the map
    for y in 0..lagoon.len() {
        // print!("{}: ", y);
        for x in 0..lagoon[0].len() {
            print!("{}", lagoon[y][x]);
        }
        println!();
    }

    lagoon.iter().flatten().filter(|&x| x == "#").count() as u32
}

fn main() {
    println!("Part 1: {}", part1());
    // aint no way im doing part 2
}