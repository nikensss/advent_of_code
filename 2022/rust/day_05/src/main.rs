use std::{fs, str::FromStr};

use nom::{
    bytes::complete::tag,
    bytes::complete::{take, take_until},
    error::Error,
    sequence::delimited,
};

fn main() {
    let lines = fs::read_to_string("sample.txt").unwrap();
    let crates_description = get_crates_description(&lines);
    let moves = get_moves(&lines);
    println!("{:?}", crates_description);
    println!("{:?}", moves);

    let crates: Vec<Vec<Option<Crate>>> = crates_description
        .iter()
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|maybe_crate| {
                    let maybe_crate = maybe_crate.iter().collect::<String>();
                    return maybe_crate.parse::<Crate>().ok();
                })
                .collect::<Vec<Option<Crate>>>()
        })
        .collect();
    println!("{:?}", crates);

    let moves: Vec<Move> = moves
        .iter()
        .map(|line| line.parse::<Move>().unwrap())
        .collect();
    println!("{:?}", moves);
}

#[derive(Debug)]
struct Crate {
    name: String,
}

impl FromStr for Crate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = delimited::<_, _, _, _, Error<_>, _, _, _>(tag("["), take(1u8), tag("]"));
        let (_, name) = parser(s).map_err(|_| ())?;

        return Ok(Crate {
            name: name.to_string(),
        });
    }
}

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, _) = tag::<_, _, Error<_>>("move ")(s).map_err(|_| ())?;
        let (s, amount) = take_until::<_, _, Error<_>>(" ")(s).map_err(|_| ())?;
        let (s, _) = tag::<_, _, Error<_>>(" from ")(s).map_err(|_| ())?;
        let (s, from) = take_until::<_, _, Error<_>>(" ")(s).map_err(|_| ())?;
        let (to, _) = tag::<_, _, Error<_>>(" to ")(s).map_err(|_| ())?;

        return Ok(Move {
            from: from.parse::<usize>().map_err(|_| ())?,
            to: to.parse::<usize>().map_err(|_| ())?,
            amount: amount.parse::<usize>().map_err(|_| ())?,
        });
    }
}

fn get_crates_description(lines: &str) -> Vec<&str> {
    lines.lines().take_while(|line| !line.is_empty()).collect()
}

fn get_moves(lines: &str) -> Vec<&str> {
    lines
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .collect()
}
