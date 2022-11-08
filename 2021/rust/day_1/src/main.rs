use std::fs;

fn main() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("Could not read file");
    let contents = contents
        .lines()
        .map(|x| x.parse::<u32>().expect("Not a number"))
        .collect::<Vec<u32>>();

    part_1(&contents, 1676);
    part_2(&contents, 1706);
}

fn part_1(contents: &Vec<u32>, expectation: u32) {
    let mut increases = 0;

    for i in 0..contents.len() - 1 {
        if contents[i + 1] > contents[i] {
            increases += 1
        }
    }

    println!("{} ({})", increases, increases == expectation)
}

fn part_2(contents: &Vec<u32>, expectation: u32) {
    let mut rolling_window: Vec<u32> = Vec::new();

    for i in 0..contents.len() - 2 {
        rolling_window.push(contents[i] + contents[i + 1] + contents[i + 2])
    }

    part_1(&rolling_window, expectation)
}
