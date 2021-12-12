use std::{collections::HashMap, fs};

fn main() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("Could not read file");
    let contents: Vec<&str> = contents.lines().collect();

    let mut moves: Vec<(&str, &str)> = Vec::new();

    for _move in contents {
        let split: Vec<&str> = _move.split(' ').collect();
        moves.push((split[0], split[1]))
    }

    part_1(&moves);
    part_2(&moves);
}

fn part_1(moves: &Vec<(&str, &str)>) {
    let mut dive = HashMap::new();

    for m in moves {
        match m.0 {
            "forward" => {
                let count = dive.entry("forward").or_insert(0);
                *count += m.1.parse::<u32>().expect("Not a number");
            }
            "down" => {
                let count = dive.entry("depth").or_insert(0);
                *count += m.1.parse::<u32>().expect("Not a number")
            }
            "up" => {
                let count = dive.entry("depth").or_insert(0);
                *count -= m.1.parse::<u32>().expect("Not a number")
            }
            _ => panic!("Unknown direction: {}", m.0),
        }
    }

    let mut mult = 1;
    for value in dive.values() {
        mult *= value;
    }
    println!("{} ({})", mult, mult == 2120749);
}

fn part_2(moves: &Vec<(&str, &str)>) {
    let mut dive = HashMap::new();

    for m in moves {
        match m.0 {
            "forward" => {
                let forward_move = m.1.parse::<u64>().expect("Not a number");
                let count = dive.entry("forward").or_insert(0);
                *count += forward_move;

                let aim = *dive.entry("aim").or_insert(0);
                let count = dive.entry("depth").or_insert(0);
                *count += aim * forward_move;
            }
            "down" => {
                let count = dive.entry("aim").or_insert(0);
                *count += m.1.parse::<u64>().expect("Not a number")
            }
            "up" => {
                let count = dive.entry("aim").or_insert(0);
                *count -= m.1.parse::<u64>().expect("Not a number")
            }
            _ => panic!("Unknown direction: {}", m.0),
        }
    }

    let forward = dive.get("forward").ok_or("No forward").expect("No forward");
    let depth = dive.get("depth").ok_or("No forward").expect("No forward");
    println!("{} ({})", forward * depth, forward * depth == 2138382217);
}
