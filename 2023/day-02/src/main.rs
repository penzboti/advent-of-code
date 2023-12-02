extern crate util;
use util::{read_file, split_at_newline};

use std::collections::HashMap;

fn part1() -> u32 {
    let lines: Vec<String> = split_at_newline(read_file("input.txt".to_string()));

    let limit: HashMap<&str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);
    
    let mut sum = 0;
    for line in lines {
        let split_info: Vec<&str> = line.split(": ").collect();
        let linenum = split_info[0].split(' ').next_back().unwrap_or("x").to_string().parse::<u32>().unwrap_or(0);
        let mut goodline = true;
        // print!("Game {}: ", linenum);
        
        // some split help
        // https://stackoverflow.com/questions/57839231/why-cant-i-call-next-back-on-a-split
        let chunks: Vec<&str> = split_info[1].split("; ").collect::<Vec<_>>();
        for chunk in &chunks {
            let turn = chunk.split(", ").collect::<Vec<_>>();
            for ball in turn {
                let ball_info: Vec<&str> = ball.split(' ').collect::<Vec<_>>();
                let ballnum = ball_info[0].to_string().parse::<u32>().unwrap_or(0);
                let ballcolor = ball_info[1];
                
                if limit[ballcolor] < ballnum { goodline = false; }
            }
        }
        if goodline { sum += linenum; }
    }
    sum
}

struct FewestBalls {
    red: u32,
    green: u32,
    blue: u32
}

fn part2() -> u32 {
    let lines: Vec<String> = split_at_newline(read_file("input.txt".to_string()));

    let mut sum = 0;
    for line in lines {
        let split_info: Vec<&str> = line.split(": ").collect();
        // print!("Game {}: ", linenum);

        let mut fewest = FewestBalls { red: 0, green: 0, blue: 0 };
        
        let chunks: Vec<&str> = split_info[1].split("; ").collect::<Vec<_>>();
        for chunk in &chunks {
            let turn = chunk.split(", ").collect::<Vec<_>>();
            for ball in turn {
                let ball_info: Vec<&str> = ball.split(' ').collect::<Vec<_>>();
                let ballnum = ball_info[0].to_string().parse::<u32>().unwrap_or(0);
                let ballcolor = ball_info[1];

                match ballcolor {
                    "red" => { if fewest.red < ballnum { fewest.red = ballnum; } },
                    "green" => { if fewest.green < ballnum { fewest.green = ballnum; } },
                    "blue" => { if fewest.blue < ballnum { fewest.blue = ballnum; } },
                    _ => ()
                }
                
            }
        }

        let diff = fewest.red * fewest.green * fewest.blue;
        sum += diff;
    }
    sum
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
