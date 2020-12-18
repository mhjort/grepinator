use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);

    let contents = fs::read_to_string(filename).expect(&format!("Failed to read file {}", filename));
    let matches = search(query, &contents);
    for matched_line in matches {
        println!("{}", matched_line);
    }
}

fn parse_config<'a>(args: &'a Vec<String>) -> (&'a str, &'a str) {
    return (&args[1], &args[2]);
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
