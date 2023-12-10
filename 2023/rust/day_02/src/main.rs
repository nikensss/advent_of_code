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
}

#[derive(Debug)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (color, amount) = take_until::<_, _, ()>(" ")(input.trim()).map_err(|_| ())?;
        let amount = amount.trim().parse::<u32>().map_err(|_| ())?;
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
