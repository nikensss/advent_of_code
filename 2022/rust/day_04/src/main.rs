use std::{collections::HashSet, fs};

fn main() {
    let filename = "input_01.txt";
    let lines = fs::read_to_string(filename).expect("Cannot read file");
    let lines: Vec<&str> = lines.lines().collect();

    let (res_part_01, res_part_02) = part_01_and_02(&lines);
    let is_part_01_ok = if res_part_01 == 567 { "ok" } else { "wrong" };
    let is_part_02_ok = if res_part_02 == 567 { "ok" } else { "wrong" };

    println!("{} ({})", res_part_01, is_part_01_ok);
    println!("{} ({})", res_part_02, is_part_02_ok);
}

fn part_01_and_02(lines: &Vec<&str>) -> (i32, i32) {
    let mut fully_overlapping_sets = 0;
    let mut overlapping_sets = 0;

    for line in lines {
        let mut boundaries_of_sets = line.split(',');

        let set_a = match boundaries_of_sets.next() {
            Some(boundaries) => create_set_from_boundaries(boundaries),
            None => panic!("No boundaries: {}", line),
        };

        let set_b = match boundaries_of_sets.next() {
            Some(boundaries) => create_set_from_boundaries(boundaries),
            None => panic!("No boundaries: {}", line),
        };

        if set_b.is_superset(&set_a) || set_a.is_superset(&set_b) {
            fully_overlapping_sets += 1;
        }

        if !set_a.is_disjoint(&set_b) {
            overlapping_sets += 1;
        }
    }

    (fully_overlapping_sets, overlapping_sets)
}

fn create_set_from_boundaries(boundaries: &str) -> HashSet<u32> {
    let mut split_boundaries = boundaries.split('-');

    let start = match split_boundaries.next() {
        Some(n) => n.parse().expect("Not a number"),
        None => panic!("Not a number"),
    };

    let end = match split_boundaries.next() {
        Some(n) => n.parse().expect("Not a number"),
        None => panic!("No end: {}", boundaries),
    };

    (start..=end).collect::<HashSet<u32>>()
}
