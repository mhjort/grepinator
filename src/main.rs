use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    let contents = fs::read_to_string(filename).expect("Failed to read file");
    let matches = search(query, &contents);
    for matched_line in matches {
        println!("{}", matched_line);
    }
}

fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    return results;
}
