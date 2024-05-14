mod astral_body;

use std::collections::HashMap;

use self::astral_body::AstralBody;
use self::astral_body::AstralBody::{Galaxy, Void};

#[derive(Debug)]
pub struct Universe {
    astral_bodies: HashMap<(usize, usize), AstralBody>,
}

impl Universe {
    pub fn new(input: &str) -> Self {
        let mut astral_bodies = HashMap::new();
        let lines = input.lines();
        for (row, line) in lines.enumerate() {
            for (col, c) in line.char_indices() {
                match c {
                    '.' => astral_bodies.insert((col, row), Void),
                    '#' => astral_bodies.insert((col, row), Galaxy),
                    _ => panic!("Invalid character in input"),
                };
            }
        }

        Universe { astral_bodies }
    }

    pub fn expand(&mut self, expansion_rate: usize) {
        let expansion_rate = match expansion_rate {
            0 | 1 => 2,
            rate => rate,
        };
        let (cols, rows) = self.size();
        let (empty_rows, empty_cols) = (self.get_empty_rows(), self.get_empty_cols());

        let mut astral_bodies = HashMap::new();

        for col in 0..cols {
            let col_offset =
                empty_cols.iter().filter(|&&c| c <= col).count() * (expansion_rate - 1);
            for row in 0..rows {
                let row_offset =
                    empty_rows.iter().filter(|&&r| r <= row).count() * (expansion_rate - 1);
                let new_coords = (col + col_offset, row + row_offset);
                match self.get_astral_body((col, row)) {
                    Some(Galaxy) => astral_bodies.insert(new_coords, Galaxy),
                    Some(Void) => astral_bodies.insert(new_coords, Void),
                    None => panic!("No astral body found at {:?}", (col, row)),
                };
            }
        }

        let (expanded_cols, expanded_rows) = (cols + empty_cols.len(), rows + empty_rows.len());
        for (col, row) in (0..expanded_cols).zip(0..expanded_rows) {
            astral_bodies.entry((col, row)).or_insert(Void);
        }

        self.astral_bodies = astral_bodies;
    }

    pub fn get_distances_between_galaxies(&self) -> Vec<usize> {
        let galaxies = self.get_galaxy_coordinates();
        let mut distances = vec![];
        for a in 0..galaxies.len() {
            for b in (a + 1)..galaxies.len() {
                let x_steps = num::abs(galaxies[a].0 as isize - galaxies[b].0 as isize);
                let y_steps = num::abs(galaxies[a].1 as isize - galaxies[b].1 as isize);
                let distance = (x_steps + y_steps) as usize;
                distances.push(distance);
            }
        }

        distances
    }

    fn get_galaxy_coordinates(&self) -> Vec<(usize, usize)> {
        self.astral_bodies
            .iter()
            .filter_map(|a| if *a.1 == Galaxy { Some(*a.0) } else { None })
            .collect()
    }

    fn get_empty_rows(&self) -> Vec<usize> {
        let mut empty_rows = vec![];
        let (width, height) = self.size();

        'outer: for col in 0..height {
            for row in 0..width {
                if self.get_astral_body((row, col)) == Some(&AstralBody::Galaxy) {
                    continue 'outer;
                }
            }
            empty_rows.push(col);
        }

        empty_rows
    }

    fn get_empty_cols(&self) -> Vec<usize> {
        let mut empty_cols = vec![];
        let (width, height) = self.size();

        'outer: for col in 0..width {
            for row in 0..height {
                if self.get_astral_body((col, row)) == Some(&AstralBody::Galaxy) {
                    continue 'outer;
                }
            }
            empty_cols.push(col);
        }

        empty_cols
    }

    fn get_astral_body(&self, coords: (usize, usize)) -> Option<&AstralBody> {
        self.astral_bodies.get(&coords)
    }

    fn size(&self) -> (usize, usize) {
        let mut max_x = 0;
        let mut max_y = 0;
        for (x, y) in self.astral_bodies.keys() {
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
        }

        (max_x + 1, max_y + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = include_str!("../test-input-1.txt");

    #[test]
    fn test_parse_input() {
        let universe = Universe::new(TEST_INPUT_1);
        assert_eq!(universe.get_astral_body((0, 0)), Some(&Void));
        assert_eq!(universe.get_astral_body((3, 0)), Some(&Galaxy));
        assert_eq!(universe.get_astral_body((0, 2)), Some(&Galaxy));
        assert_eq!(universe.size(), (10, 10))
    }

    #[test]
    fn test_find_empty_rows() {
        let universe = Universe::new(TEST_INPUT_1);
        assert_eq!(universe.get_empty_rows(), vec![3, 7]);
    }

    #[test]
    fn test_find_empty_cols() {
        let universe = Universe::new(TEST_INPUT_1);
        assert_eq!(universe.get_empty_cols(), vec![2, 5, 8]);
    }

    #[test]
    fn test_expand() {
        let mut universe = Universe::new(TEST_INPUT_1);
        universe.expand(2);
        assert_eq!(universe.size(), (13, 12));
        assert_eq!(universe.get_astral_body((0, 0)), Some(&Void));
        assert_eq!(universe.get_astral_body((3, 0)), Some(&Void));
        assert_eq!(universe.get_astral_body((4, 0)), Some(&Galaxy));
        assert_eq!(universe.get_astral_body((0, 2)), Some(&Galaxy));
        assert_eq!(universe.get_astral_body((0, 9)), Some(&Void));
        assert_eq!(universe.get_astral_body((0, 11)), Some(&Galaxy));

        let mut universe = Universe::new(TEST_INPUT_1);
        universe.expand(10);
        assert_eq!(universe.size(), (37, 28));
        assert_eq!(universe.get_astral_body((0, 0)), Some(&Void));
        assert_eq!(universe.get_astral_body((12, 0)), Some(&Galaxy));
    }

    #[test]
    fn test_get_distances() {
        let mut universe = Universe::new(TEST_INPUT_1);
        universe.expand(2);
        assert_eq!(
            universe
                .get_distances_between_galaxies()
                .iter()
                .sum::<usize>(),
            374
        );

        let mut universe = Universe::new(TEST_INPUT_1);
        universe.expand(10);
        assert_eq!(
            universe
                .get_distances_between_galaxies()
                .iter()
                .sum::<usize>(),
            1030
        );

        let mut universe = Universe::new(TEST_INPUT_1);
        universe.expand(100);
        assert_eq!(
            universe
                .get_distances_between_galaxies()
                .iter()
                .sum::<usize>(),
            8410
        );
    }
}
