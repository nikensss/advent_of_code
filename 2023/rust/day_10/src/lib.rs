use std::{cell::RefCell, rc::Rc};

use direction::Direction;
use pipe::Pipe;

mod direction;
mod pipe;
mod pipe_type;

pub fn part_1(input: &str) -> usize {
    let pipes = connect_pipes(parse_input(input));
    let start = pipes.iter().find(|pipe| pipe.borrow().is_start()).unwrap();

    find_steps_to_farthest_pipe(start)
}

fn find_steps_to_farthest_pipe(start: &Rc<RefCell<Pipe>>) -> usize {
    let mut steps = 1;
    let connected_pipes = start.borrow().get_connected_pipes();
    let mut path_a = (connected_pipes[0].0, connected_pipes[0].1.clone());
    let mut path_b = (connected_pipes[1].0, connected_pipes[1].1.clone());

    loop {
        if path_a
            .1
            .upgrade()
            .unwrap()
            .borrow()
            .is_at(path_b.1.upgrade().unwrap().borrow().get_coordinates())
        {
            break;
        }

        let next_path_a = path_a
            .1
            .upgrade()
            .unwrap()
            .borrow()
            .traverse_from(&path_a.0);
        let next_path_b = path_b
            .1
            .upgrade()
            .unwrap()
            .borrow()
            .traverse_from(&path_b.0);

        path_a = (next_path_a.clone().unwrap().0, next_path_a.unwrap().1);
        path_b = (next_path_b.clone().unwrap().0, next_path_b.unwrap().1);

        steps += 1;
    }

    steps
}

fn parse_input(input: &str) -> Vec<Rc<RefCell<Pipe>>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let pipe_type = c.to_string().parse().unwrap();
                Pipe::new(pipe_type, x, y)
            })
        })
        .collect()
}

fn connect_pipes(pipes: Vec<Rc<RefCell<Pipe>>>) -> Vec<Rc<RefCell<Pipe>>> {
    pipes.iter().for_each(|pipe| {
        let (x, y) = pipe.borrow().get_coordinates();

        if y > 0 {
            let north_coords = (x, y - 1);
            if let Some(north) = pipes
                .iter()
                .find(|pipe| pipe.borrow().is_at(north_coords))
                .cloned()
            {
                pipe.borrow_mut().set_pipe(&Direction::North, north);
            }
        }

        let east_coords = (x + 1, y);
        if let Some(east) = pipes
            .iter()
            .find(|pipe| pipe.borrow().is_at(east_coords))
            .cloned()
        {
            pipe.borrow_mut().set_pipe(&Direction::East, east);
        }

        let south_coords = (x, y + 1);
        if let Some(south) = pipes
            .iter()
            .find(|pipe| pipe.borrow().is_at(south_coords))
            .cloned()
        {
            pipe.borrow_mut().set_pipe(&Direction::South, south);
        }

        if x > 0 {
            let west_coords = (x - 1, y);
            if let Some(west) = pipes
                .iter()
                .find(|pipe| pipe.borrow().is_at(west_coords))
                .cloned()
            {
                pipe.borrow_mut().set_pipe(&Direction::West, west);
            }
        }
    });

    pipes
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = include_str!("../test-input-1.txt");
    const TEST_INPUT_2: &str = include_str!("../test-input-2.txt");
    const COMPLETE_INPUT: &str = include_str!("../complete-input.txt");

    #[test]
    fn test_parse_input() {
        let expected = vec![
            Pipe::new(".".parse().unwrap(), 0, 0),
            Pipe::new(".".parse().unwrap(), 1, 0),
            Pipe::new(".".parse().unwrap(), 2, 0),
            Pipe::new(".".parse().unwrap(), 3, 0),
            Pipe::new(".".parse().unwrap(), 4, 0),
            Pipe::new(".".parse().unwrap(), 0, 1),
            Pipe::new("S".parse().unwrap(), 1, 1),
            Pipe::new("-".parse().unwrap(), 2, 1),
            Pipe::new("7".parse().unwrap(), 3, 1),
            Pipe::new(".".parse().unwrap(), 4, 1),
            Pipe::new(".".parse().unwrap(), 0, 2),
            Pipe::new("|".parse().unwrap(), 1, 2),
            Pipe::new(".".parse().unwrap(), 2, 2),
            Pipe::new("|".parse().unwrap(), 3, 2),
            Pipe::new(".".parse().unwrap(), 4, 2),
            Pipe::new(".".parse().unwrap(), 0, 3),
            Pipe::new("L".parse().unwrap(), 1, 3),
            Pipe::new("-".parse().unwrap(), 2, 3),
            Pipe::new("J".parse().unwrap(), 3, 3),
            Pipe::new(".".parse().unwrap(), 4, 3),
            Pipe::new(".".parse().unwrap(), 0, 4),
            Pipe::new(".".parse().unwrap(), 1, 4),
            Pipe::new(".".parse().unwrap(), 2, 4),
            Pipe::new(".".parse().unwrap(), 3, 4),
            Pipe::new(".".parse().unwrap(), 4, 4),
        ];
        assert_eq!(parse_input(TEST_INPUT_1), expected);
    }

    #[test]
    fn test_connect_pipes() {
        let pipes = parse_input(TEST_INPUT_1);
        let pipes = connect_pipes(pipes);

        let start = pipes.iter().find(|pipe| pipe.borrow().is_start()).unwrap();
        let start = start.borrow();

        assert!(start.goes(&Direction::East));
        assert!(start.goes(&Direction::South));

        assert_eq!(start.goes(&Direction::North), false);
        assert_eq!(start.goes(&Direction::West), false);
    }

    #[test]
    fn test_follow_connected_pipes() {
        let pipes = parse_input(TEST_INPUT_1);
        let pipes = connect_pipes(pipes);

        let start = pipes.iter().find(|pipe| pipe.borrow().is_start()).unwrap();
        let start = start.borrow();

        let connected_pipes = start.get_connected_pipes();
        assert_eq!(connected_pipes.len(), 2);

        let (path_a, path_b) = (&connected_pipes[0], &connected_pipes[1]);
        assert!(path_a.1.upgrade().unwrap().borrow().is_at((2, 1)));
        assert!(path_b.1.upgrade().unwrap().borrow().is_at((1, 2)));

        let (path_a, path_b) = (
            path_a
                .1
                .upgrade()
                .unwrap()
                .borrow()
                .traverse_from(&path_a.0),
            path_b
                .1
                .upgrade()
                .unwrap()
                .borrow()
                .traverse_from(&path_b.0),
        );

        assert!(path_a.unwrap().1.upgrade().unwrap().borrow().is_at((3, 1)));
        assert!(path_b.unwrap().1.upgrade().unwrap().borrow().is_at((1, 3)));
    }

    #[test]
    fn test_find_steps_to_farthest_pipe() {
        let pipes = connect_pipes(parse_input(TEST_INPUT_1));
        let start = pipes.iter().find(|pipe| pipe.borrow().is_start()).unwrap();
        assert_eq!(find_steps_to_farthest_pipe(start), 4);

        let pipes = connect_pipes(parse_input(TEST_INPUT_2));
        let start = pipes.iter().find(|pipe| pipe.borrow().is_start()).unwrap();
        assert_eq!(find_steps_to_farthest_pipe(start), 8);
    }

    #[test]
    fn test_part_1_with_test_input() {
        assert_eq!(part_1(TEST_INPUT_1), 4);
        assert_eq!(part_1(TEST_INPUT_2), 8);
    }

    #[test]
    fn test_part_1_with_complete_input() {
        assert_eq!(part_1(COMPLETE_INPUT), 6927);
    }
}
