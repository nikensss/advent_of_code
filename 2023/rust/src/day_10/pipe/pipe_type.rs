use std::{fmt::Display, str::FromStr};

use crate::day_10::direction::Direction;

#[derive(Debug, PartialEq, Eq)]
pub enum PipeType {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Start,
    Ground,
}

impl FromStr for PipeType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(PipeType::NS),
            "-" => Ok(PipeType::EW),
            "L" => Ok(PipeType::NE),
            "J" => Ok(PipeType::NW),
            "7" => Ok(PipeType::SW),
            "F" => Ok(PipeType::SE),
            "S" => Ok(PipeType::Start),
            "." => Ok(PipeType::Ground),
            _ => Err(format!("Invalid pipe type: {}", s)),
        }
    }
}

impl Display for PipeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PipeType::NS => "|",
            PipeType::EW => "-",
            PipeType::NE => "L",
            PipeType::NW => "J",
            PipeType::SW => "7",
            PipeType::SE => "F",
            PipeType::Start => "S",
            PipeType::Ground => ".",
        };
        write!(f, "{}", s)
    }
}

impl PipeType {
    pub fn is_connected_to(&self, direction: &Direction) -> bool {
        match self {
            PipeType::NS => matches!(direction, Direction::North | Direction::South),
            PipeType::NW => matches!(direction, Direction::North | Direction::West),
            PipeType::NE => matches!(direction, Direction::North | Direction::East),
            PipeType::SW => matches!(direction, Direction::South | Direction::West),
            PipeType::SE => matches!(direction, Direction::South | Direction::East),
            PipeType::EW => matches!(direction, Direction::East | Direction::West),
            PipeType::Start => false,
            PipeType::Ground => false,
        }
    }
}
