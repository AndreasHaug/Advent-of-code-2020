use std::collections::HashMap;
use std::fs;

fn part1(instructions: &Vec<(String, i32)>) -> i32 {
    let mut m: HashMap<i32, ()> = HashMap::new();
    let mut instr: i32 = 0;
    let mut acc: i32 = 0;

    while !m.contains_key(&instr) {
        m.insert(instr, ());

        match instructions.get(instr as usize).unwrap().0.as_str() {
            "nop" => instr += 1,
            "acc" => {
                acc += instructions.get(instr as usize).unwrap().1;
                instr += 1;
            }
            "jmp" => instr += instructions.get(instr as usize).unwrap().1,
            _ => panic!("Got unexpected instruction"),
        }
    }
    acc
}

fn main() {
    let lines = fs::read_to_string("input.txt")
        .expect("Could not read input file")
        .lines()
        .map(|x| {
            (
                x[..3].to_string(),
                x[4..].to_string().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(String, i32)>>();

    println!("Part1: {}", part1(&lines));
}
