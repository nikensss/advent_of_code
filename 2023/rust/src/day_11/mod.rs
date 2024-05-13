mod universe;

use universe::Universe;

pub fn part_1(input: &str) -> usize {
    let mut universe = Universe::new(input);
    universe.expand();

    universe
        .get_distances_between_galaxies()
        .iter()
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = include_str!("./test-input-1.txt");
    const COMPLETE_INPUT: &str = include_str!("./complete-input.txt");

    #[test]
    fn test_part_1_with_test_input() {
        assert_eq!(part_1(TEST_INPUT_1), 374)
    }
    #[test]
    fn test_part_1_with_complete_input() {
        assert_eq!(part_1(COMPLETE_INPUT), 374)
    }
}
