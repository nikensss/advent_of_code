use std::fs;

fn main() {
    let lines = fs::read_to_string("test-input.txt").unwrap();
    let lines = lines.lines().collect();
    let numbers = part_01(&lines);

    println!("{:?}", numbers);
}

fn part_01(lines: &Vec<&str>) -> Vec<u32> {
    lines
        .into_iter()
        .map(|line| get_associated_number(line))
        .collect()
}

fn get_associated_number(line: &str) -> u32 {
    let mut numbers = line.chars().filter(|c| c.is_digit(10));
    let first = numbers.next().unwrap();
    let last = match numbers.last() {
        Some(n) => n,
        None => first,
    };

    let number = format!("{}{}", first, last);
    number.parse::<u32>().unwrap()
}

#[test]
fn test_part_01_with_test_input() {
    let lines = fs::read_to_string("test-input.txt").unwrap();
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
