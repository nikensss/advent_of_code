use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::day_08::instruction::Instruction;

#[derive(Debug)]
pub struct Node {
    id: String,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(id: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            id: id.to_string(),
            left: None,
            right: None,
        }))
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn set_left(&mut self, child: Rc<RefCell<Node>>) {
        if child.borrow().get_id() == self.id {
            return;
        }

        self.left = Some(child);
    }

    pub fn set_right(&mut self, child: Rc<RefCell<Node>>) {
        if child.borrow().get_id() == self.id {
            return;
        }

        self.right = Some(child);
    }

    pub fn get_left(&self) -> Option<Rc<RefCell<Node>>> {
        self.left.clone()
    }

    pub fn get_right(&self) -> Option<Rc<RefCell<Node>>> {
        self.right.clone()
    }

    pub fn take(&self, instruction: &Instruction) -> Option<Rc<RefCell<Node>>> {
        match instruction {
            Instruction::Left => self.get_left(),
            Instruction::Right => self.get_right(),
        }
    }

    pub fn is_start(&self) -> bool {
        self.id == "AAA"
    }

    pub fn is_end(&self) -> bool {
        self.id == "ZZZ"
    }

    pub fn is_start_for_ghost(&self) -> bool {
        self.id.ends_with("A")
    }

    pub fn is_end_for_ghost(&self) -> bool {
        self.id.ends_with("Z")
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let left = self.get_left();
        let mut left_id = "None".to_string();
        if left.is_some() {
            left_id = left.unwrap().borrow().id.clone();
        }

        let right = self.get_right();
        let mut right_id = "None".to_string();
        if right.is_some() {
            right_id = right.unwrap().borrow().id.clone();
        }

        write!(f, "{} -> ({:?}, {:?})", self.id, left_id, right_id)
    }
}
