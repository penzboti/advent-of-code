extern crate util;
use util::{read_file, split_at_newline};

const NUMBERS: &str = "1234567890";

fn part1() -> u32 {
    let lines = split_at_newline(read_file("input.txt".to_string()));

    let mut sum = 0;
    for i in 0..lines.len() {
        let line = &lines[i];

        let mut curnum = 0;
        let mut start = 0;
        let mut goodnum = false;
        for c in 0..line.len() {
            // indexing strings
            // https://stackoverflow.com/a/24542502/12706133
            let startc = line.chars().nth(c).unwrap();
            if NUMBERS.contains(startc) {
                if curnum == 0 { start = c; }
                let num = startc.to_string().parse::<u32>().unwrap_or(0);
                curnum = curnum*10 + num;
            } 

            if curnum != 0 && (!NUMBERS.contains(startc) || c == (line.len() - 1)) {
                let mut lines_to_check: Vec<&String> = vec![];
                let mut start_index = start;
                let end_index = c;

                let curline = line.chars().map(|c| if NUMBERS.contains(c) { '.'.to_string() } else { c.to_string() }).collect::<Vec<String>>().join("").to_string();
                lines_to_check.push(&curline);
                
                if i != 0 {
                    let prev_line = &lines[i - 1];
                    lines_to_check.push(prev_line);
                }
                if i != lines.len() - 1 {
                    let next_line = &lines[i + 1];
                    lines_to_check.push(next_line);
                }
                
                if start != 0 {
                    start_index -= 1;
                }
                
                for l in lines_to_check {
                    for j in start_index..(end_index + 1) {
                        let prev_c = l.chars().nth(j).unwrap();
                        if prev_c != '.' {
                            goodnum = true;
                        }
                    }
                }


                if goodnum {
                    sum += curnum;
                }
                curnum = 0;
                start = 0;
                goodnum = false;
            }
        }
        
    }
    sum
}


#[derive(Debug)]
struct Gear {
    nums: Vec<u32>,
    x: u32,
    y: u32,
}

fn part2() -> u32 {
    let lines = split_at_newline(read_file("input.txt".to_string()));

    let mut sum = 0;
    let mut gears: Vec<Gear> = vec![];
    for i in 0..lines.len() {
        let line = &lines[i];

        let mut curnum = 0;
        let mut start = 0;
        for c in 0..line.len() {
            let startc = line.chars().nth(c).unwrap();
            if NUMBERS.contains(startc) {
                if curnum == 0 { start = c; }
                let num = startc.to_string().parse::<u32>().unwrap_or(0);
                curnum = curnum*10 + num;
            } 

            if curnum != 0 && (!NUMBERS.contains(startc) || c == (line.len() - 1)) {
                let mut lines_to_check: Vec<&String> = vec![];
                let mut start_index = start;
                let end_index = c;
                let mut relative_index: i32 = 0;

                let curline = line.chars().map(|c| if NUMBERS.contains(c) { '.'.to_string() } else { c.to_string() }).collect::<Vec<String>>().join("").to_string();
                
                if i != 0 {
                    let prev_line = &lines[i - 1];
                    lines_to_check.push(prev_line);
                    relative_index -= 1;
                }
                lines_to_check.push(&curline);
                if i != lines.len() - 1 {
                    let next_line = &lines[i + 1];
                    lines_to_check.push(next_line);
                }
                
                if start != 0 {
                    start_index -= 1;
                }
                
                for l in 0..lines_to_check.len() {
                    for j in start_index..(end_index + 1) {
                        let prev_c = lines_to_check[l].chars().nth(j).unwrap();
                        if prev_c == '*' {
                            let mut exists = false;
                            // i didn't use this, but this also could've been a good solution
                            // https://www.becomebetterprogrammer.com/rust-find-index-of-element-in-array/
                            let index = (i as i32 + l as i32 + relative_index) as u32;
                            for g in &mut gears {
                                if g.x == j as u32 && g.y == index as u32 {
                                    exists = true;
                                    g.nums.push(curnum);
                                }
                            }
                            if !exists {
                                gears.push(Gear { nums: vec![curnum], x: j as u32, y: index as u32 });
                            }
                        }
                    }
                }

                curnum = 0;
                start = 0;
            }
        }
    }
    
    for g in &gears {
        if g.nums.len() == 2 {
            sum += g.nums[0] * g.nums[1];
        }
    }
    sum
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
