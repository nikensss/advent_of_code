use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace1, newline, u64},
    combinator::eof,
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    fn count_possible_ways_to_win(&self) -> u64 {
        let mut possible_ways_to_win = 0;

        for i in 0..=self.time {
            let (speed, time_left) = (i, self.time - i);
            let distance = speed * time_left;
            if distance > self.record_distance {
                possible_ways_to_win += 1;
            }
        }

        possible_ways_to_win
    }
}

pub fn part_1(input: &str) -> Result<u64, String> {
    let Ok((_, races)) = parse_input(input) else {
        return Err("Failed to parse input".to_string());
    };

    Ok(races
        .into_iter()
        .map(|r| r.count_possible_ways_to_win())
        .product::<u64>())
}

fn parse_input(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, time) = parse_time(input)?;
    let (input, _) = newline(input)?;
    let (input, distance) = parse_distance(input)?;

    let races = time
        .into_iter()
        .zip(distance.into_iter())
        .map(|(t, d)| Race {
            time: t,
            record_distance: d,
        })
        .collect::<Vec<_>>();

    Ok((input, races))
}

fn parse_time(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, numbers) = parse_numbers(input)?;

    Ok((input, numbers))
}

fn parse_distance(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("Distance:")(input)?;
    let (input, numbers) = parse_numbers(input)?;

    Ok((input, numbers))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(multispace1, separated_list1(multispace1, u64))(input)
}

pub fn part_2(input: &str) -> Result<u64, String> {
    let Ok((_, race)) = parse_input_as_one_number(input) else {
        return Err("Failed to parse input".to_string());
    };

    Ok(race.count_possible_ways_to_win())
}

fn parse_input_as_one_number(input: &str) -> IResult<&str, Race> {
    let (input, time) = parse_time_as_one_number(input)?;
    let (input, distance) = parse_distance_as_one_number(input)?;

    Ok((
        input,
        Race {
            time,
            record_distance: distance,
        },
    ))
}
fn parse_time_as_one_number(input: &str) -> IResult<&str, u64> {
    let (input, _) = tag("Time:")(input)?;
    let (input, chars) = parse_as_one_number(input)?;

    Ok((input, chars.concat().parse().unwrap()))
}

fn parse_distance_as_one_number(input: &str) -> IResult<&str, u64> {
    let (input, _) = tag("Distance:")(input)?;
    let (input, chars) = parse_as_one_number(input)?;

    Ok((input, chars.concat().parse().unwrap()))
}

fn parse_as_one_number(input: &str) -> IResult<&str, Vec<&str>> {
    terminated(
        preceded(multispace1, separated_list1(multispace1, digit1)),
        alt((line_ending, eof)),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../day_06/test_input.txt");
    const COMPLETE_INPUT: &str = include_str!("../day_06/complete_input.txt");

    #[test]
    fn test_parse_time() {
        let input = r#"Time:      7  15   30"#;
        assert_eq!(parse_time(input), Ok(("", vec![7, 15, 30])));
    }

    #[test]
    fn test_parse_distance() {
        let input = r#"Distance:  9  40  200"#;
        assert_eq!(parse_distance(input), Ok(("", vec![9, 40, 200])));
    }

    #[test]
    fn test_parse_input() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        assert_eq!(
            parse_input(input),
            Ok((
                "",
                vec![
                    Race {
                        time: 7,
                        record_distance: 9
                    },
                    Race {
                        time: 15,
                        record_distance: 40
                    },
                    Race {
                        time: 30,
                        record_distance: 200
                    }
                ]
            ))
        );
    }

    #[test]
    fn test_possible_ways_to_win() {
        let race = Race {
            time: 7,
            record_distance: 9,
        };
        assert_eq!(race.count_possible_ways_to_win(), 4);

        let race = Race {
            time: 15,
            record_distance: 40,
        };
        assert_eq!(race.count_possible_ways_to_win(), 8);

        let race = Race {
            time: 30,
            record_distance: 200,
        };
        assert_eq!(race.count_possible_ways_to_win(), 9);
    }

    #[test]
    fn test_part_1_with_test_input() {
        assert_eq!(part_1(TEST_INPUT), Ok(288));
    }

    #[test]
    fn test_part_1_with_complete_input() {
        assert_eq!(part_1(COMPLETE_INPUT), Ok(3317888));
    }

    #[test]
    fn test_parse_time_as_one_number() {
        let input = r#"Time:      7  15   30"#;
        assert_eq!(parse_time_as_one_number(input), Ok(("", 71530)));
    }

    #[test]
    fn test_parse_distance_as_one_number() {
        let input = r#"Distance:  9  40  200"#;
        assert_eq!(parse_distance_as_one_number(input), Ok(("", 940200)));
    }

    #[test]
    fn test_parse_input_as_one_number() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        assert_eq!(
            parse_input_as_one_number(input),
            Ok((
                "",
                Race {
                    time: 71530,
                    record_distance: 940200
                }
            ))
        );
    }

    #[test]
    fn test_part_2_with_test_input() {
        assert_eq!(part_2(TEST_INPUT), Ok(71503));
    }

    #[test]
    fn test_part_2_with_complete_input() {
        assert_eq!(part_2(COMPLETE_INPUT), Ok(24655068));
    }
}
