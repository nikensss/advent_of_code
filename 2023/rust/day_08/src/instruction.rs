use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Right,
    Left,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Instruction::Right),
            "L" => Ok(Instruction::Left),
            _ => Err(()),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Right => write!(f, "R"),
            Instruction::Left => write!(f, "L"),
        }
    }
}
