use std::fs;

fn main() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("Could not read file");
    let contents: Vec<&str> = contents.lines().collect();

    let gamma = calculate_gamma(&contents);
    let epsilon = calculate_epsilon(&contents);

    let oxygen = calculate_oxygen(&contents);
    let co2 = calculate_co2(&contents);

    println!("{} * {} = {}", gamma, epsilon, gamma * epsilon);
    println!("{} * {} = {}", oxygen, co2, oxygen * co2);
}

fn find_most_common<'a>(contents: &'a Vec<&str>) -> Vec<&'a str> {
    count_bit_occurrence(&contents)
        .into_iter()
        .map(|x| if x >= 0 { "1" } else { "0" })
        .collect::<Vec<&str>>()
}

fn find_least_common<'a>(contents: &'a Vec<&str>) -> Vec<&'a str> {
    count_bit_occurrence(&contents)
        .into_iter()
        .map(|x| if x >= 0 { "0" } else { "1" })
        .collect::<Vec<&str>>()
}

fn count_bit_occurrence(contents: &Vec<&str>) -> Vec<i32> {
    let mut count: Vec<i32> = vec![0; contents[0].len()];
    for i in 0..count.len() {
        count[i] = count_bit_occurrence_at(&contents, i);
    }

    count
}

fn count_bit_occurrence_at(contents: &Vec<&str>, index: usize) -> i32 {
    let mut occurence = 0;
    for line in contents {
        let character = &line[index..index + 1];
        match character {
            "0" => occurence -= 1,
            "1" => occurence += 1,
            _ => panic!("Invalid character found: {}", character),
        };
    }

    occurence
}

fn calculate_gamma(contents: &Vec<&str>) -> isize {
    let mut gamma_binary = String::from("");
    let most_common = find_most_common(&contents);
    for c in most_common {
        gamma_binary.push_str(c);
    }

    isize::from_str_radix(&gamma_binary, 2).unwrap()
}

fn calculate_epsilon(contents: &Vec<&str>) -> isize {
    let mut epsilon_binary = String::from("");
    let least_common = find_least_common(&contents);
    for c in least_common {
        epsilon_binary.push_str(c);
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

        let moving_count = count_bit_occurrence_at(&oxygen_candidates, i);
        let most_common = if moving_count >= 0 { "1" } else { "0" };
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

        let moving_count = count_bit_occurrence_at(&co2_candidates, i);
        let least_common = if moving_count >= 0 { "0" } else { "1" };
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

        let gamma = calculate_gamma(&contents);
        let epsilon = calculate_epsilon(&contents);

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
