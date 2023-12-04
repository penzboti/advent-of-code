extern crate util;
use util::{read_file, split_at_newline};

fn part1() -> u32 {
    let file = split_at_newline(read_file("input.txt".to_string()));

    let mut sum = 0;
    for i in 0..file.len() {
        let chunks = file[i].split(": ").collect::<Vec<_>>();
        let thenums = chunks[1].split(" | ").collect::<Vec<_>>();

        // honestly, vector guide here:
        // https://mkaz.blog/working-with-rust/vectors/
        let mut winnums = thenums[0].split(" ").map(|x| x.parse::<u32>().unwrap_or(0)).collect::<Vec<u32>>();
        winnums.retain(|x| *x != 0);
        let mut cardnumns = thenums[1].split(" ").map(|x| x.parse::<u32>().unwrap_or(0)).collect::<Vec<u32>>();
        cardnumns.retain(|x| *x != 0);

        let mut points = 1;
        for n in 0..cardnumns.len() {
            for m in 0..winnums.len() {
                if cardnumns[n] == winnums[m] {
                    points *= 2;
                }
            }
        }

        if points != 1 {
            sum += points/2;
        }

    }
    sum
}

fn part2() -> usize {
    let file = split_at_newline(read_file("input.txt".to_string()));

    let mut cards: Vec<(usize, usize)> = vec![];
    for i in 0..file.len() {
        cards.push((i+1, 1));
    }

    let mut sum = 0;
    for i in 0..file.len() {
        let chunks = file[i].split(": ").collect::<Vec<_>>();
        let thenums = chunks[1].split(" | ").collect::<Vec<_>>();

        let mut winnums = thenums[0].split(" ").map(|x| x.parse::<u32>().unwrap_or(0)).collect::<Vec<u32>>();
        winnums.retain(|x| *x != 0);
        let mut cardnumns = thenums[1].split(" ").map(|x| x.parse::<u32>().unwrap_or(0)).collect::<Vec<u32>>();
        cardnumns.retain(|x| *x != 0);

        let mut points = 0;
        for n in 0..cardnumns.len() {
            for m in 0..winnums.len() {
                if cardnumns[n] == winnums[m] {
                    points += 1;
                }
            }
        }

        let mut card_multiplier = 1;
        for card in &cards {
            if card.0 == (i+1) {
                card_multiplier = card.1;
            }
        }
        sum += card_multiplier;
        if points >= 1 {
            for n in 0..points {
                let nextcard = i+n+2;
                cards[pos].1 += card_multiplier;
            }
        }

    }
    sum
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
