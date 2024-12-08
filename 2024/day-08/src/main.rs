use util::{read_file,split_at_newline};

#[derive(Debug, Clone)]
struct Node {
    tipus: char,
    locations: Vec<(u32,u32)>
}

fn part1() -> u32 {
    let mut n = 0;
    let file = split_at_newline(read_file("input.txt".to_string()));
    let table: Vec<Vec<char>> = file.iter().map(|x| x.split("").collect::<Vec<&str>>()
        .iter().filter(|&y| y!=&"")
        .map(|y| y.chars().nth(0).unwrap()).collect::<Vec<char>>()
    ).collect();

    let mut list: Vec<Node> = vec![];

    for i in 0..table.len() {
        for j in 0..table[0].len() {
            let c = table[i][j];

            if c == '.' {continue;}

            if list.iter().map(|x| x.tipus).collect::<Vec<char>>().contains(&c) {
                // thank god this works it makes this type of shit so easy
                // https://users.rust-lang.org/t/filtering-and-editing-a-vector-full-of-custom-structs-based-on-a-struct-variable/58515
                list.iter_mut().find(|x| x.tipus == c).unwrap().locations.push((i as u32,j as u32));
            }
            else {
                list.push(Node {tipus: c, locations: vec![(i as u32,j as u32)]});
            }
        }
    }

    let mut antinodes: Vec<(i32,i32)> = vec![];
    for node in &list {
        for l1 in node.locations.clone() {
            for l2 in node.locations.clone() {

                let (n1y,n1x) = (l1.0 as i32,l1.1 as i32);
                let (n2y,n2x) = (l2.0 as i32,l2.1 as i32);
                if n1y == n2y && n1x == n2x {continue;}

                let dx:i32 = n2x-n1x;
                let dy:i32 = n2y-n1y;

                let (a1y,a1x) = (n1y-dy,n1x-dx);
                let (a2y,a2x) = (n2y+dy,n2x+dx);

                if a1y < table.len() as i32 && a1x < table[0].len() as i32 && a1x >= 0 && a1y >= 0 && !antinodes.contains(&(a1y,a1x)) {n+=1; antinodes.push((a1y,a1x));}
                if a2y < table.len() as i32 && a2x < table[0].len() as i32 && a2x >= 0 && a2y >= 0 && !antinodes.contains(&(a2y,a2x)) {n+=1; antinodes.push((a2y,a2x));}
            }
        }
    }

    // this requires table to be mutable
    // for (y,x) in &antinodes {
    //     table[*y as usize][*x as usize] = 'X';
    // }

    // for row in table {
    //     for c in row {
    //         print!("{c}");
    //     }
    //     println!("");
    // }

    n
}

fn part2() -> u32 {
    let mut n = 0;
    let file = split_at_newline(read_file("input.txt".to_string()));
    let table: Vec<Vec<char>> = file.iter().map(|x| x.split("").collect::<Vec<&str>>()
        .iter().filter(|&y| y!=&"")
        .map(|y| y.chars().nth(0).unwrap()).collect::<Vec<char>>()
    ).collect();

    let mut list: Vec<Node> = vec![];

    for i in 0..table.len() {
        for j in 0..table[0].len() {
            let c = table[i][j];

            if c == '.' {continue;}

            if list.iter().map(|x| x.tipus).collect::<Vec<char>>().contains(&c) {
                list.iter_mut().find(|x| x.tipus == c).unwrap().locations.push((i as u32,j as u32));
            }
            else {
                list.push(Node {tipus: c, locations: vec![(i as u32,j as u32)]});
            }
        }
    }

    // https://www.reddit.com/r/learnrust/comments/wno37e/ternary_operator_or_equivalent/
    let maxiter = if table.len() > table[0].len() {table.len()} else {table[0].len()};
    let mut antinodes: Vec<(i32,i32)> = vec![];

    for node in &list {
        for l1 in node.locations.clone() {
            for l2 in node.locations.clone() {
                let (n1y,n1x) = (l1.0 as i32,l1.1 as i32);
                let (n2y,n2x) = (l2.0 as i32,l2.1 as i32);

                if n1y == n2y && n1x == n2x {continue;}

                let dx:i32 = n2x-n1x;
                let dy:i32 = n2y-n1y;

                for i in 0..maxiter {
                    let (a1y,a1x) = (n1y-dy*i as i32,n1x-dx*i as i32);
                    let (a2y,a2x) = (n2y+dy*i as i32,n2x+dx*i as i32);

                    if a1y < table.len() as i32 && a1x < table[0].len() as i32 && a1x >= 0 && a1y >= 0 && !antinodes.contains(&(a1y,a1x)) {n+=1; antinodes.push((a1y,a1x));}
                    if a2y < table.len() as i32 && a2x < table[0].len() as i32 && a2x >= 0 && a2y >= 0 && !antinodes.contains(&(a2y,a2x)) {n+=1; antinodes.push((a2y,a2x));}
                }
            }
        }
    }

    n
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
