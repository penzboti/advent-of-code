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
    //* if it wasn't 1 trillion times, this program could've finished it. Sadly, it is 1 trillion times.
    // im not smart enough to do this efficiently yet
    let file = split_at_newline(read_file("demo1.txt".to_string()));

    // at the start we turn it into east so that the algorithm doesnt break
    // but at the start it is west, so like we invert it fully, then at the start it just goes like normal
    // let mut rows: Vec<String> = file.clone();
    let mut rows: Vec<String> = vec![];
    // for i in 0..file[0].len() {
    //     rows.push("".to_string());
    // }
    // for y in 0..file.len() {
    //     for x in 0..file[0].len() {
    //         rows[x] = rows[x].clone() + &String::from(file[y].chars().nth(x).unwrap());
    //     }
    // }

    // this is the invertion algorithm :3
    // https://www.techiediaries.com/reverse-string-rust/
    // and the vector's reverse function just popped up, so i used it
    for i in 0..file.len() {
        rows.push(file[i].chars().rev().collect::<String>());
    }
    rows.reverse();

    // println!("start");
    // for l in &rows {
    //     println!("{}", l);
    // }

    let mut index: u32 = 0;
    loop {
        if index == 4000000000 { break; }
        if index % 100000 == 0 {
            // https://stackoverflow.com/a/40348109/12706133
            let percent: f32 = (index as f32 / 4000000000.0) * 100.0;
            println!("~{:.4}%", percent); }
        let mut relativerows: Vec<String> = vec![];
        for _x in 0..rows[0].len() {
            relativerows.push("".to_string());
        }
        // println!("\nINDEX HERE!: {}", index);

        // new flip algorithm here, and it does flip it 90 degrees, but to the right
        // and it works :3
        // let length = rows[0].len()-1;
        // for y in 0..rows.len() {
        //     for x in 0..(length+1) {
        //         relativerows[length - x] = relativerows[length - x].clone() + &String::from(rows[y].chars().nth(x).unwrap());
        //     }
        // }

        // and maybe invertion does the job for us, so that when we flip right and invert it, it is like flipping left!
        // IT DOES!!!!
        // let mut flippedrows: Vec<String> = vec![];
        // for l in &relativerows {
        //     flippedrows.push(l.chars().rev().collect::<String>());
        // }
        // flippedrows.reverse();
        // relativerows = flippedrows;
        
        // lets flip it to the left this time
        for y in 0..rows.len() {
            for x in 0..rows[0].len() {
                relativerows[x] = relativerows[x].clone() + &String::from(rows[rows.len() - 1 - y].chars().nth(x).unwrap());
            }
        }
        
        for row in &mut relativerows {
            let mut startindex = 0;
            for i in 0..row.len() {
                match row.chars().nth(i).unwrap() {
                    'O' => {
                        let mut replaced = false;
                        for n in startindex..(i+1) {
                            if !replaced && row.chars().nth(n).unwrap() == '.' {
                                row.replace_range( (n)..(n + 1), "O" );
                                row.replace_range( (i)..(i + 1), "." );
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
    
        rows = relativerows;
        index += 1;
    }

    let mut flippedrows: Vec<String> = vec![];
    for l in &rows {
        flippedrows.push(l.chars().rev().collect::<String>());
    }
    flippedrows.reverse();

    rows = flippedrows;
    
        println!("\nend");
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
