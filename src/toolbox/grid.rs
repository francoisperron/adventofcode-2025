use crate::toolbox::Direction;
use crate::toolbox::position::Position;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct Grid {
    pub elements: HashMap<Position, char>,
    pub max_x: isize,
    pub max_y: isize,
}

impl Grid {
    pub fn new(max_x: isize, max_y: isize) -> Grid {
        Grid { elements: HashMap::new(), max_x, max_y }
    }

    pub fn from(input: &str) -> Grid {
        let elements = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, element)| (Position::new(x as isize, y as isize), element)))
            .collect();
        let max_y = input.lines().count() as isize;
        let max_x = input.lines().next().unwrap().chars().count() as isize;
        Grid { elements, max_x, max_y }
    }

    pub fn element_at(&self, position: &Position) -> &char {
        self.elements.get(position).unwrap_or(&'.')
    }

    pub fn set_element_at(&mut self, position: &Position, element: char) {
        self.elements.insert(*position, element);
    }

    pub fn position_of(&self, element: char) -> Option<Position> {
        self.elements.iter().find(|&(_, &v)| v == element).map(|(position, _)| *position)
    }

    pub fn positions_of(&self, element: char) -> impl Iterator<Item = Position> + '_ {
        self.elements.iter().filter(move |&(_, &v)| v == element).map(|(position, _)| *position)
    }

    pub fn is_inbound(&self, position: &Position) -> bool {
        position.x >= 0 && position.x < self.max_x && position.y >= 0 && position.y < self.max_y
    }

    pub fn group_by_value(&self) -> HashMap<char, HashSet<Position>> {
        self.elements.iter().fold(HashMap::new(), |mut grouping, (&position, &value)| {
            grouping.entry(value).or_default().insert(position);
            grouping
        })
    }

    pub fn swap_elements(&mut self, position1: &Position, position2: &Position) {
        let element1 = *self.element_at(position1);
        let element2 = *self.element_at(position2);
        self.set_element_at(position1, element2);
        self.set_element_at(position2, element1);
    }

    pub fn print(&self) -> String {
        (0..self.max_y)
            .map(|y| {
                (0..self.max_x)
                    .map(|x| self.element_at(&Position::new(x, y)).to_string())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn rotate(&self) -> Grid {
        let mut rotated = Grid::new(self.max_y, self.max_x);

        for (&positions, &value) in self.elements.iter() {
            let x = self.max_y - 1 - positions.y;
            let y = positions.x;
            rotated.set_element_at(&Position::new(x, y), value);
        }

        rotated
    }

    pub fn flip(&self, direction: Direction) -> Grid {
        let mut flipped = Grid::new(self.max_x, self.max_y);

        for (&positions, &value) in self.elements.iter() {
            let x = if direction == Direction::Up || direction == Direction::Down { positions.x } else { self.max_x - 1 - positions.x };
            let y = if direction == Direction::Left || direction == Direction::Right { positions.y } else { self.max_y - 1 - positions.y };
            flipped.set_element_at(&Position::new(x, y), value);
        }

        flipped
    }
}

#[cfg(test)]
mod tests {
    use crate::toolbox::Direction;
    use crate::toolbox::grid::Grid;
    use crate::toolbox::position::Position;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn creates_grid() {
        let grid = Grid::from("ab\ncd");

        assert_eq!(grid.elements, HashMap::from([(Position::new(0, 0), 'a'), (Position::new(1, 0), 'b'), (Position::new(0, 1), 'c'), (Position::new(1, 1), 'd')]));
    }

    #[test]
    fn gets_element_at_position() {
        let grid = Grid::from("ab\ncd");

        assert_eq!(grid.element_at(&Position::new(0, 0)), &'a');
        assert_eq!(grid.element_at(&Position::new(1, 0)), &'b');
        assert_eq!(grid.element_at(&Position::new(0, 1)), &'c');
        assert_eq!(grid.element_at(&Position::new(1, 1)), &'d');
    }

    #[test]
    fn gets_empty_when_element_not_found() {
        let grid = Grid::from("ab\ncd");

        assert_eq!(grid.element_at(&Position::new(2, 2)), &'.');
    }

    #[test]
    fn determines_if_position_is_inbound() {
        let grid = Grid::from("ab\ncd");

        assert!(grid.is_inbound(&Position::new(0, 0)));
        assert!(grid.is_inbound(&Position::new(1, 0)));
        assert!(grid.is_inbound(&Position::new(0, 1)));
        assert!(grid.is_inbound(&Position::new(1, 1)));

        assert!(!grid.is_inbound(&Position::new(-1, 0)));
        assert!(!grid.is_inbound(&Position::new(0, -1)));
        assert!(!grid.is_inbound(&Position::new(2, 0)));
        assert!(!grid.is_inbound(&Position::new(0, 2)));
    }

    #[test]
    fn groups_by_value() {
        let grid = Grid::from("ab\nad");
        let grouping = grid.group_by_value();

        assert_eq!(
            grouping,
            HashMap::from([('a', HashSet::from([Position::new(0, 0), Position::new(0, 1)])), ('b', HashSet::from([Position::new(1, 0)])), ('d', HashSet::from([Position::new(1, 1)]))])
        );
    }

    #[test]
    fn swaps_elements() {
        let mut grid = Grid::from("ab");
        grid.swap_elements(&Position::new(0, 0), &Position::new(1, 0));

        assert_eq!(grid.element_at(&Position::new(0, 0)), &'b');
        assert_eq!(grid.element_at(&Position::new(1, 0)), &'a');
    }

    #[test]
    fn rotates_clockwise() {
        let grid = Grid::from("1234\n5678");
        let rotated = grid.rotate();

        assert_eq!(rotated, Grid::from("51\n62\n73\n84"));
    }

    #[test]
    fn flips_up_down() {
        let grid = Grid::from("12\n34\n56");

        let flipped_up = grid.flip(Direction::Up);
        assert_eq!(flipped_up, Grid::from("56\n34\n12"));

        let flipped_down = grid.flip(Direction::Down);
        assert_eq!(flipped_down, Grid::from("56\n34\n12"));
    }

    #[test]
    fn flips_right_or_left() {
        let grid = Grid::from("123\n456");

        let flipped_up = grid.flip(Direction::Left);
        assert_eq!(flipped_up, Grid::from("321\n654"));

        let flipped_down = grid.flip(Direction::Right);
        assert_eq!(flipped_down, Grid::from("321\n654"));
    }
}
