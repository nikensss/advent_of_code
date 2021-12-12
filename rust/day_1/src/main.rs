use std::fs;

fn main() {
    part_1();
}

fn part_1() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("Could not read file");
    let contents = contents
        .lines()
        .map(|x| x.parse::<u32>().expect("Not a number"))
        .collect::<Vec<u32>>();

    let mut increases = 0;

    for i in 0..contents.len() - 1 {
        if contents[i + 1] > contents[i] {
            increases += 1
        }
    }

    println!("Increases: {}", increases)
}
