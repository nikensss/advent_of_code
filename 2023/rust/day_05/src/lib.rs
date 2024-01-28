use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{newline, space1, u64},
    multi::{separated_list0, separated_list1},
    sequence::pair,
    IResult,
};

pub fn part_1(input: &str) -> Result<u64, String> {
    let Ok((_, almanac)) = parse_almanac(input) else {
        return Err("Failed to parse almanac".to_string());
    };

    almanac
        .get_closest_seed_location()
        .ok_or("Failed to find closest seed location".to_string())
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, _) = pair(newline, newline)(input)?;
    let (input, maps) = parse_almanac_maps(input)?;

    Ok((input, Almanac { seeds, maps }))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list0(space1, u64)(input)?;

    Ok((input, seeds))
}

fn parse_almanac_maps(input: &str) -> IResult<&str, Vec<AlmanacMap>> {
    separated_list1(pair(newline, newline), parse_almanac_map)(input)
}

fn parse_almanac_map(input: &str) -> IResult<&str, AlmanacMap> {
    let (input, map_name) = take_until(" ")(input)?;
    let (input, _) = tag(" map:\n")(input)?;
    let (input, ranges) = separated_list1(newline, parse_almanac_range)(input)?;

    Ok((
        input,
        AlmanacMap {
            name: map_name.to_string(),
            ranges,
        },
    ))
}

fn parse_almanac_range(input: &str) -> IResult<&str, AlmanacRange> {
    let (input, numbers) = separated_list1(space1, u64)(input)?;

    if numbers.len() != 3 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::SeparatedList,
        )));
    }

    Ok((
        input,
        AlmanacRange {
            destination_starts: numbers[0],
            source_start: numbers[1],
            length: numbers[2],
        },
    ))
}

#[derive(Debug, Eq, PartialEq)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn get_closest_seed_location(&self) -> Option<u64> {
        self.seeds
            .iter()
            .map(|seed| self.get_seed_location(seed))
            .min()
    }

    fn get_seed_location(&self, seed: &u64) -> u64 {
        let seed_to_soil_map = self
            .maps
            .iter()
            .find(|map| map.name == "seed-to-soil")
            .unwrap();
        let soil = seed_to_soil_map.map(seed);

        let soil_to_fertilizer_map = self
            .maps
            .iter()
            .find(|map| map.name == "soil-to-fertilizer")
            .unwrap();
        let fertilizer = soil_to_fertilizer_map.map(&soil);

        let fertilizer_to_water_map = self
            .maps
            .iter()
            .find(|map| map.name == "fertilizer-to-water")
            .unwrap();
        let water = fertilizer_to_water_map.map(&fertilizer);

        let water_to_light_map = self
            .maps
            .iter()
            .find(|map| map.name == "water-to-light")
            .unwrap();
        let light = water_to_light_map.map(&water);

        let light_to_temperature_map = self
            .maps
            .iter()
            .find(|map| map.name == "light-to-temperature")
            .unwrap();
        let temperature = light_to_temperature_map.map(&light);

        let temperature_to_humidity_map = self
            .maps
            .iter()
            .find(|map| map.name == "temperature-to-humidity")
            .unwrap();
        let humidity = temperature_to_humidity_map.map(&temperature);

        let humidity_to_location_map = self
            .maps
            .iter()
            .find(|map| map.name == "humidity-to-location")
            .unwrap();
        let location = humidity_to_location_map.map(&humidity);

        location
    }
}

#[derive(Debug, Eq, PartialEq)]
struct AlmanacMap {
    name: String,
    ranges: Vec<AlmanacRange>,
}

