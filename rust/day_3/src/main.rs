use std::fs;

fn main() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("Could not read file");
    let contents: Vec<&str> = contents.lines().collect();

    let count = count_bit_occurrence(&contents);
    let gamma = calculate_gamma(&count);
    let epsilon = calculate_epsilon(&count);

    let oxygen = calculate_oxygen(&contents);
    let co2 = calculate_co2(&contents);

    println!("{} * {} = {}", gamma, epsilon, gamma * epsilon);
    println!("{} * {} = {}", oxygen, co2, oxygen * co2);
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

fn calculate_oxygen(contents: &Vec<&str>) -> isize {
    let mut oxygen_candidates = contents.clone();
    let total_bits = oxygen_candidates[0].len();
    for i in 0..total_bits {
        if oxygen_candidates.len() == 1 {
            break;
        }

        let moving_count = count_bit_occurrence(&oxygen_candidates);
        let most_common = if moving_count[i] >= 0 { "1" } else { "0" };
        oxygen_candidates = oxygen_candidates
            .into_iter()
            .filter(|candidate| &candidate[i..i + 1] == most_common)
            .collect::<Vec<&str>>();
    }

    isize::from_str_radix(&oxygen_candidates[0], 2).unwrap()
}

fn calculate_co2(contents: &Vec<&str>) -> isize {
    let mut co2_candidates: Vec<&str> = contents.clone();
    let total_bits = co2_candidates[0].len();
    for i in 0..total_bits {
        if co2_candidates.len() == 1 {
            break;
        }

        let moving_count = count_bit_occurrence(&co2_candidates);
        let least_common = if moving_count[i] >= 0 { "0" } else { "1" };
        co2_candidates = co2_candidates
            .into_iter()
            .filter(|candidate| &candidate[i..i + 1] == least_common)
            .collect::<Vec<&str>>();
    }

    isize::from_str_radix(&co2_candidates[0], 2).unwrap()
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

    #[test]
    fn should_match_part2_solution() {
        let filename = "input.txt";

        let contents = fs::read_to_string(filename).expect("Could not read file");
        let contents: Vec<&str> = contents.lines().collect();

        let oxygen = calculate_oxygen(&contents);
        let co2 = calculate_co2(&contents);

        assert_eq!(oxygen * co2, 2135254);
    }
}
