use std::{collections::HashSet, fs};

fn main() {
    let filename = "input_01.txt";
    let lines = fs::read_to_string(filename).expect("Cannot read file");
    let lines: Vec<&str> = lines.lines().collect();

    let res_part_1 = part_1(&lines);
    let is_part_1_ok = if res_part_1 == 7826 { "ok" } else { "wrong" };
    println!("{} ({})", res_part_1, is_part_1_ok);

    let res_part_2 = part_2(&lines);
    let is_part_2_ok = if res_part_2 == 2577 { "ok" } else { "wrong" };
    println!("{} ({})", res_part_2, is_part_2_ok);
}

fn part_1(lines: &Vec<&str>) -> u32 {
    let mut total = 0;
    for line in lines {
        match find_repeated_char(line) {
            Ok(c) => total += char_to_priority(c),
            Err(_) => println!("No repeated char: {}", line),
        }
    }

    total
}

fn find_repeated_char(line: &str) -> Result<char, ()> {
    if line.len() % 2 != 0 {
        return Err(());
    }

    let upper: HashSet<char> = line[0..line.len() / 2].chars().collect();
    let lower: HashSet<char> = line[line.len() / 2..line.len()].chars().collect();

    match upper.intersection(&lower).next() {
        Some(c) => Ok(*c),
        None => Err(()),
    }
}

fn char_to_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 1 + 26
    }
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let mut total = 0;
    for i in (0..lines.len()).step_by(3) {
        let group = (lines[i], lines[i + 1], lines[i + 2]);
        let priority = char_to_priority(find_group_badge(group).expect("No intersection"));
        total += priority;
    }

    total
}

fn find_group_badge(group: (&str, &str, &str)) -> Result<char, ()> {
    let a: HashSet<char> = group.0.chars().collect();
    let b: HashSet<char> = group.1.chars().collect();
    let c: HashSet<char> = group.2.chars().collect();

    let intersection_of_a_and_b = a.intersection(&b).cloned().collect::<HashSet<char>>();
    match intersection_of_a_and_b.intersection(&c).next() {
        Some(letter) => Ok(*letter),
        None => Err(()),
    }
}
