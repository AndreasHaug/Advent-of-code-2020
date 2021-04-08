use std::fs;
use std::collections::HashMap;
use seven::*;

fn part1(bags: &HashMap<String, Bag>, bagname: &String) -> usize   {
    let mut count: usize = 0;

    for (k, v) in bags.iter() {
	if k != bagname && v.can_contain(bags, bagname) {	    
	    count +=1;	    
	}
    }
    count
}

fn part2(bags: &HashMap<String, Bag>, bagname: &String) -> usize {
    bags.get(bagname).unwrap().count_content(bags, 0)
}

fn main() {
    let input = fs::read_to_string("input.txt")
	.expect("Could not read input file");

    let bags = bagnames(&input);

    println!("Part 1: {}",
	     part1(&bags, &String::from("shiny gold")));
    
    println!("Part 2: {}",
	     part2(&bags, &String::from("shiny gold")));
}
