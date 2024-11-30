// using the help of this guide
// https://www.tutorialspoint.com/rust/rust_modules.htm
// and some good ideas on
// https://github.com/andrew8088/advent-of-code/tree/main/2022/util

// also help from
// https://doc.rust-lang.org/book/ch12-02-reading-a-file.html

// https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust/47805781
#[cfg(target_os = "linux")]
static ENDLINE: &str = "\n";
#[cfg(target_os = "windows")]
static ENDLINE: &str = "\r\n";
#[cfg(target_os = "macos")]
static ENDLINE: &str = "\r";

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
    for line in contents.split(ENDLINE) {
        // idk if i should disable empty lines here
        if line != "" {
            lines.push(line.to_string());
        }
    }
    return lines;
}

pub fn split_at_empty_line(contents: String) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let double_split: String = format!("{}{}",ENDLINE,ENDLINE);
    for line in contents.split(&double_split) {
        lines.push(line.to_string());
    }
    return lines;
}

// some future update posibilites:
// https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html
