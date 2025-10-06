fn split_at_newline(contents: String) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    // https://stackoverflow.com/questions/1761051/difference-between-n-and-r
    for line in contents.split("\n") {
        // idk if i should disable empty lines here
        if line != "" {
            lines.push(line.to_string());
        }
    }
    return lines;
}

const DEMO1: &str = "AAAA
BBCD
BBCC
EEEC";

#[derive(Clone,Debug,PartialEq,Copy)]
struct Coord {
    x: usize,
    y: usize
}

#[derive(PartialEq, Clone,Debug)]
struct Crop {
    area: u32,
    perimeter: u32
}

fn part1() -> u32 {
    let file = split_at_newline(DEMO1.to_string());
    let mut table = file.iter().map(|x| x.split("").collect::<Vec<&str>>().iter().filter(|&y| y!=&""&&y!=&"\n").map(|y| y.chars().nth(0).unwrap()).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut list: Vec<Crop> = vec![];
    let mut outerqueue = vec![Coord{ x: 0, y: 0}];
    
    //println!("{table:?}");

    loop {
        println!("outer: {outerqueue:?}");
        if outerqueue.len() == 0 {break;}
        let start = outerqueue.remove(0);
        let c = table[start.y][start.x];
        table[start.y][start.x] = ' ';

        let mut crop = Crop { area: 0, perimeter: 0 };

        let mut innerqueue: Vec<Coord> = vec![start];
        // the loop is still infinite
        loop {
            println!("inner: {innerqueue:?}");
            if innerqueue.len() == 0 {break;}
            let curr = innerqueue.remove(0);
            crop.area += 1;

            let north = (-1i32,0i32);
            let east = (0i32,1i32);
            let south = (1i32,0i32);
            let west = (0i32,-1i32);
            for (changey,changex) in [north,east,south,west] {
                //println!("a direction");
                let newy = curr.y.clone() as i32 + changey;
                let newx = curr.x.clone() as i32 + changex;
                if newy < 0 || newx < 0 {continue;}
                //println!("{newy} {newx}");

                let row = table.iter().nth(newy as usize);
                if row.is_none() {continue;}
                let raw = row.unwrap().iter().nth(newx as usize);
                if raw.is_none() {continue;}

                let currc = table[newy as usize][newx as usize];
                //println!("'{c}' '{currc}' {} {}",c==currc,currc==' ');
                if currc == ' ' {continue;}
                //println!("got trough");
                if currc == c {
                    //println!("chars match");
                    innerqueue.push(Coord { x: newx as usize, y: newy as usize });
                    if outerqueue.contains(&Coord{x: newx as usize,y:newy as usize}){
                        outerqueue = outerqueue.into_iter().filter(|&e| e!=Coord{x:newx as usize,y:newy as usize}).collect();
                    }
                }
                else if !outerqueue.contains(&Coord{x: newx as usize,y:newy as usize}){
                    //outerqueue = outerqueue.into_iter().filter(|&e| e!=Coord{x:newx as usize,y:newy as usize}).collect();
                    outerqueue.push(Coord{x: newx as usize,y:newy as usize});
                };
            }
        }
        list.push(crop);
    }

    list.iter().map(|x| x.area * x.perimeter).sum()
}

fn main() {
    println!("Part 1: {}", part1());
}
