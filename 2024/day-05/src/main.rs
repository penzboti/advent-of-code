use util::{read_file, split_at_newline, split_at_empty_line};

fn part1() -> u32 {
    let mut n = 0;
    let file = split_at_empty_line(read_file("input.txt".to_string()));
    let rules: Vec<(u32,u32)> = split_at_newline(file[0].clone()).iter().map(|x| x.split('|').collect::<Vec<&str>>()).map(|x| (x[0].parse().unwrap(), x[1].parse().unwrap())).collect();
    let all_pages: Vec<Vec<u32>> = split_at_newline(file[1].clone()).iter().map(|x| x.split(',').map(|s| s.parse::<u32>().unwrap()).collect()).collect();

    for pages in all_pages {
        let mut correct = true;

        for i in 0..pages.len() {
            let page = pages[i];

            for rule in &rules {
                if !pages.contains(&rule.0) || !pages.contains(&rule.1) {continue}

                // the first number is later than the second
                if page == rule.0 {
                    // probably forgot this from last year
                    // https://users.rust-lang.org/t/get-index-of-element-in-vec/73563
                    let second_index = pages.iter().position(|&x| x == rule.1).unwrap();
                    if i > second_index {
                        correct = false;
                        // println!("1 {:?}, {i}, {second_index}", rule);
                    }
                }

                // the second number is earlier than the first
                if page == rule.1 {
                    let first_index = pages.iter().position(|&x| x == rule.0).unwrap();
                    if i < first_index {
                        correct = false;
                        // println!("2 {:?}, {first_index}, {i}", rule);
                    }
                }
            }
        }

        // println!("{:?} -> {}", pages, correct);
        let middleindex = pages.len()/2;
        if correct {
            n+=pages[middleindex];
        }
    }
    n
}

fn part2() -> u32 {
    let mut n = 0;
    let file = split_at_empty_line(read_file("input.txt".to_string()));
    let rules: Vec<(u32,u32)> = split_at_newline(file[0].clone()).iter().map(|x| x.split('|').collect::<Vec<&str>>()).map(|x| (x[0].parse().unwrap(), x[1].parse().unwrap())).collect();
    let all_pages: Vec<Vec<u32>> = split_at_newline(file[1].clone()).iter().map(|x| x.split(',').map(|s| s.parse::<u32>().unwrap()).collect()).collect();

    for mut pages in all_pages {
        let mut correct = true;

        // the loop is needed, but like for only one case
        loop {
            let mut corrected = false;

            for i in 0..pages.len() {
                let mut page = pages[i];

                for rule in &rules {
                    if !pages.contains(&rule.0) || !pages.contains(&rule.1) {continue}

                    // the first number is later than the second
                    if page == rule.0 {
                        let second_index = pages.iter().position(|&x| x == rule.1).unwrap();
                        if i > second_index {
                            correct = false;
                            corrected = true;
                            // println!("1 {:?}", rule);
                            // println!("{:?}", pages);
                            pages[i] = rule.1;
                            page = rule.1;
                            pages[second_index] = rule.0;
                            // println!("{:?}", pages);
                        }
                    }

                    // the second number is earlier than the first
                    else if page == rule.1 {
                        let first_index = pages.iter().position(|&x| x == rule.0).unwrap();
                        if i < first_index {
                            correct = false;
                            corrected = true;
                            // println!("2 {:?}", rule);
                            // println!("{:?}", pages);
                            pages[first_index] = rule.1;
                            page = rule.0;
                            pages[i] = rule.0;
                            // println!("{:?}", pages);
                        }
                    }
                }
            }
            if corrected == false {break}
        }

        // println!("{:?} -> {}", pages, correct);
        let middleindex = pages.len()/2;
        if !correct {
            n+=pages[middleindex];
        }
    }
    n
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
