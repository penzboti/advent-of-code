extern crate util;
use util::{read_file, split_at_newline};

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}
use crate::Direction::*;

#[derive(Debug, Clone, PartialEq)]
struct Ray {
    x: i16,
    y: i16,
    direction: Direction,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Tile {
    x: i16,
    y: i16,
}

fn part1() -> u32 {
    let file = split_at_newline(read_file("input.txt".to_string()));
    let mut rays: Vec<Ray> = Vec::new();
    let mut firstrays: Vec<Ray> = Vec::new();
    // found it in some random code somewhere that this exists, so i thought i might use it
    // https://www.programiz.com/rust/hashset
    let mut tiles: HashSet<Tile> = HashSet::new();
    rays.push(Ray{x: -1, y: 0, direction: East});
    let mut i = 0;
    while i != rays.len() {
        for j in i..rays.len() {
            i += 1;
            let mut topush: Vec<Ray> = Vec::new();
            loop {
                let ray = &mut rays[j];
                match ray.direction {
                    North => {ray.y-=1},
                    South => {ray.y+=1},
                    East => {ray.x+=1},
                    West => {ray.x-=1},
                }
                if ray.x < 0 || ray.y < 0 {
                    break;
                }
                if ray.y as usize >= file.len() || ray.x as usize >= file[ray.y as usize].len() {
                    break;
                }
                tiles.insert(Tile{x: ray.x, y: ray.y});
                match file[ray.y as usize].chars().nth(ray.x as usize).unwrap() {
                    '/' => {
                        let newdir = match ray.direction {
                            North => East,
                            South => West,
                            East => North,
                            West => South,
                        };
                        topush.push(Ray{x: ray.x, y: ray.y, direction: newdir});
                        break;
                    },
                    '\\' => {
                        let newdir = match ray.direction {
                            North => West,
                            South => East,
                            East => South,
                            West => North,
                        };
                        topush.push(Ray{x: ray.x, y: ray.y, direction: newdir});
                        break;
                    },
                    '|' => {
                        match ray.direction {
                            North => {},
                            South => {},
                            East | West => {topush.push(Ray{x: ray.x, y: ray.y, direction: South}); topush.push(Ray{x: ray.x, y: ray.y, direction: North}); break;},
                        }
                    },
                    '-' => {
                        match ray.direction {
                            North | South => {topush.push(Ray{x: ray.x, y: ray.y, direction: East}); topush.push(Ray{x: ray.x, y: ray.y, direction: West}); break;},
                            East => {},
                            West => {},
                        }
                    },
                    _ => {},
                }
            }
            for ray in &topush {
                if !firstrays.contains(&ray) {
                    firstrays.push(ray.clone());
                    rays.push(ray.clone());
                }
            }
        }
    }
    // printing the end out
    // for i in 0..file.len() {
    //     for j in 0..file[i].len() {
    //         if tiles.contains(&Tile{x: j as i16, y: i as i16}) {
    //             if topush.len() != 0 && j == topush.last().unwrap().x as usize && i == topush.last().unwrap().y as usize {
    //                 print!("X");
    //             } else {
    //                 print!("#");
    //             }
    //         }
    //         else {
    //             print!(".");
    //         }
    //     }
    //     println!("");
    // }

    tiles.len() as u32
}

fn part2() -> u32 {
    let file = split_at_newline(read_file("input.txt".to_string()));
    // basically part 1, bundled into a function
    fn run_instance(x: i16, y: i16, direction: Direction, fileraw: &Vec<String>) -> u32 {
        let file = fileraw.clone();
        let mut rays: Vec<Ray> = Vec::new();
        let mut firstrays: Vec<Ray> = Vec::new();
        let mut tiles: HashSet<Tile> = HashSet::new();
        rays.push(Ray{x: x, y: y, direction: direction});
        let mut i = 0;
        while i != rays.len() {
            for j in i..rays.len() {
                i += 1;
                let mut topush: Vec<Ray> = Vec::new();
                loop {
                    let ray = &mut rays[j];
                    match ray.direction {
                        North => {ray.y-=1},
                        South => {ray.y+=1},
                        East => {ray.x+=1},
                        West => {ray.x-=1},
                    }
                    if ray.x < 0 || ray.y < 0 {
                        break;
                    }
                    if ray.y as usize >= file.len() || ray.x as usize >= file[ray.y as usize].len() {
                        break;
                    }
                    tiles.insert(Tile{x: ray.x, y: ray.y});
                    match file[ray.y as usize].chars().nth(ray.x as usize).unwrap() {
                        '/' => {
                            let newdir = match ray.direction {
                                North => East,
                                South => West,
                                East => North,
                                West => South,
                            };
                            topush.push(Ray{x: ray.x, y: ray.y, direction: newdir});
                            break;
                        },
                        '\\' => {
                            let newdir = match ray.direction {
                                North => West,
                                South => East,
                                East => South,
                                West => North,
                            };
                            topush.push(Ray{x: ray.x, y: ray.y, direction: newdir});
                            break;
                        },
                        '|' => {
                            match ray.direction {
                                North => {},
                                South => {},
                                East | West => {topush.push(Ray{x: ray.x, y: ray.y, direction: South}); topush.push(Ray{x: ray.x, y: ray.y, direction: North}); break;},
                            }
                        },
                        '-' => {
                            match ray.direction {
                                North | South => {topush.push(Ray{x: ray.x, y: ray.y, direction: East}); topush.push(Ray{x: ray.x, y: ray.y, direction: West}); break;},
                                East => {},
                                West => {},
                            }
                        },
                        _ => {},
                    }
                }
                for ray in &topush {
                    if !firstrays.contains(&ray) {
                        firstrays.push(ray.clone());
                        rays.push(ray.clone());
                    }
                }
            }
        }
        tiles.len() as u32
    }

    let mut energizations: Vec<u32> = Vec::new();
    for i in 0..file.len() {
        energizations.push(run_instance(-1 as i16, i as i16, East, &file));
        energizations.push(run_instance(file[0].len() as i16 + 1, i as i16, West, &file));
    }
    for j in 0..file[0].len() {
        energizations.push(run_instance(j as i16, -1 as i16, South, &file));
        energizations.push(run_instance(j as i16, file.len() as i16 + 1, North, &file));
    }
    energizations.sort();
    energizations.last().unwrap().clone()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}