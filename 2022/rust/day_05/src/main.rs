use std::{collections::HashMap, fs, str::FromStr};

use nom::{
    bytes::complete::tag,
    bytes::complete::{take, take_until},
    error::Error,
    sequence::delimited,
};

fn main() {
    let lines = fs::read_to_string("input_01.txt").unwrap();
    let crates_description = get_crates_description(&lines);
    let moves = get_moves(&lines);

    let crate_rows: Vec<Vec<Option<Crate>>> = crates_description
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

    let moves: Vec<Move> = moves
        .iter()
        .map(|line| line.parse::<Move>().unwrap())
        .collect();

    let mut stacks: Vec<CrateStack> = vec![];
    for i in 0..crate_rows.get(0).unwrap().len() {
        stacks.push(CrateStack::new(i + 1));
    }

    for stack in &mut stacks {
        for row in crate_rows.iter().rev() {
            if let Some(crate_) = row.get(stack.id - 1).unwrap() {
                stack.push(crate_.clone());
            }
        }
    }

    let mut crane = Crane::new(stacks.clone());
    crane.apply_9k_moves(&moves);
    let is_ok = crane.peek_stacks() == "DHBJQJCCW";
    println!("{} ({})", crane.peek_stacks(), is_ok);

    let mut crane = Crane::new(stacks.clone());
    crane.apply_9k1_moves(&moves);
    let is_ok = crane.peek_stacks() == "WJVRLSJJT";
    println!("{} ({})", crane.peek_stacks(), is_ok);
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

impl Clone for Crate {
    fn clone(&self) -> Self {
        return Self {
            name: self.name.clone(),
        };
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

#[derive(Debug)]
struct CrateStack {
    id: usize,
    crates: Vec<Crate>,
}

impl Clone for CrateStack {
    fn clone(&self) -> Self {
        return Self {
            id: self.id,
            crates: self.crates.clone(),
        };
    }
}

impl CrateStack {
    fn new(id: usize) -> Self {
        return Self { id, crates: vec![] };
    }

    fn push(&mut self, crate_: Crate) {
        self.crates.push(crate_);
    }

    fn pop(&mut self) -> Option<Crate> {
        return self.crates.pop();
    }

    fn peek(&self) -> Option<&Crate> {
        return self.crates.last();
    }
}

#[derive(Debug)]
struct Crane {
    stacks: HashMap<usize, CrateStack>,
}

impl Crane {
    fn new(stacks: Vec<CrateStack>) -> Self {
        let stacks = stacks
            .into_iter()
            .map(|stack| (stack.id, stack))
            .collect::<HashMap<usize, CrateStack>>();
        return Self { stacks };
    }

    fn apply_9k_move(&mut self, move_: &Move) {
        let mut crates = vec![];

        {
            let from = self.stacks.get_mut(&move_.from).unwrap();
            for _ in 0..move_.amount {
                let Some(crt) = from.pop() else { continue; };
                crates.push(crt);
            }
        }

        {
            let to = self.stacks.get_mut(&move_.to).unwrap();
            for crate_ in crates.into_iter() {
                to.push(crate_);
            }
        }
    }

    fn apply_9k_moves(&mut self, moves: &Vec<Move>) {
        for move_ in moves {
            self.apply_9k_move(move_);
        }
    }

    fn apply_9k1_move(&mut self, move_: &Move) {
        let mut crates = vec![];

        {
            let from = self.stacks.get_mut(&move_.from).unwrap();
            for _ in 0..move_.amount {
                let Some(crt) = from.pop() else { continue; };
                crates.push(crt);
            }
        }

        {
            let to = self.stacks.get_mut(&move_.to).unwrap();
            for crate_ in crates.into_iter().rev() {
                to.push(crate_);
            }
        }
    }

    fn apply_9k1_moves(&mut self, moves: &Vec<Move>) {
        for move_ in moves {
            self.apply_9k1_move(move_);
        }
    }

    fn peek_stacks(&self) -> String {
        let mut sorted_stacks = self.stacks.values().collect::<Vec<&CrateStack>>();
        sorted_stacks.sort_by_key(|stack| stack.id);

        return sorted_stacks
            .iter()
            .filter_map(|stack| stack.peek())
            .map(|crate_| crate_.name.clone())
            .collect::<_>();
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
