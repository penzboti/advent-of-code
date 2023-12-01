extern crate util;
use util::{read_file, split_at_newline};

const NUMBERS: &str = "0123456789";
const BASE: u32 = 10;

fn part1() -> u32 {
    let file = split_at_newline(read_file("input.txt".to_string()));
    let mut sum = 0;
    for line in file {
        let mut nums_inline: Vec<u32> = vec![];

        for i in 0..(line.len()) {
            let c = line.chars().nth(i).unwrap();
            if NUMBERS.contains(c) {
                // this is how you convert a char to an int
                // https://stackoverflow.com/a/43985962
                nums_inline.push(c.to_digit(BASE).unwrap());
            }
        }
        if nums_inline.len() == 1 {
            sum += nums_inline[0]*11;
        } else {
            sum += nums_inline[0]*10 + nums_inline[nums_inline.len()-1];
        }
    }
    sum
}

const SPELLED_NUMS: &[(&str, u32); 9] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9)
];
fn part2() -> u32 {

    let file = split_at_newline(read_file("input.txt".to_string()));
    
    let mut sum = 0;
    for line in file {
        let mut nums_inline: Vec<(u32, u32)> = vec![];

        for tup in SPELLED_NUMS {
            if line.contains(tup.0) {
                // mapping is really cool
                // https://stackoverflow.com/a/69425503
                let v: Vec<_> = line.match_indices(tup.0).map(|(i, _)|(tup.1, i)).collect();
                v.iter().for_each(|(n, i)| {
                    nums_inline.push((*n, *i as u32));
                });
            }
        }

        for i in 0..(line.len()) {
            let c = line.chars().nth(i).unwrap();
            if NUMBERS.contains(c) {
                nums_inline.push((c.to_digit(BASE).unwrap(), i as u32));
            }
        }

        if nums_inline.len() == 1 {
            sum += nums_inline[0].0*11;
        } else {
            let mut lowest: u32 = (line.len()-1).try_into().unwrap();
            let mut highest: u32 = 0;
            // uhm i guess usize has to go here
            // https://stackoverflow.com/a/65262108
            let mut lowest_index: usize = 0;
            let mut highest_index: usize = 0;
            for i in 0..(nums_inline.len()) {
                if nums_inline[i].1 <= lowest {
                    lowest = nums_inline[i].1;
                    lowest_index = i;
                }
                if nums_inline[i].1 >= highest {
                    highest = nums_inline[i].1;
                    highest_index = i;
                }
            }
            let lowtup = nums_inline[lowest_index];
            let hightup = nums_inline[highest_index];
            let sumdiff = lowtup.0*10 + hightup.0;
            sum += sumdiff;
        }
    }
    sum
}


fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}