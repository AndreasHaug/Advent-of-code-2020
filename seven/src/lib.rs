use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Bag {

    pub rep: String,
    pub contains: Vec<String>,
    pub count: u32,
}

impl Bag {
    pub fn new(r: String, c: Vec<String>) -> Bag {
	Bag {rep: r, contains: c, count: 0}
    }

    pub fn can_contain(&self,
		   bags: &HashMap<String, Bag>,
		   bagname: &String) -> bool {
	
	for a in self.contains.iter() {
	    if &a[2..] == bagname ||
		match bags.get(&a[2..]) {
		    Some(t) => t,
		    None => panic!("Could not look up \"{}\"", &a[2..]),
		}.can_contain(bags, bagname) {
		return true;
	    }
	}
	false
    }

    pub fn count_content(&self, bags: &HashMap<String, Bag>, mut count: usize) -> usize {
	for a in self.contains.iter() {
	    let c = a.chars().nth(0).unwrap().to_digit(10).unwrap() as usize;
	    count += c;
	    for _ in 0..c {
		count = bags.get(&a[2..]).unwrap().count_content(bags, count);		
	    }
	}
	count
    }
}

fn get_line_bagname(line: &str) -> String {
    itertools::join(
	line.split(" ")
	    .enumerate()
	    .filter(|&(i, _)| i < 2)
	    .map(|(_, x)| x), " ")
}

fn get_line_content(line: &str) -> Vec<String> {
    if line.contains("no other bags") {
	Vec::new()
    }
    else {
	itertools::join(
	    line.split(" ")
		.enumerate()
		.filter(|&(i, _)| i >= 4)
		.map(|(_, x)| x), " ")
	    .split(", ")
	    .map(|x| {
		if x.ends_with(".") {
		    x.strip_suffix(".").unwrap()
		}
		else {x}
	    })
	    .map(|x| {
		if x.ends_with("s") {
		    x.strip_suffix("s").unwrap()
		}
		else {x}
	    })
	    .map(|x| {
		if x.ends_with("bag") {
		    x.strip_suffix(" bag").unwrap()
		}
		else {x}
	    })
	    .map(|x| x.to_string())
	    .collect::<Vec<String>>()
    }
}

pub fn bagnames(lines: &str) -> HashMap<String, Bag> {
    lines
	.lines()
	.map(|line| (get_line_bagname(line), get_line_content(line)))
	.map(|(x, content)| (x.clone(), Bag::new(x, content)))
	.collect::<HashMap<String, Bag>>()	
}
