extern crate util;
use util::{read_file, split_at_newline};

#[derive(Debug)]
struct Sequence {
    original: Vec<i32>,
    reductions: Vec<Vec<i32>>,
    nextval: i32,
}

fn part1() -> i32 {
    let file = split_at_newline(read_file("input.txt".to_string()));
    let numbers: Vec<Vec<i32>> = file
        .iter()
        .map(|line| line.split(" ").map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();
    let mut sequences = numbers.iter().map(|x| Sequence { original: x.to_vec(), reductions: Vec::new(), nextval: 0 })
        .collect::<Vec<Sequence>>();

    for s in &mut sequences {
        let mut canbreak = false;
        loop {
            if canbreak { break; }
            // the reduction part
            let mut nextreduc: Vec<i32> = vec![];
            let vec_to_reduc = s.reductions.last()
                .unwrap_or(&s.original);
            for n in 0..vec_to_reduc.len() {
                if n != vec_to_reduc.len() - 1 {
                    nextreduc.push(vec_to_reduc[n + 1] - vec_to_reduc[n]);
                }
            }

            if nextreduc.iter().filter(|n| **n != 0).collect::<Vec<&i32>>().len() == 0 { canbreak = true; }
            s.reductions.push(nextreduc);
        }
    }
    
    let mut sum = 0;
    
    for s in &mut sequences {
        s.nextval = s.reductions.iter().map(|x| x.iter().last().unwrap()).sum::<i32>() + s.original.last().unwrap();
        sum += s.nextval;
    }
    
    sum
}

fn part2() -> i32 {
    let file = split_at_newline(read_file("input.txt".to_string()));
    let numbers: Vec<Vec<i32>> = file
        .iter()
        .map(|line| line.split(" ").map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();
    let mut sequences = numbers.iter().map(|x| Sequence { original: x.to_vec(), reductions: Vec::new(), nextval: 0 })
        .collect::<Vec<Sequence>>();

    for s in &mut sequences {
        let mut canbreak = false;
        loop {
            if canbreak { break; }
            let mut nextreduc: Vec<i32> = vec![];
            let vec_to_reduc = s.reductions.last()
                .unwrap_or(&s.original);
            for n in 0..vec_to_reduc.len() {
                if n != vec_to_reduc.len() - 1 {
                    nextreduc.push(vec_to_reduc[n + 1] - vec_to_reduc[n]);
                }
            }

            if nextreduc.iter().filter(|n| **n != 0).collect::<Vec<&i32>>().len() == 0 { canbreak = true; }
            s.reductions.push(nextreduc);
        }
    }
    
    let mut sum = 0;
    
    for s in &mut sequences {
        let mut diff = 0;
        for n in 0..s.reductions.len()-1 {
            let i = s.reductions.len() - n;
            diff = s.reductions[i-2][0] - diff;
        }
        s.nextval = s.original[0] - diff;

        sum += s.nextval;
    }
    
    sum
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}
