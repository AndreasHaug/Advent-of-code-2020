use std::fs;
use regex::Regex;

fn has_valid_fields(s: &str, r: &Regex, part2: bool) -> bool {
    
    let (mut ecl, mut pid, mut eyr, mut hcl, mut byr, mut iyr, mut hgt) =
	(false, false, false, false, false, false, false);
    
    for b in r.split(s) {

	let substrings = Regex::new(":")
	    .expect("Could not read regex")
	    .split(b)
	    .collect::<Vec<&str>>();
	
	if substrings[0] == "ecl" {
	    ecl = true;

	    if part2 {
		ecl = substrings[1] == "amb" ||
		    substrings[1] == "blu" || substrings[1] == "brn" ||
		    substrings[1] == "gry" || substrings[1] == "grn" ||
		    substrings[1] == "hzl" || substrings[1] == "oth";		
	    }
	}
	else if substrings[0] == ("pid") {
	    pid = true;

	    if part2 {
		if substrings[1].len() != 9 {
		    pid = false;
		    continue;
		}
		
		match substrings[1].parse::<i32>() {
		    Ok(_) => continue,
		    Err(_) => pid = false,
		}
	    }
	}	
	else if substrings[0] == "eyr" {
	    eyr = true;
	    
	    if part2 {
		match substrings[1].parse::<i32>() {
		    Ok(o) => eyr = o >= 2020 && o <= 2030,			
		    Err(_) => eyr = false,
		}
	    }
	}	
	else if substrings[0] == "hcl" {
	    hcl = true;

	    if part2 {		
		let mut chars = substrings[1].chars();
		if substrings[1].len() != 7 || chars.nth(0).unwrap() != '#' {
		    hcl = false;
		    continue;
		}
		for c in chars {
		    if !c.is_alphanumeric() {
			hcl = false;
			break;
		    }
		}		
	    }	    
	}	
	else if substrings[0] == "byr" {
	    byr = true;
	    if part2 {
		match substrings[1].parse::<i32>() {
		    Ok(n) => byr = n >= 1920 && n <= 2002,		
		    Err(_) => byr = false,
		}		
	    }
	}	
	else if substrings[0] == "iyr" {
	    iyr = true;

	    if part2 {		
		match substrings[1].parse::<i32>() {
		    Ok(n) => iyr = n >= 2010 && n <= 2020,
		    Err(_) => {
			iyr = false;
			continue;			
		    }
		}
	    }
	}	
	else if substrings[0] == "hgt" {
	    hgt = true;

	    if part2 {
		if substrings[1].ends_with("cm") && substrings[1].len() == 5 {
		    let sl = &String::from(substrings[1])[0..3];

		    match sl.parse::<i32>() {
			Ok(n) => hgt = n >= 150 && n <= 193,
			Err(_) => hgt = false,
		    }
		}		
		else if substrings[1].ends_with("in") && substrings[1].len() == 4 {
		    let sl = &String::from(substrings[1])[0..2];
		    match sl.parse::<i32>() {
			Ok(n) => hgt = n >= 59 && n <= 76,
			Err(_) => hgt = false,
		    }
		}
		else {
		    hgt = false;
		}		
	    }	    
	}
    }
    
    ecl && pid && eyr && hcl &&	byr && iyr && hgt
}

fn count(input: &str, part2: bool) -> usize {
    
    Regex::new("\n\n")
	.expect("Could not read regex1")
	.split(input)
	.collect::<Vec<&str>>()
	.iter()
	.filter(|x| has_valid_fields(x, &Regex::new("\n| ").expect("Could not read regex2"), part2))
	.count()
}

fn main() {

    let inputfile = fs::read_to_string("input.txt").expect("File could not be read");
    println!("Part 1: {}", count(&inputfile, false));
    println!("Part 2: {}", count(&inputfile, true));
}
