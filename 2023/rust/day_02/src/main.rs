use nom::{bytes::complete::tag, bytes::complete::take_until};
use std::{fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("test-input-01.txt").unwrap();
    let games = input
        .lines()
        .map(|l| l.parse::<Game>().unwrap())
        .collect::<Vec<_>>();
    let possible_games = games.iter().filter(|g| g.is_possible());
    let sum_of_indexes = possible_games.map(|g| g.get_index()).sum::<usize>();

    println!("Sum of indexes: {}", sum_of_indexes);

    let minimum_sets = games
        .iter()
        .map(|g| g.get_minimum_sets_of_cubes())
        .collect::<Vec<_>>();
    println!("minimum sets: {:?}", minimum_sets);

    let powers = minimum_sets
        .iter()
        .map(|set| set.get_power())
        .collect::<Vec<_>>();
    println!("powers: {:?}", powers);

    let sum_of_powers = powers.iter().sum::<usize>();

    println!("Sum of powers of minimum sets of cubes: {}", sum_of_powers);
}

#[derive(Debug)]
enum Cube {
    Red(usize),
    Green(usize),
    Blue(usize),
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (color, amount) = take_until::<_, _, ()>(" ")(input.trim()).map_err(|_| ())?;
        let amount = amount.trim().parse::<usize>().map_err(|_| ())?;
        let cube = match color.trim() {
            "red" => Cube::Red(amount),
            "green" => Cube::Green(amount),
            "blue" => Cube::Blue(amount),
            _ => return Err(()),
        };

        Ok(cube)
    }
}

impl Cube {
    fn is_possible(&self) -> bool {
        match self {
            Cube::Red(amount) => amount <= &12,
            Cube::Green(amount) => amount <= &13,
            Cube::Blue(amount) => amount <= &14,
        }
    }
}

#[derive(Debug)]
struct Set {
    cubes: Vec<Cube>,
}

impl Set {
    fn is_possible(&self) -> bool {
        self.cubes.iter().all(|c| c.is_possible())
    }

    fn get_red_amount(&self) -> Option<usize> {
        self.cubes
            .iter()
            .find(|c| match c {
                Cube::Red(_) => true,
                _ => false,
            })
            .map(|c| match c {
                Cube::Red(amount) => *amount,
                _ => unreachable!(),
            })
    }

    fn get_green_amount(&self) -> Option<usize> {
        self.cubes
            .iter()
            .find(|c| match c {
                Cube::Green(_) => true,
                _ => false,
            })
            .map(|c| match c {
                Cube::Green(amount) => *amount,
                _ => unreachable!(),
            })
    }

    fn get_blue_amount(&self) -> Option<usize> {
        self.cubes
            .iter()
            .find(|c| match c {
                Cube::Blue(_) => true,
                _ => false,
            })
            .map(|c| match c {
                Cube::Blue(amount) => *amount,
                _ => unreachable!(),
            })
    }

    fn get_power(&self) -> usize {
        self.cubes
            .iter()
            .map(|c| match c {
                Cube::Red(amount) => amount,
                Cube::Green(amount) => amount,
                Cube::Blue(amount) => amount,
            })
            .fold(1, |a, b| a * b)
    }
}

impl FromStr for Set {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let cubes = input.split(", ").collect::<Vec<_>>();

        Ok(Set {
            cubes: cubes
                .iter()
                .map(|c| c.parse::<Cube>())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug)]
struct Game {
    index: usize,
    sets: Vec<Set>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.sets.iter().all(|s| s.is_possible())
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_minimum_sets_of_cubes(&self) -> Set {
        let mut red = Cube::Red(0);
        let mut green = Cube::Green(0);
        let mut blue = Cube::Blue(0);

        for set in &self.sets {
            if let Some(red_amount) = set.get_red_amount() {
                red = match red {
                    Cube::Red(amount) => {
                        if amount > red_amount {
                            Cube::Red(amount)
                        } else {
                            Cube::Red(red_amount)
                        }
                    }
                    _ => unreachable!(),
                };
            }

            if let Some(green_amount) = set.get_green_amount() {
                green = match green {
                    Cube::Green(amount) => {
                        if amount > green_amount {
                            Cube::Green(amount)
                        } else {
                            Cube::Green(green_amount)
                        }
                    }
                    _ => unreachable!(),
                };
            }

            if let Some(blue_amount) = set.get_blue_amount() {
                blue = match blue {
                    Cube::Blue(amount) => {
                        if amount > blue_amount {
                            Cube::Blue(amount)
                        } else {
                            Cube::Blue(blue_amount)
                        }
                    }
                    _ => unreachable!(),
                };
            }
        }

        Set {
            cubes: vec![red, green, blue],
        }
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (input, _) = tag::<_, _, ()>("Game ")(input).map_err(|_| ())?;
        let (input, index) = take_until::<_, _, ()>(": ")(input).map_err(|_| ())?;
        let (input, _) = tag::<_, _, ()>(": ")(input).map_err(|_| ())?;
        let sets = input.split("; ").collect::<Vec<_>>();

        Ok(Game {
            index: index.parse::<usize>().map_err(|_| ())?,
            sets: sets
                .iter()
                .map(|s| s.parse::<Set>())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[test]
fn test_part_01_with_test_input() {
    let input = fs::read_to_string("test-input-01.txt").unwrap();
    let games = input
        .lines()
        .map(|l| l.parse::<Game>().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(
        games
            .iter()
            .filter(|g| g.is_possible())
            .map(|g| g.get_index())
            .sum::<usize>(),
        8
    );
}

#[test]
fn test_part_02_with_test_input() {
    let input = fs::read_to_string("test-input-01.txt").unwrap();
    let games = input
        .lines()
        .map(|l| l.parse::<Game>().unwrap())
        .collect::<Vec<_>>();

    let powers = games
        .iter()
        .map(|g| g.get_minimum_sets_of_cubes())
        .map(|set| set.get_power())
        .collect::<Vec<_>>();

    assert_eq!(powers, vec![48, 12, 1560, 630, 36]);
}

#[test]
fn test_part_01_with_complete_input() {
    let input = fs::read_to_string("input-01.txt").unwrap();
    let games = input
        .lines()
        .map(|l| l.parse::<Game>().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(
        games
            .iter()
            .filter(|g| g.is_possible())
            .map(|g| g.get_index())
            .sum::<usize>(),
        2239
    );
}

#[test]
fn test_part_02_with_complete_input() {
    let input = fs::read_to_string("input-01.txt").unwrap();
    let games = input
        .lines()
        .map(|l| l.parse::<Game>().unwrap())
        .collect::<Vec<_>>();

    let sum_of_powers = games
        .iter()
        .map(|g| g.get_minimum_sets_of_cubes())
        .map(|set| set.get_power())
        .sum::<usize>();

    assert_eq!(sum_of_powers, 83435);
}
