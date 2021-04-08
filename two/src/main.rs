use std::str::FromStr;
use std::fs;

fn valid_part1(line: String) -> bool {
    let split: Vec<&str> = line.split(|ch: char| ch == ' ' || ch == '-' || ch == ':').collect();
    let f: usize = FromStr::from_str(split[0]).unwrap();
    let t: usize = FromStr::from_str(split[1]).unwrap();
    let search_str = split[4];
    let count = search_str.chars().filter(|x| x.to_string() == split[2]).count();
    count >= f && count <= t
}

fn valid_part2(line: String) -> bool {
    let split: Vec<&str> = line.split(|ch: char| ch == ' ' || ch == '-' || ch == ':').collect();
    let f: usize = FromStr::from_str(split[0]).unwrap();
    let t: usize = FromStr::from_str(split[1]).unwrap();
    let search_str = split[4];

    search_str.chars().nth(f - 1).unwrap().to_string() == split[2] &&
	search_str.chars().nth(t - 1).unwrap().to_string() != split[2] ||
	search_str.chars().nth(f - 1).unwrap().to_string() != split[2] &&
	search_str.chars().nth(t - 1).unwrap().to_string() == split[2]
}

fn count(inputstr: &String, fun: &dyn Fn(String) -> bool) -> usize {
    inputstr.lines().filter(|x| {fun(String::from(*x))}).count()
}

fn main() {
    let inputfile = fs::read_to_string("input.txt").expect("File could not be read.");
    println!("Part 1: {}", count(&inputfile, &valid_part1));
    println!("Part 2: {}", count(&inputfile, &valid_part2));
}
