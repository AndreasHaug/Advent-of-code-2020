use std::fs;
use regex::Regex;

fn find(a: usize, seat: &str, start: usize, end: usize,
	lower: usize, upper: usize,
	 upper_char: char, lower_char: char) -> usize {

    match seat.chars().nth(a) {
	Some(c) => {
	    if c == lower_char {
		if a == end - 1 {
		    return lower;
		}
		return find(a+1, seat, start, end,
		      lower, (upper + lower) / 2,
		      upper_char, lower_char);
	    }
	    else if c == upper_char {
		if a == end - 1 {
		    return upper;
		}
		return find(a+1, seat, start, end,
		      (upper + lower) / 2 + 1, upper,
		      upper_char, lower_char);
	    }
	    else {
		println!("Error: invalid input");
		return 0;
	    }
	}
	None => {
	    return 0;
	}
    }
}

fn find_row(s: &str) -> usize {
    find(0, s, 0, 7, 0, 127, 'B', 'F')
}

fn find_column(s: &str) -> usize {
    find(7, s, 7, 10, 0, 7, 'R', 'L')
}

fn find_rowid(s: &str) -> usize {
    find_row(s) * 8 + find_column(s)
}

fn part1(inputstr: &str) -> usize {
    *(Regex::new("\n")
	.expect("Could not read regex")
	.split(inputstr)
	.map(|x| find_rowid(x))
	.collect::<Vec<usize>>()
	.iter()
	.max()
	.unwrap())
}

fn part2(inputstr: &str) -> usize {
    let mut l = Regex::new("\n")
      .expect("Could not read regex")
	.split(inputstr)
	.filter(|x| {
	    let r = find_row(x);
	    if r == 0 || r == 127 {
		return false;
	    }
	    true
	})
	.map(|x| find_rowid(x))
	.collect::<Vec<usize>>();

    l.sort();

    for a in 0..l.len()-2 {
	if l[a] != l.len() - 1 {
	    if l[a] + 1 != l[a + 1] {
		return l[a] + 1;
	    }
	}
    }
    0
}

fn main() {
    
    let inputfile = fs::read_to_string("input.txt")
	.expect("File could not be read");

println!("Part1: {}",
	     part1(&inputfile));

    println!("Part2: {}", part2(&inputfile));
}
