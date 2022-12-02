use int_enum::IntEnum;
use std::{fs, str::FromStr};

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
enum OpponentMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for OpponentMove {
    type Err = ();

    fn from_str(s: &str) -> Result<OpponentMove, ()> {
        match s {
            "A" => Ok(OpponentMove::Rock),
            "B" => Ok(OpponentMove::Paper),
            "C" => Ok(OpponentMove::Scissors),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
enum MyMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for MyMove {
    type Err = ();

    fn from_str(s: &str) -> Result<MyMove, ()> {
        match s {
            "X" => Ok(MyMove::Rock),
            "Y" => Ok(MyMove::Paper),
            "Z" => Ok(MyMove::Scissors),
            _ => Err(()),
        }
    }
}

impl MyMove {
    fn get_outcome(&self, opponent_move: OpponentMove) -> Outcome {
        match opponent_move {
            OpponentMove::Rock => match self {
                MyMove::Rock => Outcome::Draw,
                MyMove::Paper => Outcome::Win,
                MyMove::Scissors => Outcome::Lose,
            },
            OpponentMove::Paper => match self {
                MyMove::Rock => Outcome::Lose,
                MyMove::Paper => Outcome::Draw,
                MyMove::Scissors => Outcome::Win,
            },
            OpponentMove::Scissors => match self {
                MyMove::Rock => Outcome::Win,
                MyMove::Paper => Outcome::Lose,
                MyMove::Scissors => Outcome::Draw,
            },
        }
    }

    fn from_desired_outcome(outcome: Outcome, opponent_move: OpponentMove) -> MyMove {
        match outcome {
            Outcome::Lose => match opponent_move {
                OpponentMove::Rock => MyMove::Scissors,
                OpponentMove::Paper => MyMove::Rock,
                OpponentMove::Scissors => MyMove::Paper,
            },
            Outcome::Draw => match opponent_move {
                OpponentMove::Rock => MyMove::Rock,
                OpponentMove::Paper => MyMove::Paper,
                OpponentMove::Scissors => MyMove::Scissors,
            },
            Outcome::Win => match opponent_move {
                OpponentMove::Rock => MyMove::Paper,
                OpponentMove::Paper => MyMove::Scissors,
                OpponentMove::Scissors => MyMove::Rock,
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

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Outcome, ()> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

fn main() {
    let filename = "input_01.txt";
    let strategy_guide = fs::read_to_string(filename).expect("Could not read file");
    let strategy_guide = strategy_guide.lines().collect::<Vec<&str>>();

    let part_1_score = part_1(&strategy_guide);
    let part_1_ok = if part_1_score == 11475 { "ok" } else { "wrong" };
    println!("{} ({})", part_1_score, part_1_ok);

    let part_2_score = part_2(&strategy_guide);
    let part_2_ok = if part_2_score == 16862 { "ok" } else { "wrong" };
    println!("{} ({})", part_2_score, part_2_ok)
}

fn part_1(strategy_guide: &Vec<&str>) -> u32 {
    let mut result: u32 = 0;
    for line in strategy_guide {
        let strategy: Vec<&str> = line.split(' ').collect();
        let Ok(opponent_move) = strategy[0].parse::<OpponentMove>() else {
            panic!("Cannot parse opponent move: {}", strategy[0]);
        };
        let Ok(my_move) = strategy[1].parse::<MyMove>() else {
            panic!("Cannot parse my move: {}", strategy[1]);
        };

        let outcome = my_move.get_outcome(opponent_move);

        result += my_move.int_value() + outcome.int_value();
    }

    result
}

fn part_2(strategy_guide: &Vec<&str>) -> u32 {
    let mut result: u32 = 0;

    for line in strategy_guide {
        let strategy: Vec<&str> = line.split(' ').collect();
        let Ok(opponent_move) = strategy[0].parse::<OpponentMove>() else {
            panic!("Cannot parse opponent move: {}", strategy[0]);
        };
        let Ok(desired_outcome) = strategy[1].parse::<Outcome>() else {
            panic!("Cannot parse desired outcome: {}", strategy[1]);
        };

        let my_move = MyMove::from_desired_outcome(desired_outcome, opponent_move);

        result += my_move.int_value() + desired_outcome.int_value();
    }

    result
}
