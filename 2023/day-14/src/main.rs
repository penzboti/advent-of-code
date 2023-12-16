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
                            // https://stackoverflow.com/questions/66661118/how-do-i-change-characters-at-a-specific-index-within-a-string-in-rust
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
        // https://programming-idioms.org/idiom/82/count-substring-occurrences/797/rust
        let n = rows[i].matches("O").count();
        let diff = n * ((rows.len() - i));
        sum += diff;
    }
    sum
}

fn part2() -> usize {
    let file = split_at_newline(read_file("demo1.txt".to_string()));

    // at the start we turn it into east so that the algorithm doesnt break
    // but at the start it is west, so like we invert it fully, then at the start it just goes like normal
    let mut rows: Vec<String> = file.clone();
    // let mut rows: Vec<String> = vec![];
    // for i in 0..file[0].len() {
    //     rows.push("".to_string());
    // }
    // for y in 0..file.len() {
    //     for x in 0..file[0].len() {
    //         rows[x] = rows[x].clone() + &String::from(file[y].chars().nth(x).unwrap());
    //     }
    // }
    // for i in 0..file.len() {
    //     rows.push(file[i].chars().rev().collect::<String>());
    // }
    // rows.reverse();

    println!("start");
        for l in &rows {
            println!("{}", l);
        }

    let mut index = 0;
    loop {
        // if index == 1000000000 { break; }
        if index == 2 { break; }
        let mut relativerows: Vec<String> = vec![];
        for _x in 0..rows[0].len() {
            relativerows.push("".to_string());
        }
        println!("INDEX HERE!: {}", index);
        // we turn it 90 degres to the left
        //* THIS DOESNT ACTUALLY FLIP IT
        // or at least not in a way i want it to flip it
        // for y in 0..rows.len() {
        //     for x in 0..rows[0].len() {
        //         relativerows[x] = relativerows[x].clone() + &String::from(rows[y].chars().nth(x).unwrap());
        //     }
        // }

        // new flip algorithm here
        // and it works :3
        let length = rows[0].len()-1;
        for y in 0..rows.len() {
            for x in 0..(length+1) {
                relativerows[length - x] = relativerows[length - x].clone() + &String::from(rows[y].chars().nth(x).unwrap());
            }
        }

        println!("flipped");
        for l in &relativerows {
            println!("{}", l);
        }
    
        // for row in &mut relativerows {
        //     let mut startindex = 0;
        //     for i in 0..row.len() {
        //         match row.chars().nth(i).unwrap() {
        //             'O' => {
        //                 let mut replaced = false;
        //                 for n in startindex..(i+1) {
        //                     if !replaced && row.chars().nth(n).unwrap() == '.' {
        //                         row.replace_range( (n)..(n + 1), "O" );
        //                         row.replace_range( (i)..(i + 1), "." );
        //                         replaced = true;
        //                     }
        //                 }
        //             }
        //             '#' => {
        //                 startindex = i;
        //             }
        //             _ => {}
        //         }
        //     }
        // }
    
        let localrows: Vec<String> = relativerows.clone();
        // let mut localrows: Vec<String> = vec![];
        // for _y in 0..relativerows[0].len() {
        //     localrows.push("".to_string());
        // }
        // for x in 0..relativerows.len() {
        //     for y in 0..relativerows[0].len() {
        //         localrows[y] = localrows[y].clone() + &String::from(relativerows[x].chars().nth(y).unwrap());
        //     } 
        // }

        rows = localrows;
        index += 1;
    }

        println!("end");
        for l in &rows {
            println!("{}", l);
        }

    let mut sum = 0;
    for i in 0..rows.len() {
        let n = rows[i].matches("O").count();
        let diff = n * ((rows.len() - i));
        sum += diff;
    }
    sum
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
