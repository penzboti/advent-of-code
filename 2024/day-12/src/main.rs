use util::{split_at_newline, read_file};

#[derive(Clone,Debug,PartialEq)]
struct Coord {
    x: usize,
    y: usize
}

#[derive(PartialEq, Clone,Debug)]
enum Direction {
    North,
    East,
    South,
    West,
    None
}

struct DirectionalCoord {
    x: usize,
    y: usize,
    dir: Direction
}

fn part1() -> u32 {
    let file = split_at_newline(read_file("demo1.txt".to_string()));
    let table = file.iter().map(|x| x.split("").collect::<Vec<&str>>().iter().filter(|&y| y!=&"").map(|y| y.chars().nth(0).unwrap()).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut outerqueue = vec![Coord{ x: 0, y: 0}];

    // then we make every bunch of crops different letters
    loop {
        if outerqueue.len() == 0 {break;}
        let start = outerqueue.remove(0);
        let c = table[start.y][start.x];

        let mut a = 0;
        // because perimeter starts with u, like when area starts with a, right?
        let mut u = 0;

        let mut innerqueue: Vec<DirectionalCoord> = vec![DirectionalCoord { x: start.x,y: start.y, dir: Direction::None}];
        loop {
            if innerqueue.len() == 0 {break;}
            let curr = innerqueue.remove(0);

            // reused this code from day 10
            let north = (-1i32,0i32,Direction::South);
            let east = (0i32,1i32,Direction::West);
            let south = (1i32,0i32,Direction::North);
            let west = (0i32,-1i32,Direction::East);
            let excluedir = match curr.dir {
                Direction::North => { Direction::South },
                Direction::East => {Direction::West},
                Direction::South => {Direction::North},
                Direction::West => {Direction::East}
                Direction::None => {Direction::None}
            };
            for (changey,changex,fromdir) in [north,east,south,west] {
                if fromdir == excluedir {continue;}

                let newy = curr.y.clone() as i32 +changey;
                let newx = curr.x.clone() as i32 + changex;
                if newy < 0 || newx < 0 {continue;}

                let row = table.iter().nth(newy as usize);
                if row.is_none() {continue;}
                let raw = row.unwrap().iter().nth(newx as usize);
                if raw.is_none() {continue;}

                let currc = table[newy as usize][newx as usize];
                if currc == c {innerqueue.push(DirectionalCoord { x: newx as usize, y: newy as usize, dir: fromdir });}
                else if outerqueue.contains(&Coord{x: newx as usize,y:newy as usize}){
                    outerqueue.iter().filter(|&e| e!=Coord{x:newx as usize,y:newy as usize});
                };
            }
        }
    }

    0
}

fn main() {
    println!("Part 1: {}", part1());
}
