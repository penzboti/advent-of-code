use util::{read_file, split_at_newline};

#[derive(Debug, Clone)]
struct Line {
    number: u64,
    numbers: Vec<u32>
}

use convert_base::*;

fn part1() -> u64 {
    let mut n = 0;
    let file = split_at_newline(read_file("input.txt".to_string()));
    let list: Vec<Line> = file.iter().
        map(|x| x.split(": ").collect::<Vec<&str>>())
        .map(|x| Line {
            number: x[0].parse::<u64>().unwrap(),
            numbers: x[1].split(" ").collect::<Vec<&str>>().iter().map(|y| y.parse::<u32>().unwrap()).collect::<Vec<u32>>()
        })
    .collect();

    // so we have to systematically go trough every iteration with the operators between the numbers
    // we can do this, with a base 2 number
    // every digit in that number represents an operator.
    // 0 => addition
    // 1 => multiplication
    // 2 => for part 2; combination? ( 1&5 -> 15 )
    // there is a max, so we never loop forever, and the operations are clear

    for line in list {
        // https://docs.rs/convert-base/latest/convert_base/ - converting numbers between bases
        // https://stackoverflow.com/questions/52673580/how-can-a-range-be-used-in-a-vec - filling a vector with numbers
        let maxnumvec: Vec<u32> = Convert::new(2,10).convert::<u8,u32>(&(0..line.numbers.len()-1).map(|_| 1).collect::<Vec<u8>>());
        // https://stackoverflow.com/questions/51208703/how-to-raise-a-number-to-a-power - 10^i
        let maxnum: u32 = maxnumvec.iter().enumerate().map(|(i, x)| x*(10_u32.pow(i as u32))).sum();

        for j in 0..maxnum+1 {
            let mut base2 = Convert::new(10,2).convert::<u32,u8>(&vec![j]);
            // if the converted number is not long enough (with zeros before the number, like 000100)
            for _ in base2.len()..line.numbers.len()-1 {
                base2.push(0);
            }

            let mut number: u64 = line.numbers[0] as u64;

            for i in 1..line.numbers.len() {
                let currnum = line.numbers[i] as u64;
                let operator = base2[i-1];

                match operator {
                    0 => {number+=currnum},
                    1 => {number*=currnum},
                    _ => {}
                }
            }

            if number == line.number {n+=number;break;}
        }
    }
    n
}

fn part2() -> u64 {
    let mut n = 0;
    let file = split_at_newline(read_file("input.txt".to_string()));
    let list: Vec<Line> = file.iter().
        map(|x| x.split(": ").collect::<Vec<&str>>())
        .map(|x| Line {
            number: x[0].parse::<u64>().unwrap(),
            numbers: x[1].split(" ").collect::<Vec<&str>>().iter().map(|y| y.parse::<u32>().unwrap()).collect::<Vec<u32>>()
        })
    .collect();

    for line in list {
        // i just switched it to base three
        let maxnumvec: Vec<u32> = Convert::new(3,10).convert::<u8,u32>(&(0..line.numbers.len()-1).map(|_| 2).collect::<Vec<u8>>());
        let maxnum: u32 = maxnumvec.iter().enumerate().map(|(i, x)| x*(10_u32.pow(i as u32))).sum();

        for j in 0..maxnum+1 {
            let mut base3 = Convert::new(10,3).convert::<u32,u8>(&vec![j]);
            for _ in base3.len()..line.numbers.len()-1 {
                base3.push(0);
            }

            let mut number: u64 = line.numbers[0] as u64;

            for i in 1..line.numbers.len() {
                let currnum = line.numbers[i] as u64;
                let operator = base3[i-1];

                match operator {
                    0 => {number+=currnum},
                    1 => {number*=currnum},
                    // and just added this operator logic
                    // https://rustjobs.dev/blog/convert-int-to-string-in-rust/ - for the numbers length in combination
                    2 => {number = number* 10_u64.pow(currnum.to_string().len() as u32) + currnum}
                    _ => {}
                }
            }
            if number == line.number {n+=number;break;}
        }
    }
    n
}

fn main() {
    // the numbers are pretty big
    println!("Part 1: {}", part1());
    // part 2 is quite slow to run
    println!("Part 2: {}", part2());
}
