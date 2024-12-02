use util::{split_at_newline, read_file};

fn part1() -> u32 {
    let mut n: u32 = 0;
    let file = split_at_newline(read_file("input.txt".to_owned()));
    let numbers: Vec<Vec<u32>> = file.iter()
        .map(|x| x.split(" ").collect::<Vec<&str>>().iter().map(|&y| y.parse::<u32>().unwrap() ).collect::<Vec<u32>>() )
        .collect();
    for line in numbers {
        let mut pass = true;
        let mut increasing = false;
        let mut prev = line[0];
        // this works because it doesnt matter which comes first when its changing direction
        if prev < line[1] {
            increasing = true;
        }
        for i in 1..(line.len()) {
            if !pass {break;}
            let curr = line[i];
            let diff = (prev as i32 - curr as i32).abs();
            if diff > 3 || diff < 1 { pass = false; }
            if prev > curr && increasing { pass = false; }
            if prev < curr && !increasing { pass = false; }
            prev = curr;
        }
        if pass {n+=1;}
    }
    n
}

fn part2() -> u32 {
    let mut n: u32 = 0;
    let file = split_at_newline(read_file("input.txt".to_owned()));
    let numbers: Vec<Vec<u32>> = file.iter()
        .map(|x| x.split(" ").collect::<Vec<&str>>().iter().map(|&y| y.parse::<u32>().unwrap() ).collect::<Vec<u32>>() )
        .collect();
    for line in numbers {
        let mut pass = true;
        let mut increasing = false;
        // remember the comment i made in part 1?
        // now it matters.
        // my mistake was only checking the first two numbers for decrease / increase
        let mut test_increasing = vec![];
        for k in 1..line.len() {
            let prev = line[k-1];
            let curr = line[k];
            // increasing
            if prev < curr {
                test_increasing.push(1);
            } else {
                // decreasing
                test_increasing.push(-1);
            }
        }
        let testnum = test_increasing.iter().sum::<i32>();
        if testnum > 0 {
            increasing = true;
        }
        let mut prev = line[0];
        for i in 1..(line.len()) {
            if !pass {break;}
            let curr = line[i];
            let diff = (prev as i32 - curr as i32).abs();
            if diff > 3 || diff < 1 { pass = false; }
            if prev > curr && increasing { pass = false; }
            if prev < curr && !increasing { pass = false; }
            prev = curr;
        }

        // if its good before removing, we continue with the next one
        if pass {n+=1; continue;}
        // if its not, we remove its elements one by one and then check if its good
        for i in 0..line.len() {
            let mut newline = line.clone();
            newline.remove(i);
            let mut localpass = true;
            
            prev = newline[0];
            for i in 1..(newline.len()) {
                if !localpass {break;}
                let curr = newline[i];
                let diff = (prev as i32 - curr as i32).abs();
                if diff > 3 || diff < 1 { localpass = false; }
                if prev > curr && increasing { localpass = false; }
                if prev < curr && !increasing { localpass = false; }
                prev = curr;
            }
            if localpass {n+=1; break;}
        }
    }
    n
}

fn main() {
    let p1 = part1();
    println!("part 1: {}", p1);
    let p2 = part2();
    println!("part 2: {}", p2);
}