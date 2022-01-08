use std::fs;

fn main() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("Could not read file");
    let contents: Vec<&str> = contents.lines().collect();

    let count = count_bit_occurrence(&contents);
    let gamma = calculate_gamma(&count);
    let epsilon = calculate_epsilon(&count);

    println!("{}", gamma * epsilon)
}

fn count_bit_occurrence(contents: &Vec<&str>) -> Vec<i32> {
    let mut count: Vec<i32> = vec![0; contents[0].len()];
    for line in contents {
        for i in 0..line.len() {
            let character = &line[i..i + 1];
            match character {
                "0" => count[i] = count[i] - 1,
                "1" => count[i] = count[i] + 1,
                _ => panic!("Invalid character found: {}", character),
            }
        }
    }

    count
}

fn calculate_gamma(count: &Vec<i32>) -> isize {
    let mut gamma_binary = String::from("");
    for c in count {
        let most_common = if *c >= 0 { "1" } else { "0" };
        gamma_binary.push_str(most_common);
    }

    isize::from_str_radix(&gamma_binary, 2).unwrap()
}

fn calculate_epsilon(count: &Vec<i32>) -> isize {
    let mut epsilon_binary = String::from("");
    for c in count {
        let least_common = if *c >= 0 { "0" } else { "1" };
        epsilon_binary.push_str(least_common);
    }

    isize::from_str_radix(&epsilon_binary, 2).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_match_part1_solution() {
        let filename = "input.txt";

        let contents = fs::read_to_string(filename).expect("Could not read file");
        let contents: Vec<&str> = contents.lines().collect();

        let count = count_bit_occurrence(&contents);
        let gamma = calculate_gamma(&count);
        let epsilon = calculate_epsilon(&count);

        assert_eq!(gamma * epsilon, 2595824);
    }
}
