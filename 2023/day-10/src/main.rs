extern crate util;
use util::{read_file, split_at_newline};

#[derive(Debug, Clone)]
#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum Direction {
    NW, // J
    Vertical, // |
    NE, // L
    Horizontal, // -
    SE, // F
    SW, // 7
    Empty, // .
}

use crate::Direction::*;
impl Direction {
    fn matchdir(c: char) -> crate::Direction {
        match c {
            'J' => NW,
            '|' => Vertical,
            'L' => NE,
            '-' => Horizontal,
            'F' => SE,
            '7' => SW,
            _ => Empty,
        }
    }
}

#[derive(Debug, Clone)]
struct Pos {
    x: usize,
    y: usize,
    direction: Direction,
    distance: u32, // S = 0
}

fn part1() -> u32 {
    let file = split_at_newline(read_file("demo1.txt".to_string()));
    let mut pipe: Vec<Pos> = vec![];
    for i in 0..file.len() {
        match file[i].find('S') {
            Some(index) => {
                // we should get the direction of S here
                // we should also get the 2 pipe connecting tho this one
                let mut spos = Pos { x: index, y: i, direction: Empty, distance: 0 };
                
                // using vector as [n, e, s, w]
                let x = spos.x; let y = spos.y;
                let mut char_around: Vec<char> = vec![];
                if y != 0 { char_around.push( file[y-1].chars().nth(x).unwrap() ); }
                if x != file[0].len() { char_around.push( file[y].chars().nth(x+1).unwrap() ); }
                if y != file.len() { char_around.push( file[y+1].chars().nth(x).unwrap() ); }
                if x != 0 { char_around.push( file[y].chars().nth(x-1).unwrap() ); }

                let mut dir_around: Vec<Direction> = vec![];
                for c in &char_around {
                    dir_around.push( crate::Direction::matchdir(*c) );
                }
                
                for d in 0..dir_around.len() {
                    match d {
                        0 => {
                            if vec![Vertical, NW, NE].contains( &dir_around[d] ) {
                                pipe.push( Pos { x: index, y: i-1, direction: dir_around[d].clone(), distance: 1 } )
                            }
                        }
                        1 => {
                            if vec![Horizontal, NE, SE].contains( &dir_around[d] ) {
                                pipe.push( Pos { x: index+1, y: i, direction: dir_around[d].clone(), distance: 1 } )
                            }
                        }
                        2 => {
                            if vec![Vertical, SW, SE].contains( &dir_around[d] ) {
                                pipe.push( Pos { x: index, y: i+1, direction: dir_around[d].clone(), distance: 1 } )
                            }
                        }
                        3 => {
                            if vec![Horizontal, NW, SW].contains( &dir_around[d] ) {
                                pipe.push( Pos { x: index-1, y: i, direction: dir_around[d].clone(), distance: 1 } )
                            }
                        }
                        _ => {},
                    }
                }

                pipe.push(spos.clone());
            },
            None => {},
        }
    }

    println!("{:?}", pipe);

    let mut prevlast_x = 0;
    let mut prevlast_y = 0;
    loop {
        let last_pos = pipe.last().unwrap().clone();
        let x = last_pos.x; let y = last_pos.y;
        // let mut n = Empty; let mut e = Empty; let mut s = Empty; let mut w = Empty;
        // if y == 0 { n = file[y-1].chars().nth(x); }
        // if x == file[0].chars().len() { e = file[y].chars().nth(x+1); }
        // if y == file.len() { s = file[y+1].chars().nth(x); }
        // if x == 0 { w = file[y].chars().nth(x-1); }
        match last_pos.direction {
            _ => {},
        }
        break;
    }

    //TODO -list
    // find point S & also assign a direction to it
    // follow path, and go around
    // assign each point to struct Pos,
    // then generate distance numbers
    // then find max

    let mut sum = 0;
    sum
}

fn main() {
    println!("part 1: {}", part1());
}