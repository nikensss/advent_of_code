use nom::{
    branch::alt,
    character::complete::{i64, line_ending, space0},
    combinator::eof,
    multi::many1,
    sequence::{pair, terminated},
    IResult,
};

pub fn part_1(input: &str) -> i64 {
    let Ok((_, oasis_report)) = parse_input(input) else {
        panic!("Failed to parse input: {:?}", input);
    };

    oasis_report
        .into_iter()
        .map(|x| extrapolate_forward(x))
        .sum()
}

fn extrapolate_forward(report_line: Vec<i64>) -> i64 {
    let mut iterations = process_report_line(report_line);
    iterations.last_mut().unwrap().push(0);

    iterations.iter().map(|x| x.iter().last().unwrap()).sum()
}

pub fn part_2(input: &str) -> i64 {
    let Ok((_, oasis_report)) = parse_input(input) else {
        panic!("Failed to parse input: {:?}", input);
    };

    oasis_report
        .into_iter()
        .map(|x| extrapolate_backwards(x))
        .sum()
}

fn extrapolate_backwards(report_line: Vec<i64>) -> i64 {
    let mut iterations = process_report_line(report_line);
    iterations.last_mut().unwrap().insert(0, 0);

    iterations
        .into_iter()
        .map(|x| x.into_iter().next().unwrap())
        .rev()
        .fold(0, |acc, x| x - acc)
}

fn process_report_line(report_line: Vec<i64>) -> Vec<Vec<i64>> {
    let mut iterations: Vec<Vec<i64>> = vec![report_line];

    while !iterations.last().unwrap().iter().all(|x| *x == 0) {
        let current = iterations.last().unwrap();
        let next = process_report_line_iteration(current);
        iterations.push(next);
    }

    iterations
}

fn process_report_line_iteration(report_line_iteration: &Vec<i64>) -> Vec<i64> {
    report_line_iteration
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    many1(parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    terminated(many1(parse_number), alt((line_ending, eof)))(input)
}

fn parse_number(input: &str) -> IResult<&str, i64> {
    let (input, (number, _)) = pair(i64, space0)(input)?;
    Ok((input, number))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../day_09/test_input.txt");
    const COMPLETE_INPUT: &str = include_str!("../day_09/complete_input.txt");

    #[test]
    fn test_parse_line() {
        let input = "1 3 6 10 15 21";
        let result = parse_line(input);
        assert_eq!(result, Ok(("", vec![1, 3, 6, 10, 15, 21])));

        let input = "-4 -7 0 27 90";
        let result = parse_line(input);
        assert_eq!(result, Ok(("", vec![-4, -7, 0, 27, 90])));
    }

    #[test]
    fn test_parse_lines() {
        let input = "1 3 6 10 15 21\n0 3 6 9 12 15";
        let result = parse_input(input);
        assert_eq!(
            result,
            Ok((
                "",
                vec![vec![1, 3, 6, 10, 15, 21], vec![0, 3, 6, 9, 12, 15]]
            ))
        );
    }

    #[test]
    fn test_process_report_line_iteration() {
        let input: Vec<i64> = vec![1, 3, 6, 10, 15, 21];
        let result = process_report_line_iteration(&input);
        assert_eq!(result, vec![2, 3, 4, 5, 6]);

        let result = process_report_line_iteration(&result);
        assert_eq!(result, vec![1, 1, 1, 1]);
    }

    #[test]
    fn test_process_report_line() {
        let input: Vec<i64> = vec![1, 3, 6, 10, 15, 21];
        let result = extrapolate_forward(input);
        assert_eq!(result, 28);

        let input: Vec<i64> = vec![0, 3, 6, 9, 12, 15];
        let result = extrapolate_forward(input);
        assert_eq!(result, 18);

        let input: Vec<i64> = vec![10, 13, 16, 21, 30, 45];
        let result = extrapolate_forward(input);
        assert_eq!(result, 68);
    }

    #[test]
    fn test_part_1_with_test_input() {
        assert_eq!(part_1(TEST_INPUT), 114);
    }

    #[test]
    fn test_part_1_with_complete_input() {
        assert_eq!(part_1(COMPLETE_INPUT), 2101499000);
    }

    #[test]
    fn test_part_2_with_test_input() {
        assert_eq!(part_2(TEST_INPUT), 2);
    }

    #[test]
    fn test_part_2_with_complete_input() {
        assert_eq!(part_2(COMPLETE_INPUT), 1089);
    }
}
