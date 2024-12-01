use util::{read_file, split_at_newline};
fn part1() -> u32 {
    let file = split_at_newline(read_file("input.txt".to_string()));
    let mut num:i32 = 0;
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for line in file {
        // these two could go into one line
        let numbers_raw: Vec<&str> = line.split("   ").collect();
        let numbers: Vec<u32> = numbers_raw.iter().map(|x| x.parse::<u32>().unwrap()).collect();
        left.push(numbers[0]);
        right.push(numbers[1]);
    }
    left.sort();
    right.sort();
    let wip: Vec<i32> = left.iter().enumerate().map(|(i, &x)| right[i] as i32 - x as i32).collect();
    // i feel like this would have been doable with iter
    for n in wip {
        if n > 0 {num += n} else {num -= n}
    }
    num as u32
}

fn part2() -> u32 {
    let file = split_at_newline(read_file("input.txt".to_string()));
    let mut num:u32 = 0;
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for line in file {
        let numbers_raw: Vec<&str> = line.split("   ").collect();
        let numbers: Vec<u32> = numbers_raw.iter().map(|x| x.parse::<u32>().unwrap()).collect();
        left.push(numbers[0]);
        right.push(numbers[1]);
    }
    left.sort();
    for n in &left {
        let mut db: u32 = 0;
        for i in &right {
            if n == i {db+=1;}
        }
        num += n * db;
    }
    num
}

fn main() {
    let p1 = part1();
    let p2 = part2();
    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}
