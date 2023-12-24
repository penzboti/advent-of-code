extern crate util;
use util::{read_file, split_at_empty_line, split_at_newline};

#[derive(Debug, Eq, PartialEq)]
enum State {
    Rejected,
    Accepted,
    Empty,
}
#[derive(Debug)]
enum Mode {
    Less,
    Greater,
}
// can copy
// https://stackoverflow.com/questions/35458562/how-can-i-implement-rusts-copy-trait
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Part {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct Rule {
    part: Part,
    mode: Mode,
    value: u32,
    // gonna be a workflow name
    destination: String,
}
#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    // either a workflow (name) or a state (as string). will check for it code later =="A" && =="R"
    // technically, destination can also have state aswell
    else_rule: String,
}

#[derive(Debug, Copy, Clone)]
struct RatingPart {
    part: Part,
    value: u32,
}
#[derive(Debug)]
struct Rating {
    parts: Vec<RatingPart>,
    state: State,
}

fn part1() -> u32 {
    let file = split_at_empty_line(read_file("input.txt".to_string()));
    let startworkflows = split_at_newline(file[0].clone());
    let startratings = split_at_newline(file[1].clone());

    let raw_workflows = startworkflows.iter().map(|x| 
        x.split('{').collect::<Vec<&str>>()
    ).collect::<Vec<Vec<&str>>>();
    let workflows = raw_workflows.iter() .map(|x| {
        let name = x[0].to_string();
        let rawrules = x[1].split(',').collect::<Vec<&str>>();
        // found it on the docs
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take
        let rules = rawrules.iter().take(rawrules.len()-1)
            .map(|rule| {
                let partraw = rule.chars().nth(0).unwrap();
                let part = match partraw {
                    'x' => Part::X,
                    'm' => Part::M,
                    'a' => Part::A,
                    's' => Part::S,
                    _ => panic!("Unknown part: {}", partraw),
                };
                let moderaw = rule.chars().nth(1).unwrap();
                let mode = match moderaw {
                    '<' => Mode::Less,
                    '>' => Mode::Greater,
                    _ => panic!("Unknown mode: {}", moderaw),
                };
                // https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351
                let raw_value_and_destination = &rule[2..].split(':').collect::<Vec<&str>>();
                let value = raw_value_and_destination[0].parse::<u32>().unwrap();
                let destination = raw_value_and_destination[1].to_string();
                Rule { part, mode, value, destination }
            })
            .collect::<Vec<Rule>>();
        let else_rule = rawrules.last().unwrap().replace("}", "").to_string();
        Workflow { name, rules, else_rule } })
    .collect::<Vec<Workflow>>();

    let binding = startratings.iter().map(|x| {
        // WHY DOESN'T THIS WORK
        // x.as_str().replace(&['{', '}'][..], "").split(",").collect::<Vec<&str>>()
        // it shows error E0515.
        // its supposed to work, isn't it
        x.as_str().replace(&['{','}'][..], "")
    }).collect::<Vec<String>>();
    // and also why the binding
    // the compiler told me to do the binding, so that the value lives longer
    // the value doesn't need to live longer
    // why
    let raw_ratings = binding.iter().map(|x| {
        x.split(',').collect::<Vec<&str>>()
    }).collect::<Vec<Vec<&str>>>();

    let ratingparts = raw_ratings.iter().map(|x| {
        x.iter().map(|y| {
            let rawrating = y.split('=').collect::<Vec<&str>>();
            let part = match rawrating[0] {
                "x" => Part::X,
                "m" => Part::M,
                "a" => Part::A,
                "s" => Part::S,
                _ => panic!("Unknown part: {}", rawrating[0]),
            };
            let value = rawrating[1].parse::<u32>().unwrap();
            RatingPart { part, value }
        }).collect::<Vec<RatingPart>>()
    }).collect::<Vec<Vec<RatingPart>>>();
    let mut ratings = ratingparts.iter()
    .map(|x| {
        let mut parts = Vec::new();
        for e in x {
            parts.push(*e);
        }
        Rating { parts, state: State::Empty }
    }).collect::<Vec<Rating>>();

    // just getting to this point took like 1.5 hours

    for rating in &mut ratings {
        let mut current_workflow = workflows.iter().find(|x| x.name == "in").unwrap();
        let mut check_flow = "";
        loop {
            if check_flow == "A" {
                rating.state = State::Accepted;
                break;
            }
            else if check_flow == "R" {
                rating.state = State::Rejected;
                break;
            }
            else if check_flow != "" {
                current_workflow = workflows.iter().find(|x| x.name == check_flow).unwrap();
                check_flow = "";
            }
            
            let mut index = 0;
            loop {
                if index == current_workflow.rules.len() {
                    check_flow = &current_workflow.else_rule;
                    break;
                }
                let rule = &current_workflow.rules[index];
                let current_part = rating.parts.iter().find(|x| x.part == rule.part).unwrap();
                match rule.mode {
                    Mode::Less => {
                        if current_part.value < rule.value {
                            check_flow = &rule.destination;
                            break;
                        }
                    },
                    Mode::Greater => {
                        if current_part.value > rule.value {
                            check_flow = &rule.destination;
                            break;
                        }
                    },
                }
                index += 1;
            }
        }
    }

    ratings.iter().filter(|x| x.state == State::Accepted).map(|x| x.parts.iter().map(|&y| y.value).sum::<u32>()).sum::<u32>()
}

