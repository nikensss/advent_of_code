use std::{cmp, collections::HashMap, ops::Range};

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

pub fn part_2(input: &str) -> Result<u64, String> {
    let Ok((_, almanac)) = parse_almanac(input) else {
        return Err("Failed to parse almanac".to_string());
    };

    almanac
        .get_closest_location_for_range_of_seeds()
        .ok_or("Failed to find closest seed location".to_string())
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, _) = pair(newline, newline)(input)?;
    let (input, maps) = parse_almanac_maps(input)?;

    let maps = maps
        .into_iter()
        .map(|map| (map.name.clone(), map))
        .collect::<HashMap<_, _>>();

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
    maps: HashMap<String, AlmanacMap>,
}

impl Almanac {
    fn get_closest_seed_location(&self) -> Option<u64> {
        self.seeds
            .iter()
            .map(|seed| self.get_seed_location(seed))
            .min()
    }

    fn get_seed_location(&self, seed: &u64) -> u64 {
        let value = self.get_map("seed-to-soil").map(seed);
        let value = self.get_map("soil-to-fertilizer").map(&value);
        let value = self.get_map("fertilizer-to-water").map(&value);
        let value = self.get_map("water-to-light").map(&value);
        let value = self.get_map("light-to-temperature").map(&value);
        let value = self.get_map("temperature-to-humidity").map(&value);
        let value = self.get_map("humidity-to-location").map(&value);

        value
    }

    fn get_closest_location_for_range_of_seeds(&self) -> Option<u64> {
        self.seeds
            .as_slice()
            .chunks(2)
            .filter_map(|r| {
                let start = r[0];
                let length = r[1];
                let end = start + length;

                self.get_locations_for_range_of_seeds(start..end)
            })
            .flatten()
            .map(|r| r.start)
            .min()
    }

    fn get_locations_for_range_of_seeds(&self, range: Range<u64>) -> Option<Vec<Range<u64>>> {
        let value = self.get_map("seed-to-soil").map_ranges(vec![range]);
        let value = self.get_map("soil-to-fertilizer").map_ranges(value);
        let value = self.get_map("fertilizer-to-water").map_ranges(value);
        let value = self.get_map("water-to-light").map_ranges(value);
        let value = self.get_map("light-to-temperature").map_ranges(value);
        let value = self.get_map("temperature-to-humidity").map_ranges(value);
        let value = self.get_map("humidity-to-location").map_ranges(value);

        Some(value)
    }

    fn get_map(&self, map_name: &str) -> &AlmanacMap {
        self.maps
            .get(map_name)
            .expect(format!("could not find '{}' map", map_name).as_str())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct AlmanacMap {
    name: String,
    ranges: Vec<AlmanacRange>,
}

impl AlmanacMap {
    fn map(&self, seed: &u64) -> u64 {
        for range in &self.ranges {
            if range.contains(seed) {
                return range.map(seed);
            }
        }
        return *seed;
    }

    fn map_ranges(&self, ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        ranges
            .into_iter()
            .map(|r| self.map_range(r))
            .flatten()
            .collect()
    }

    fn map_range(&self, range: Range<u64>) -> Vec<Range<u64>> {
        let mut result = Vec::new();
        let mut current_seed = range.start;

        while range.contains(&current_seed) {
            let range_mapper = match self.get_range_mapper(&current_seed) {
                Some(range) => range,
                None => self.create_range_mapper(current_seed..range.end),
            };
            let range = range_mapper.map_range(current_seed..range.end);
            current_seed = current_seed + (range.end - range.start);
            result.push(range);
        }

        result
    }

    fn get_range_mapper(&self, seed: &u64) -> Option<AlmanacRange> {
        for range in self.ranges.as_slice() {
            if range.contains(seed) {
                return Some(range.clone());
            }
        }

        None
    }

    fn create_range_mapper(&self, range: Range<u64>) -> AlmanacRange {
        let start = range.start;

        let end = self
            .ranges
            .iter()
            .filter_map(|r| {
                if r.source_start > start {
                    return Some(r.source_start);
                }
                None
            })
            .min_by_key(|x| x - start);

        if end.is_none() {
            return AlmanacRange {
                destination_starts: start,
                source_start: start,
                length: range.end - start,
            };
        }

        let end = end.unwrap();

        AlmanacRange {
            destination_starts: start,
            source_start: start,
            length: end - start,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

    fn map_range(&self, range: Range<u64>) -> Range<u64> {
        self.map(&range.start)..self.map(cmp::min(&(self.source_start + self.length), &range.end))
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
                    .into_iter()
                    .map(|map| (map.name.clone(), map))
                    .collect::<HashMap<_, _>>()
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

    #[test]
    fn test_part_2_with_test_input() {
        assert_eq!(part_2(TEST_INPUT), Ok(46));
    }

    #[test]
    fn test_part_2_with_complete_input() {
        assert_eq!(part_2(COMPLETE_INPUT), Ok(2520479));
    }
}
