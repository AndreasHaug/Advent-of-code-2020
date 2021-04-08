use std::fs;
use std::collections::HashSet;
use regex::Regex;

fn part1(input: &str) -> usize {
    Regex::new("\n\n")
	.expect("Could not read regex")
	.split(input)
	.map(|x|
	     x.chars()
	     .filter(|x| x.is_alphabetic())	
	     .collect::<HashSet<char>>()
	     .len())
	.sum::<usize>()
}

fn part2(input: &str) ->usize {
    Regex::new("\n\n")
	.expect("Could not read regex")
	.split(input)
	.map(|x| count_findall(x))	
	.sum::<usize>()
}

fn count_findall(group: &str) -> usize {

    let mut count: usize = 0;
    let l = Regex::new("\n")
	.expect("Could not read regex")
	.split(group)
	.filter(|x| x.len() > 0)
	.collect::<Vec<&str>>();

    if l.len() == 1 {
	count = l[0].len();
	return count;
    }
    
    for a in l[0].chars() {
	let mut is_in = false;		
	for b in 1..l.len() {
	    if l[b].contains(a) {
		is_in = true;
	    }
	    else {
		is_in = false;
		break;
	    }
	}
	if is_in {
	    count += 1;	
	}
    }
    count	
}

fn main() {

    let inputfile = fs::read_to_string("input.txt")
	.expect("File could not be read");

    println!("Part 1: {}", part1(&inputfile));
    println!("Part 2: {}", part2(&inputfile));
}
