use util::read_file;

fn part1() -> u32 {
    let file = read_file("input.txt".to_owned()).to_string();
    let mut n = 0;

    // https://users.rust-lang.org/t/how-to-find-a-substring-in-a-string/88813
    for find in file.match_indices("mul(") {
        // println!("{:?}", find);
        let mut i: usize = find.0 + 4;
        let mut first_number: Vec<char> = vec![];
        let mut second_number: Vec<char> = vec![];
        let mut over_comma = false;
        let mut failed = false;

        loop {
            // forgot this from last year
            // https://stackoverflow.com/questions/24542115/how-to-index-a-string-in-rust
            let c = file.chars().nth(i).unwrap();
            // println!("{i} {c}");
            if c == ')' {
                break;
            } else if c == ',' {
                over_comma = true;
            } else if "0123456789".contains(c) {
                if over_comma {
                    second_number.push(c);
                } else {
                    first_number.push(c);
                }
            } else {
                failed = true;
                break;
            }
            i += 1;
        }
        if failed {
            continue;
        }

        // https://stackoverflow.com/questions/23430735/how-to-convert-vecchar-to-a-string
        let n1s: String = first_number.iter().collect();
        let n2s: String = second_number.iter().collect();
        let n1: u32 = n1s.parse().unwrap();
        let n2: u32 = n2s.parse().unwrap();

        n += n1 * n2;
    }
    n
}

fn part2() -> u32 {
    let file = read_file("input.txt".to_owned()).to_string();
    let mut n = 0;
    let mut enabled = true;

    // the link on the first part has a link here
    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f609105532d44039744a3c69f0099bb2
    let keys = ["mul(", "do()", "don't()"];
    let mut finds: Vec<(usize, &str)> = keys.iter().flat_map(|key| file.match_indices(key) ).collect();
    // sort trough instructions by index otherwise the matched string will be in order
    // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html
    finds.sort_by(|a, b| a.0.cmp(&b.0));

    for find in finds {
        // println!("{find:?}");

        // disable and enable instructions
        if find.1 == "do()" {enabled = true; continue;}
        if find.1 == "don't()" {enabled = false; continue;}

        // mul instruction
        if !enabled {continue;}
        let mut i: usize = find.0 + 4;
        let mut first_number: Vec<char> = vec![];
        let mut second_number: Vec<char> = vec![];
        let mut over_comma = false;
        let mut failed = false;

        loop {
            let c = file.chars().nth(i).unwrap();
            // println!("{i} {c}");
            if c == ')' {
                break;
            } else if c == ',' {
                over_comma = true;
            } else if "0123456789".contains(c) {
                if over_comma {
                    second_number.push(c);
                } else {
                    first_number.push(c);
                }
            } else {
                failed = true;
                break;
            }
            i += 1;
        }
        if failed {
            continue;
        }

        let n1s: String = first_number.iter().collect();
        let n2s: String = second_number.iter().collect();
        let n1: u32 = n1s.parse().unwrap();
        let n2: u32 = n2s.parse().unwrap();

        n += n1 * n2;
    }
    n
}

fn main() {
    // println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
