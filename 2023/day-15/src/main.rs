extern crate util;
use util::read_file;

fn part1() -> u32 {
    let file = read_file("input.txt".to_string());
    let steps = file.split(',');
    let mut hashes: Vec<u32> = vec![];
    for s in steps {
        let mut n = 0;
        for c in s.chars() {
            // https://www.includehelp.com/rust/print-the-ascii-value-of-a-character.aspx
            n += c as u32;
            n *= 17;
            n = n % 256;
        }
        hashes.push(n);
    }

    let sum = hashes.iter().sum();
    sum
}

#[derive(Debug)]
enum Operation {
    Add,
    Remove,
}
use crate::Operation::*;

#[derive(Debug)]
struct Step {
    op: crate::Operation,
    step: String,
}

#[derive(Debug)]
struct Lens {
    label: String,
    val: u8,
}

#[derive(Debug)]
struct Box {
    i: u8,
    lenses: Vec<Lens>,
}

fn part2() -> usize {
    let file = read_file("input.txt".to_string());
    let stepsraw = file.split(',').collect::<Vec<&str>>();

    let steps = stepsraw.iter()
        .map(|&x| Step { 
            step: x.to_string(),
            // checked if it is possible
            // https://www.reddit.com/r/rust/comments/ezqeg3/rustfmt_with_singleline_if_else_expression/?share_id=iZcxRQPXz8qlJRl7S_DAe&utm_content=1&utm_medium=android_app&utm_name=androidcss&utm_source=share&utm_term=1
            op: if x.contains('=') { Add } else { Remove }
        })
        .collect::<Vec<Step>>();

    let mut boxes: Vec<Box> = vec![];
    for i in 0..=255 {
        boxes.push(Box { i: i, lenses: vec![] });
    }

    for step in steps {
        let mut hash = 0;
        match step.op {
            Add => {
                let label = step.step.split('=').next().unwrap(); 
                let val = step.step.split('=').next_back().unwrap().parse::<u8>().unwrap();
                for c in label.chars() {
                    hash += c as u32;
                    hash *= 17;
                    hash = hash % 256;
                }

                if boxes[hash as usize].lenses.iter().filter(|&x| x.label == label).collect::<Vec<&Lens>>().len() != 0 {
                    for l in &mut boxes[hash as usize].lenses {
                        if l.label == label {
                            l.val = val;
                        }
                    }
                } else {
                    let locallens = Lens { label: label.to_string(), val: val };
                    boxes[hash as usize].lenses.push(locallens);
                }

            },
            Remove => {
                let label = step.step.split('-').next().unwrap(); 
                for c in label.chars() {
                    hash += c as u32;
                    hash *= 17;
                    hash = hash % 256;
                }
                boxes[hash as usize].lenses.retain(|x| x.label != label);
            },
        }
    }

    boxes.retain(|x| x.lenses.len() != 0);

    let mut sum = 0;
    for b in 0..boxes.len() {
        for l in 0..boxes[b].lenses.len() {
            let val = boxes[b].lenses[l].val as usize;
            let diff = (boxes[b].i as usize + 1) * (l+1) * val;
            sum += diff;
        }
    }
    sum
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
