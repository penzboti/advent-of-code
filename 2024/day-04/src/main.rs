use util::{split_at_newline, read_file};

fn get_text(table: Vec<Vec<char>>, x: usize, y: usize, mult_x: i32, mult_y: i32) -> String {
    (0..4).map(|n| table[(y as i32+ n as i32*mult_y) as usize][(x as i32+ n as i32*mult_x) as usize]).collect::<Vec<char>>().iter().collect::<String>()
}

fn part1() -> u32 {
    let mut n = 0;
    let file: Vec<Vec<char>> = split_at_newline(read_file("input.txt".to_string())).iter().map(|x| x.chars().collect()).collect();
    let prewidth = file.len(); let preheight = file[0].len();

    // last year i used 2darray but this good aswell it just looks bad when creating trust me bro
    // https://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
    let mut table = vec![vec![' '; preheight+6]; prewidth+6];

    // i added 3 to every side of the file-table. so i dont have to check if i will run out
    // easier to just create a new table with the new dimensions, and then fill it with the data, indented

    let mut xlocations = vec![];

    // fill the table with the correct data from the file
    for i in 0..file.len() {
        for j in 0..file[0].len() {
            let c = file[i][j];
            let y = i+3; let x = j+3;
            if c == 'X' { xlocations.push((x,y)); }
            table[y][x] = c;
        }
    }

    for (x, y) in xlocations {
        // https://stackoverflow.com/questions/74998452/how-do-i-map-a-range
        // https://stackoverflow.com/questions/70050432/how-do-i-join-a-char-vector-in-rust
        let horizontal = get_text(table.clone(), x, y, 0, 1);
        let vertical = get_text(table.clone(), x, y, 1, 0);
        let backwards_horizontal = get_text(table.clone(), x, y, 0, -1);
        let backwards_vertical = get_text(table.clone(), x, y, -1, 0);
        // --
        let topright = get_text(table.clone(), x, y, 1, 1);
        let bottomright = get_text(table.clone(), x, y, 1, -1);
        let bottomleft = get_text(table.clone(), x, y, -1, -1);
        let topleft = get_text(table.clone(), x, y, -1, 1);

        let options = vec![horizontal, vertical, backwards_vertical, backwards_horizontal, topleft, topright, bottomleft, bottomright];
        n += options.iter().map(|x| x=="XMAS").map(|x| if x {1} else {0}).sum::<u32>();
    }

    // print the table
    // for i in table {
    //     for j in i {
    //         print!("{}", j);
    //     }
    //     println!();
    // }
    // println!("{} {}", width, height);

    n
}

fn part2() -> u32 {
    let mut n = 0;
    let file: Vec<Vec<char>> = split_at_newline(read_file("input.txt".to_string())).iter().map(|x| x.chars().collect()).collect();
    let prewidth = file.len(); let preheight = file[0].len();

    let mut table = vec![vec![' '; preheight+6]; prewidth+6];

    // search for a-s here
    let mut alocations = vec![];

    for i in 0..file.len() {
        for j in 0..file[0].len() {
            let c = file[i][j];
            let y = i+3; let x = j+3;
            if c == 'A' { alocations.push((x,y)); }
            table[y][x] = c;
        }
    }

    for (x, y) in alocations {
        let mut areastring = (0..9).map(|n| (n/3, n%3)).map(|(j, i)| table[y+j-1][x+i-1]).map(|x| if x == '\n' {' '} else {x}).collect::<Vec<char>>().iter().collect::<String>();

        // they both gave me the replace_range fn
        // https://stackoverflow.com/questions/66661118/how-do-i-change-characters-at-a-specific-index-within-a-string-in-rust
        // https://stackoverflow.com/questions/62497774/how-do-you-remove-a-sub-string-according-to-index-from-a-rust-string-or-str-typ
        areastring.replace_range(1..2, "");
        areastring.replace_range(3-1..4-1, "");
        areastring.replace_range(5-2..6-2, "");
        areastring.replace_range(7-3..8-3, "");
        // println!("{}", areastring);

        let options = ["MMASS", "SMASM", "MSAMS", "SSAMM"];
        if options.contains(&areastring.as_str()) {n+=1}
    }
    n
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
