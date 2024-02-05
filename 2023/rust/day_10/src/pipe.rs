use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{direction::Direction, pipe_type::PipeType};

#[derive(Debug)]
pub struct Pipe {
    pipe_type: PipeType,

    x: usize,
    y: usize,

    north: Option<Weak<RefCell<Pipe>>>,
    east: Option<Weak<RefCell<Pipe>>>,
    south: Option<Weak<RefCell<Pipe>>>,
    west: Option<Weak<RefCell<Pipe>>>,
}

impl PartialEq for Pipe {
    fn eq(&self, other: &Self) -> bool {
        self.pipe_type == other.pipe_type
            && self.x == other.x
            && self.y == other.y
            && self.pipe_type == other.pipe_type
    }
}

impl Eq for Pipe {}

impl Pipe {
    pub fn new(pipe_type: PipeType, x: usize, y: usize) -> Rc<RefCell<Pipe>> {
        Rc::new(RefCell::new(Pipe {
            pipe_type,
            x,
            y,
            north: None,
            east: None,
            south: None,
            west: None,
        }))
    }

    pub fn is_start(&self) -> bool {
        self.pipe_type == PipeType::Start
    }

    fn is_ground(&self) -> bool {
        self.pipe_type == PipeType::Ground
    }

    pub fn set_pipe(&mut self, direction: &Direction, pipe: Rc<RefCell<Pipe>>) {
        match direction {
            Direction::North => self.north = Some(Rc::downgrade(&pipe)),
            Direction::East => self.east = Some(Rc::downgrade(&pipe)),
            Direction::South => self.south = Some(Rc::downgrade(&pipe)),
            Direction::West => self.west = Some(Rc::downgrade(&pipe)),
        }
    }

    pub fn traverse_from(&self, direction: &Direction) -> Option<(Direction, Weak<RefCell<Pipe>>)> {
        match (direction, &self.pipe_type) {
            (Direction::North, PipeType::NE) => Some((Direction::West, self.east.clone().unwrap())),
            (Direction::North, PipeType::NS) => {
                Some((Direction::North, self.south.clone().unwrap()))
            }
            (Direction::North, PipeType::NW) => Some((Direction::East, self.west.clone().unwrap())),

            (Direction::South, PipeType::NS) => {
                Some((Direction::South, self.north.clone().unwrap()))
            }
            (Direction::South, PipeType::SW) => Some((Direction::East, self.west.clone().unwrap())),
            (Direction::South, PipeType::SE) => Some((Direction::West, self.east.clone().unwrap())),

            (Direction::East, PipeType::NE) => {
                Some((Direction::South, self.north.clone().unwrap()))
            }
            (Direction::East, PipeType::SE) => {
                Some((Direction::North, self.south.clone().unwrap()))
            }
            (Direction::East, PipeType::EW) => Some((Direction::East, self.west.clone().unwrap())),

            (Direction::West, PipeType::NW) => {
                Some((Direction::South, self.north.clone().unwrap()))
            }
            (Direction::West, PipeType::EW) => Some((Direction::West, self.east.clone().unwrap())),
            (Direction::West, PipeType::SW) => {
                Some((Direction::North, self.south.clone().unwrap()))
            }

            _ => None,
        }
    }

    pub fn goes(&self, direction: &Direction) -> bool {
        if self.is_ground() {
            return false;
        }

        if !self.is_start() {
            return self.pipe_type.is_connected_to(direction);
        }

        match direction {
            Direction::North => {
                let Some(north) = &self.north else {
                    return false;
                };

                let Some(north) = north.upgrade() else {
                    return false;
                };

                let north = (*north).borrow();

                return north.goes(&Direction::South);
            }
            Direction::East => {
                let Some(east) = &self.east else {
                    return false;
                };

                let Some(east) = east.upgrade() else {
                    return false;
                };

                let east = (*east).borrow();
                return east.goes(&Direction::West);
            }
            Direction::South => {
                let Some(south) = &self.south else {
                    return false;
                };

                let Some(south) = south.upgrade() else {
                    return false;
                };

                let south = (*south).borrow();
                return south.goes(&Direction::North);
            }
            Direction::West => {
                let Some(west) = &self.west else {
                    return false;
                };

                let Some(west) = west.upgrade() else {
                    return false;
                };

                let west = (*west).borrow();
                return west.goes(&Direction::East);
            }
        }
    }

    pub fn get_connected_pipes(&self) -> Vec<(Direction, Weak<RefCell<Pipe>>)> {
        let mut connected_pipes = vec![];

        if self.north.is_some() && self.goes(&Direction::North) {
            connected_pipes.push((Direction::South, self.north.clone().unwrap()));
        }

        if self.east.is_some() && self.goes(&Direction::East) {
            connected_pipes.push((Direction::West, self.east.clone().unwrap()));
        }

        if self.south.is_some() && self.goes(&Direction::South) {
            connected_pipes.push((Direction::North, self.south.clone().unwrap()));
        }

        if self.west.is_some() && self.goes(&Direction::West) {
            connected_pipes.push((Direction::East, self.west.clone().unwrap()));
        }

        connected_pipes
    }

    pub fn is_at(&self, (x, y): (usize, usize)) -> bool {
        (self.x, self.y) == (x, y)
    }

    pub fn get_coordinates(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}