// part 2 requires min & max values
#[derive(Debug, Copy, Clone)]
struct RatingPart2 {
    part: Part,
    minvalue: u32,
    maxvalue: u32,
}
// since we only get accepted values, we don't have to deal with states, and can store ratingparts in vectors

fn part2() -> u64 {
    let file = split_at_empty_line(read_file("demo1.txt".to_string()));
    let startworkflows = split_at_newline(file[0].clone());
    let startratings = split_at_newline(file[1].clone());

    let raw_workflows = startworkflows.iter().map(|x| 
        x.split('{').collect::<Vec<&str>>()
    ).collect::<Vec<Vec<&str>>>();
    let workflows = raw_workflows.iter() .map(|x| {
        let name = x[0].to_string();
        let rawrules = x[1].split(',').collect::<Vec<&str>>();
        let rules = rawrules.iter().take(rawrules.len()-1)
            .map(|rule| {
                let partraw = rule.chars().nth(0).unwrap();
                let part = match partraw {
                    'x' => Part::X,
                    'm' => Part::M,
                    'a' => Part::A,
                    's' => Part::S,
                    _ => panic!("Unknown part: {}", partraw),
                };
                let moderaw = rule.chars().nth(1).unwrap();
                let mode = match moderaw {
                    '<' => Mode::Less,
                    '>' => Mode::Greater,
                    _ => panic!("Unknown mode: {}", moderaw),
                };
                let raw_value_and_destination = &rule[2..].split(':').collect::<Vec<&str>>();
                let value = raw_value_and_destination[0].parse::<u32>().unwrap();
                let destination = raw_value_and_destination[1].to_string();
                Rule { part, mode, value, destination }
            })
            .collect::<Vec<Rule>>();
        let else_rule = rawrules.last().unwrap().replace("}", "").to_string();
        Workflow { name, rules, else_rule } })
    .collect::<Vec<Workflow>>();

    let mut ratings = startratings.iter().map(|_| vec![
        RatingPart2 { part: Part::X, minvalue: 0, maxvalue: 4000 },
        RatingPart2 { part: Part::M, minvalue: 0, maxvalue: 4000 },
        RatingPart2 { part: Part::A, minvalue: 0, maxvalue: 4000 },
        RatingPart2 { part: Part::S, minvalue: 0, maxvalue: 4000 },
    ]).collect::<Vec<Vec<RatingPart2>>>();

    for mut rating in &mut ratings {
        let mut current_workflow = workflows.iter().find(|x| x.name == "in").unwrap();
        let mut check_flow = "";
        loop {
            if check_flow == "A" {
                break;
            }
            else if check_flow == "R" {
                break;
            }
            else if check_flow != "" {
                current_workflow = workflows.iter().find(|x| x.name == check_flow).unwrap();
                check_flow = "";
            }
            
            for index in 0..current_workflow.rules.len() {
                if index == current_workflow.rules.len() {
                    check_flow = &current_workflow.else_rule;
                }
                let rule = &current_workflow.rules[index];
                // iter_mut exists
                // https://stackoverflow.com/questions/48551026/mutable-reference-to-an-item-using-find-in-rust
                let current_part: &mut RatingPart2 = rating.iter_mut().find(|x| x.part == rule.part).unwrap();
                match rule.mode {
                    Mode::Less => {
                        if current_part.minvalue < rule.value {
                            // so uuuuh this is immutable for some reason
                            current_part.minvalue = rule.value;
                        }
                    },
                    Mode::Greater => {
                        if current_part.maxvalue > rule.value {
                            current_part.maxvalue = rule.value;
                        }
                    },
                }
                check_flow = &rule.destination;
            }
        }
    }

    // for rating in &ratings {
    //     println!("{:?}", rating);
    // }

    // aaand this also doesnt work
    // but at least multiplying in iters is a thing
    // https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.product
    // threw some errors, will use old methods for now
    // ratings.iter().map(|x| x.iter().map(|y| (y.maxvalue-y.minvalue) as u64).inspect(|&x| println!("a:{}", x)).product::<u64>() as u64).inspect(|&x| println!("b:{}", x)).product::<u64>()

    // can't really figure out what to exactly do here
    // my data gathering algorithm should work fine
    // im comparing numbers in python interpreter at this point
    // instructions unclear
    // will try to do it tomorrow after looking at someone else's solution
    let mut sum: u64 = 1;
    for rating in &ratings {
        println!("{:?}", rating);
        // let mut product: u64 = 1;
        let mut minval = 0;
        let mut maxval = 4000;
        for part in rating {
            // println!("{:?}", part);
            // product *= (part.maxvalue-part.minvalue) as u64;
            if part.minvalue > minval {
                minval = part.minvalue;
            }
            if part.maxvalue < maxval {
                maxval = part.maxvalue;
            }
        }
        // println!("{}", product);
        sum *= (maxval-minval) as u64*4;
    }
    sum
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}