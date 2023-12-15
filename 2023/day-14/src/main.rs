extern crate util;
use util::{read_file, split_at_newline};

fn part1() -> usize {
    let file = split_at_newline(read_file("input.txt".to_string()));

    let mut cols: Vec<String> = vec![];
    for _x in 0..file[0].len() {
        cols.push("".to_string());
    }
    for y in 0..file.len() {
        for x in 0..file[0].len() {
            cols[x] = cols[x].clone() + &String::from(file[y].chars().nth(x).unwrap());
        }
    }

    for col in &mut cols {
        let mut startindex = 0;
        for i in 0..col.len() {
            match col.chars().nth(i).unwrap() {
                'O' => {
                    let mut replaced = false;
                    for n in startindex..(i+1) {
                        if !replaced && col.chars().nth(n).unwrap() == '.' {
                            col.replace_range( (n)..(n + 1), "O" );
                            col.replace_range( (i)..(i + 1), "." );
                            replaced = true;
                        }
                    }
                }
                '#' => {
                    startindex = i;
                }
                _ => {}
            }
        }
    }

    // changing it back to rows in vec
    let mut rows: Vec<String> = vec![];
    for _y in 0..cols[0].len() {
        rows.push("".to_string());
    }
    for x in 0..cols.len() {
        for y in 0..cols[0].len() {
            rows[y] = rows[y].clone() + &String::from(cols[x].chars().nth(y).unwrap());
        } 
    }

    // printing it out
    // for l in &rows {
    //     println!("{}", l);
    // }

    let mut sum = 0;
    for i in 0..rows.len() {
        let n = rows[i].matches("O").count();
        let diff = n * ((rows.len() - i));
        println!("{} * {} = {}", n, (rows.len() - i), diff);
        sum += diff;
    }
    sum
}

fn main() {
    println!("Part 1: {}", part1());
}
