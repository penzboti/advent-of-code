use util::read_file;

fn part1() -> usize {
    let mut file = read_file("input.txt".to_string()).split(" ").collect::<Vec<&str>>().iter().filter(|&x| x != &"").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    const TIMES: usize = 25;
    // println!("{:?}", file);
    for i in 0..TIMES {
        let mut index = 0;
        loop {
            if index >= file.len() {break;}
            let curr = file.remove(index);
            let currstr = curr.to_string();
            if curr == 0 {
                file.insert(index, 1);
            } else if currstr.len() %2==0 {
                let halfpoint = currstr.len()/2;
                // https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351
                // substring is cool in rust (like in python)
                file.insert(index, currstr[..halfpoint].parse::<u64>().unwrap());
                file.insert(index+1, currstr[halfpoint..currstr.len()].parse::<u64>().unwrap());
                index += 1;
            } else {
                file.insert(index, curr*2024);
            }
            index += 1;
        }
        // println!("{:?}", file);
        println!("done with {i}")
    }
    file.len()
}

// part 2 is not a brute-force job.
// even part 1 is slow at the end
// some people said use "memoization" on reddit.

fn main() {
    println!("Part 1: {}", part1());
}
