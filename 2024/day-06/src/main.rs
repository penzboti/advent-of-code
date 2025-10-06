use util::{read_file, split_at_newline};

#[derive (Clone, Debug, Copy, PartialEq)]
enum Direction {
    North = 1,
    South = 2,
    East = 4,
    West = 8
}

fn part1() -> u32 {
    let file = split_at_newline(read_file("input.txt".to_string()));
    let mut table: Vec<Vec<char>> = file.iter().map(|x| x.split("").collect::<Vec<&str>>().iter().map(|y| y.chars().nth(0).unwrap_or(' ')).filter(|&y| y != ' ').collect::<Vec<char>>()).collect();

    let mut posx: i32 = table.iter().map(|x| x.iter().position(|&y| y == '^').unwrap_or(0)).sum::<usize>() as i32;
    let mut posy: i32 = table.iter().position(|x| x[posx as usize] == '^').unwrap() as i32;
    let mut dir = Direction::North;

    loop {
        table[posy as usize][posx as usize] = 'X';

        let (changey,changex) = match dir {
            Direction::North => (1,0),
            Direction::East => (0,1),
            Direction::South => (-1,0),
            Direction::West => (0,-1)
        };
        let currx = posx + changex;
        let curry = posy - changey;

        if currx < 0 || curry < 0 {break;}
        let row = table.iter().nth(curry as usize);
        if row.is_none() {break;}
        let c = row.unwrap().iter().nth(currx as usize);
        if c.is_none() {break;}

        let c = c.unwrap();
        match c {
            '#' => {dir = match dir {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North
            }},
            _ => {
                posx = currx; posy = curry;
            }
        }
    }
    table.iter().map(|x| x.iter().filter(|&&y| y == 'X').count()).sum::<usize>() as u32
}

// part 2 idea:
// every time we step, we go trough the maze beforehand, and check for any positions we crossed that we have visited before
// we check for the direction if its the same
// and if this version fails, make sure to check corners aswell
// and if its too slow, when we go beforehand and skip to until we met a previous position
fn part2() -> u32 {
    let mut n = 0;
    let file = split_at_newline(read_file("test2.txt".to_string()));
    let mut table: Vec<Vec<char>> = file.iter().map(|x| x.split("").collect::<Vec<&str>>().iter().map(|y| y.chars().nth(0).unwrap_or(' ')).filter(|&y| y != ' ').collect::<Vec<char>>()).collect();

    let mut posx: i32 = table.iter().map(|x| x.iter().position(|&y| y == '^').unwrap_or(0)).sum::<usize>() as i32;
    let mut posy: i32 = table.iter().position(|x| x[posx as usize] == '^').unwrap() as i32;

    // let startx = posx.clone(); let starty = posy.clone();
    let mut dir = Direction::North;

    table[posy as usize][posx as usize] = '1';

    loop {
        // https://stackoverflow.com/questions/28028854/how-do-i-match-enum-values-with-an-integer
        let mut dirnum = dir.clone() as u32;

        let (changey,changex) = match dir {
            Direction::North => (1,0),
            Direction::East => (0,1),
            Direction::South => (-1,0),
            Direction::West => (0,-1)
        };
        let currx = posx + changex;
        let curry = posy - changey;

        if currx < 0 || curry < 0 {break;}
        let row = table.iter().nth(curry as usize);
        if row.is_none() {break;}
        let c = row.unwrap().iter().nth(currx as usize);
        if c.is_none() {break;}

        let c = c.unwrap();

        // println!("{c:?}");

        match c {
            '#' => {dir = match dir {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North
            }; continue;},
            _ => {
                posx = currx; posy = curry;
            }
        }
        
        // https://stackoverflow.com/questions/49939145/how-can-i-convert-a-usize-to-a-single-char
        let dirnumstr = char::from_digit(dirnum,10).unwrap();

        table[posy as usize][posx as usize] = dirnumstr;

        let mut prex = posx.clone(); let mut prey = posy.clone();
        let mut predir = match dir.clone() {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        };
        // table[(posy + changey) as usize][(posx + changex) as usize] = '@';
        // for row in &table {
        //     for c in row {
        //         print!("{c}");
        //     }
        //     println!("");
        // }
        
        // it is not 1918, number is too high
        // is not 1862, number is too high still
        // it is still lower than 1873
        // just so you know it is probably not 122
        // it is not 1677
        // it is 1721, but why
        
        let mut subtable = table.clone();
        
        let change2y = posy-changey; let change2x = posx+changex;
        if change2y < 0 || change2x < 0 || change2x as usize >= subtable[0].len() || change2y as usize >= subtable.len() {continue;}

        // subtable[change2y as usize][change2x as usize] = '@';
        // println!("---");
        // for row in &subtable {
        //     for c in row {
        //         print!("{c}");
        //     }
        //     println!("");
        // }
        if table[(posy-changey) as usize][(posx + changex) as usize] != '.' {continue;}
        subtable[(posy - changey) as usize][(posx + changex) as usize] = '#';

        let mut i = 0;
        loop {
            i+=1;
            if i > 100000 {n+=1;break;}
            let (changey,changex) = match predir.clone() {
                Direction::North => (1,0),
                Direction::East => (0,1),
                Direction::South => (-1,0),
                Direction::West => (0,-1)
            };
            let currx = prex + changex;
            let curry = prey - changey;

            if currx < 0 || curry < 0 {break;}
            let row = subtable.iter().nth(curry as usize);
            if row.is_none() {break;}
            let c = row.unwrap().iter().nth(currx as usize);
            if c.is_none() {break;}

            let c = c.unwrap();

            // println!("{c:?}");
            match c {
                '#' => {predir = match predir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North
                }; continue;},
                '.' => {
                    prex = currx; prey = curry;
                },
                _ => {
                    // let dirnum = predir as i32;
                    // https://stackoverflow.com/questions/43983414/how-to-convert-a-rust-char-to-an-integer-so-that-1-becomes-1
                    // let number: i32 = c.to_digit(10).unwrap() as i32;
                    // if number == dirnum {n+=1;break;}
                    // println!("{prex} {prey} {predir:?}");
                    // if currx == posx && curry == posy && predir == dir {n+=1;break;}
                    prex = currx; prey = curry;
                }
            }
        }
        
        println!("");
        // subtable[prey as usize][prex as usize] = 'X';
        for row in &subtable {
            for c in row {
                print!("{c}");
            }
            println!("");
        }
        println!("broken out of at {i}");
        // println!("{posx} {posy} {dir:?} {n}");
        if n == 9 {break;}
    }

    // println!("{table:?}");
    // for row in &table {
    //     for c in row {
    //         print!("{c}");
    //     }
    //     println!("");
    // }
    n
}
fn main() {
    // println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
