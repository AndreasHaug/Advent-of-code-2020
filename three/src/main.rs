use std::fs;

fn solve(input: &Vec<&str>, right: usize, down: usize) -> usize {

    let mut count: usize = 0;
    let mut x: usize = 0;

    for a in (0..input.len()).step_by(down) {
	if input[a].chars().nth(x).unwrap() == '#' {
	    count += 1;
	}

	for _ in 0..right {
	    if x == input[a].len()-1 {x = 0;}
	    else {x += 1;}
	}
    }    
    count
}

fn main() {
    let inputfile = fs::read_to_string("input.txt").
	expect("File could not be read.");
    let input: Vec<&str> = inputfile.lines().collect();

    let solution1 = solve(&input, 3, 1);
    println!("Part 1: {}", solution1);

    let solution2 =
	solve(&input, 1, 1) *
	solve(&input, 3, 1) *
	solve(&input, 5, 1) *
	solve(&input, 7, 1) *
	solve(&input, 1, 2);
    
    println!("Part 2: {}", solution2);
}
