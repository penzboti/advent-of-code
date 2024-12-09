use util::read_file;

fn part1() -> usize {
    let file: Vec<u32> = read_file("input.txt".to_string()).split("").collect::<Vec<&str>>().iter().map(|x| x.parse::<u32>().unwrap_or(0)).collect();
    let mut list: Vec<Option<u32>> = vec![];

    let mut is_file = false;
    let mut index = 0;

    for n in file {
        for _ in 0..n {
            list.push(match is_file {
                true => {Some(index)},
                false => {None}
            });
        }
        if is_file {index+=1;}
        is_file = !is_file;
    }

    index = 0;
    for _ in 0..list.len() {
        let craw = list.iter().nth(index as usize);
        if craw.is_none() {break;}
        let c = craw.unwrap();

        if c.is_some() {index+=1;continue;}
        list.remove(index as usize);

        loop {
            let endindex = list.len() as usize-1;
            let c = list[endindex];

            if c.is_none() {list.pop(); continue;}

            list.insert(index as usize,c);
            list.pop();
            break;
        }
        index+=1;
    }

    list.iter().enumerate().map(|(i,x)| i*x.unwrap_or(0) as usize).sum::<usize>()
}

#[derive(Debug, Clone, Copy)]
struct File {
    id: Option<u32>,
    width: u32
}

fn part2() -> usize {
    let file: Vec<u32> = read_file("input.txt".to_string()).split("").collect::<Vec<&str>>().iter().filter(|&x| x!=&""&&x!=&"\n").map(|x| x.parse::<u32>().unwrap_or(0)).collect();
    let mut list: Vec<File> = vec![];

    let mut is_file = true;
    let mut index = 0;

    for n in file {
        if n == 0 {is_file = !is_file; continue;}
        list.push(File {
            id: match is_file {
                true => {Some(index)},
                false => {None}},
            width:n}
        );
        if is_file {index+=1;}
        is_file = !is_file;
    }

    index = 0;
    for _ in 0..list.len() {
        let raw = list.iter().nth(index as usize);
        if raw.is_none() {break;}
        let file = raw.unwrap();

        let id = file.id;
        if id.is_some() {index+=1;continue;}

        let len = file.width;

        let mut endindex = list.len() as usize;
        loop {
            endindex -= 1;
            if endindex < index as usize {break;}

            let endfile = list[endindex];
            if endfile.id.is_none() {continue;}

            if len < endfile.width {continue;}

            list.remove(index as usize);
            list.insert(index as usize, endfile);
            list.remove(endindex);
            list.insert(endindex, File {id:None,width:endfile.width});

            if len > endfile.width {index+=1;list.insert(index as usize,File {id:None,width:len-endfile.width});index-=1;}
            break;
        }
        index+=1;
    }

    let mut sumlist: Vec<Option<u32>> = vec![];

    for elem in list {
        for _ in 0..elem.width {
            sumlist.push(elem.id);
        }
    }

    sumlist.iter().enumerate().map(|(i,x)| i*x.unwrap_or(0) as usize).sum::<usize>()
}

fn main() {
    let p1 = part1();
    println!("Part 1: {p1}");

    let p2 = part2();
    println!("Part 2: {p2}");
}
