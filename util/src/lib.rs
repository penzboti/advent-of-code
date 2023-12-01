// using the help of this guide
// https://www.tutorialspoint.com/rust/rust_modules.htm
// and some good ideas on
// https://github.com/andrew8088/advent-of-code/tree/main/2022/util

// also help from
// https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
pub fn read_file(path: String) -> String {
    println!("Reading file: {}", path);
    let realpath = "input/".to_string() + &path;
    let contents = std::fs::read_to_string(realpath)
        .expect("Something went wrong reading the file");
    return contents;
}

pub fn split_at_newline(contents: String) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    // https://stackoverflow.com/questions/1761051/difference-between-n-and-r
    for line in contents.split("\r\n") {
        // idk if i should disable empty lines here
        if line != "" {
            lines.push(line.to_string());
        }
    }
    return lines;
}

pub fn split_at_empty_line(contents: String) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for line in contents.split("\r\n\r\n") {
        lines.push(line.to_string());
    }
    return lines;
}

// some future update posibilites:
// https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html