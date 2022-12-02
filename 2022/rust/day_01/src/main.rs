use std::fs;

fn main() {
    let filename = "input_01.txt";
    let calories = fs::read_to_string(filename).expect("Could not read file");
    let calories = calories.lines().collect::<Vec<&str>>();

    let res_1 = part_1(&calories);
    println!(
        "{} ({})",
        res_1,
        if res_1 == 71506 { "ok" } else { "wrong" }
    );

    let res_2 = part_2(&calories);
    println!(
        "{} ({})",
        res_2,
        if res_2 == 209603 { "ok" } else { "wrong" }
    );
}

fn part_1(calories: &Vec<&str>) -> u32 {
    let mut current_count = 0;
    let mut max = 0;

    for line in calories {
        if let Ok(n) = line.parse::<u32>() {
            current_count += n;
        } else {
            max = if current_count > max {
                current_count
            } else {
                max
            };
            current_count = 0;
        };
    }

    max
}

fn part_2(calories: &Vec<&str>) -> u32 {
    let mut current_count = 0;
    let mut calorie_count = Vec::new();

    for line in calories {
        if let Ok(n) = line.parse::<u32>() {
            current_count += n;
        } else {
            calorie_count.push(current_count);
            current_count = 0;
        };
    }

    calorie_count.sort();
    calorie_count.iter().rev().take(3).sum()
}
