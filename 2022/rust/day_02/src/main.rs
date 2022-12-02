use int_enum::IntEnum;
use std::{fmt::Display, fs, str::FromStr};

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
enum OpponentMove {
    A = 1,
    B = 2,
    C = 3,
}

impl Display for OpponentMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpponentMove::A => write!(f, "Rock"),
            OpponentMove::B => write!(f, "Paper"),
            OpponentMove::C => write!(f, "Scissors"),
        }
    }
}

impl FromStr for OpponentMove {
    type Err = ();

    fn from_str(s: &str) -> Result<OpponentMove, ()> {
        match s {
            "A" => Ok(OpponentMove::A),
            "B" => Ok(OpponentMove::B),
            "C" => Ok(OpponentMove::C),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
enum MyMove {
    X = 1,
    Y = 2,
    Z = 3,
}

impl Display for MyMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyMove::X => write!(f, "Rock"),
            MyMove::Y => write!(f, "Paper"),
            MyMove::Z => write!(f, "Scissors"),
        }
    }
}

impl FromStr for MyMove {
    type Err = ();

    fn from_str(s: &str) -> Result<MyMove, ()> {
        match s {
            "X" => Ok(MyMove::X),
            "Y" => Ok(MyMove::Y),
            "Z" => Ok(MyMove::Z),
            _ => Err(()),
        }
    }
}

impl MyMove {
    fn get_outcome(&self, opponent_move: OpponentMove) -> Outcome {
        match opponent_move {
            OpponentMove::A => match self {
                MyMove::X => Outcome::Draw,
                MyMove::Y => Outcome::Win,
                MyMove::Z => Outcome::Lose,
            },
            OpponentMove::B => match self {
                MyMove::X => Outcome::Lose,
                MyMove::Y => Outcome::Draw,
                MyMove::Z => Outcome::Win,
            },
            OpponentMove::C => match self {
                MyMove::X => Outcome::Win,
                MyMove::Y => Outcome::Lose,
                MyMove::Z => Outcome::Draw,
            },
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn main() {
    let filename = "input_01.txt";
    let lines = fs::read_to_string(filename).expect("Could not read file");

    let total_score = get_total_score(&lines);
    let is_ok = if total_score == 11475 { "ok" } else { "wrong" };

    println!("{} ({})", total_score, is_ok);
}

fn get_total_score(lines: &str) -> u32 {
    let mut result: u32 = 0;
    for line in lines.lines() {
        let match_moves: Vec<&str> = line.split(' ').collect();
        let Ok(my_move) = match_moves[1].parse::<MyMove>() else { panic!("{:?}",match_moves) };
        let Ok(opponent_move) = match_moves[0].parse::<OpponentMove>() else { panic!("{:?}",match_moves) };

        let outcome = my_move.get_outcome(opponent_move);

        result += my_move.int_value() + outcome.int_value();
    }

    result
}