impl AlmanacMap {
    fn map(&self, seed: &u64) -> u64 {
        let Some(range) = self.ranges.iter().find(|r| r.contains(seed)) else {
            return *seed;
        };

        range.map(seed)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct AlmanacRange {
    destination_starts: u64,
    source_start: u64,
    length: u64,
}

impl AlmanacRange {
    fn contains(&self, seed: &u64) -> bool {
        (self.source_start..self.source_start + self.length).contains(seed)
    }

    fn map(&self, seed: &u64) -> u64 {
        self.destination_starts + (seed - self.source_start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");
    const COMPLETE_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_parse_seeds() {
        let input = "seeds: 79 14 55 13";
        assert_eq!(parse_seeds(input), Ok(("", vec![79, 14, 55, 13])))
    }

    #[test]
    fn test_parse_almanac_range() {
        let input = "50 98 2";
        assert_eq!(
            parse_almanac_range(input),
            Ok((
                "",
                AlmanacRange {
                    destination_starts: 50,
                    source_start: 98,
                    length: 2
                }
            ))
        )
    }

    #[test]
    fn test_parse_almanac_map() {
        let input = r#"seed-to-soil map:
50 98 2
52 50 48
"#;
        assert_eq!(
            parse_almanac_map(input),
            Ok((
                "\n",
                AlmanacMap {
                    name: "seed-to-soil".to_string(),
                    ranges: vec![
                        AlmanacRange {
                            destination_starts: 50,
                            source_start: 98,
                            length: 2
                        },
                        AlmanacRange {
                            destination_starts: 52,
                            source_start: 50,
                            length: 48
                        }
                    ]
                }
            ))
        )
    }

    #[test]
    fn test_parse_almanac_maps() {
        let input = r#"seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15
"#;

        assert_eq!(
            parse_almanac_maps(input),
            Ok((
                "\n",
                vec![
                    AlmanacMap {
                        name: "seed-to-soil".to_string(),
                        ranges: vec![
                            AlmanacRange {
                                destination_starts: 50,
                                source_start: 98,
                                length: 2,
                            },
                            AlmanacRange {
                                destination_starts: 52,
                                source_start: 50,
                                length: 48,
                            },
                        ],
                    },
                    AlmanacMap {
                        name: "soil-to-fertilizer".to_string(),
                        ranges: vec![
                            AlmanacRange {
                                destination_starts: 0,
                                source_start: 15,
                                length: 37,
                            },
                            AlmanacRange {
                                destination_starts: 37,
                                source_start: 52,
                                length: 2,
                            },
                            AlmanacRange {
                                destination_starts: 39,
                                source_start: 0,
                                length: 15,
                            },
                        ],
                    }
                ]
            ))
        )
    }

    #[test]
    fn test_parse_almanac() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15
"#;

        assert_eq!(
            parse_almanac(input),
            Ok((
                "\n",
                Almanac {
                    seeds: vec![79, 14, 55, 13],
                    maps: vec![
                        AlmanacMap {
                            name: "seed-to-soil".to_string(),
                            ranges: vec![
                                AlmanacRange {
                                    destination_starts: 50,
                                    source_start: 98,
                                    length: 2,
                                },
                                AlmanacRange {
                                    destination_starts: 52,
                                    source_start: 50,
                                    length: 48,
                                },
                            ],
                        },
                        AlmanacMap {
                            name: "soil-to-fertilizer".to_string(),
                            ranges: vec![
                                AlmanacRange {
                                    destination_starts: 0,
                                    source_start: 15,
                                    length: 37,
                                },
                                AlmanacRange {
                                    destination_starts: 37,
                                    source_start: 52,
                                    length: 2,
                                },
                                AlmanacRange {
                                    destination_starts: 39,
                                    source_start: 0,
                                    length: 15,
                                },
                            ],
                        }
                    ]
                }
            ))
        )
    }

    #[test]
    fn test_part_1_with_test_input() {
        assert_eq!(part_1(TEST_INPUT), Ok(35));
    }

    #[test]
    fn test_part_1_with_complete_input() {
        assert_eq!(part_1(COMPLETE_INPUT), Ok(462648396));
    }
}
