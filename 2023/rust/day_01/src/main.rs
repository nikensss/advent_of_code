use std::fs;

fn main() {
    let lines = fs::read_to_string("test-input-01.txt").unwrap();
    let lines = lines.lines().collect();
    let result_part_01 = part_01(&lines);

    let lines = fs::read_to_string("test-input-02.txt").unwrap();
    let lines = lines.lines().collect();
    let result_part_02 = part_02(&lines);

    println!("{:?}", result_part_01);
    println!("{:?}", result_part_02);
}

fn part_01(lines: &Vec<&str>) -> Vec<u32> {
    lines
        .into_iter()
        .map(|line| {
            let mut numbers = line.chars().filter(|c| c.is_digit(10));
            let first = numbers.next().unwrap();
            let last = match numbers.last() {
                Some(n) => n,
                None => first,
            };

            let number = format!("{}{}", first, last);
            number.parse::<u32>().unwrap()
        })
        .collect()
}

fn part_02(lines: &Vec<&str>) -> Vec<u32> {
    lines
        .into_iter()
        .map(|line| {
            let first = get_first_associated_number(line).unwrap();
            let last = match get_last_associated_number(line) {
                Some(n) => n,
                None => first,
            };

            format!("{}{}", first, last).parse::<u32>().unwrap()
        })
        .collect()
}

fn get_first_associated_number(line: &str) -> Option<u32> {
    for (i, c) in line.chars().enumerate() {
        let s = &line[i..];
        if c.is_digit(10) {
            return c.to_digit(10);
        } else if s.starts_with("one") {
            return Some(1);
        } else if s.starts_with("two") {
            return Some(2);
        } else if s.starts_with("three") {
            return Some(3);
        } else if s.starts_with("four") {
            return Some(4);
        } else if s.starts_with("five") {
            return Some(5);
        } else if s.starts_with("six") {
            return Some(6);
        } else if s.starts_with("seven") {
            return Some(7);
        } else if s.starts_with("eight") {
            return Some(8);
        } else if s.starts_with("nine") {
            return Some(9);
        }
    }

    None
}

fn get_last_associated_number(line: &str) -> Option<u32> {
    for i in (0..=line.len()).rev() {
        let c = line.chars().nth(i - 1).unwrap();
        let s = &line[..i];

        if c.is_digit(10) {
            return c.to_digit(10);
        } else if s.ends_with("one") {
            return Some(1);
        } else if s.ends_with("two") {
            return Some(2);
        } else if s.ends_with("three") {
            return Some(3);
        } else if s.ends_with("four") {
            return Some(4);
        } else if s.ends_with("five") {
            return Some(5);
        } else if s.ends_with("six") {
            return Some(6);
        } else if s.ends_with("seven") {
            return Some(7);
        } else if s.ends_with("eight") {
            return Some(8);
        } else if s.ends_with("nine") {
            return Some(9);
        }
    }

    None
}

#[test]
fn test_part_01_with_test_input() {
    let lines = fs::read_to_string("test-input-01.txt").unwrap();
    let lines = lines.lines().collect();
    let numbers = part_01(&lines);

    assert_eq!(numbers, vec![12, 38, 15, 77]);
}

#[test]
fn test_part_01_with_real_input() {
    let lines = fs::read_to_string("input.txt").unwrap();
    let lines = lines.lines().collect();
    let numbers = part_01(&lines);

    assert_eq!(numbers.into_iter().sum::<u32>(), 54667);
}

#[test]
fn test_part_02_with_test_input() {
    let lines = fs::read_to_string("test-input-02.txt").unwrap();
    let lines = lines.lines().collect();
    let numbers = part_02(&lines);

    assert_eq!(numbers, vec![29, 83, 13, 24, 42, 14, 76]);
}

#[test]
fn test_part_02_with_real_input() {
    let lines = fs::read_to_string("input.txt").unwrap();
    let lines = lines.lines().collect();
    let numbers = part_02(&lines);

    assert_eq!(numbers.into_iter().sum::<u32>(), 54203);
}
