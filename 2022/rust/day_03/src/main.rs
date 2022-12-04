use std::{collections::HashSet, fs};

fn main() {
    let filename = "input_01.txt";
    let lines = fs::read_to_string(filename).expect("Cannot read file");
    let lines: Vec<&str> = lines.lines().collect();

    let res_part_1 = part_1(lines);
    let is_ok = if res_part_1 == 7826 { "ok" } else { "wrong" };
    println!("{} ({})", res_part_1, is_ok);
}

fn part_1(lines: Vec<&str>) -> u32 {
    let mut total = 0;
    for line in lines {
        match find_repeated_char(line) {
            Ok(c) => total += char_to_priority(c),
            Err(msg) => println!("{}", msg),
        }
    }

    total
}

fn find_repeated_char(line: &str) -> Result<char, &str> {
    if line.len() % 2 != 0 {
        return Err("line cannot be divided in 2");
    }

    let upper: HashSet<char> = line[0..line.len() / 2].chars().collect();
    let lower: HashSet<char> = line[line.len() / 2..line.len()].chars().collect();

    let mut intersection = upper.intersection(&lower);
    match intersection.next() {
        Some(c) => Ok(*c),
        None => Err("no interesection"),
    }
}

fn char_to_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 1 + 26
    }
}
