use crate::day09::edge::Edge;
use crate::toolbox::Position;
use itertools::Itertools;

pub struct Surface {
    pub corner_a: Position,
    pub corner_b: Position,
    pub area: usize,
}

impl Surface {
    pub fn new(corner_a: Position, corner_b: Position, area: usize) -> Surface {
        Surface { corner_a, corner_b, area }
    }

    pub fn from(corners: &[Position]) -> Vec<Surface> {
        corners
            .iter()
            .tuple_combinations()
            .map(|(a, b)| Surface::new(*a, *b, a.area(b)))
            .sorted_by(|a, b| b.area.cmp(&a.area))
            .collect::<Vec<_>>()
    }

    pub fn intersected_by(&self, edges: &[Edge]) -> bool {
        let (min_x, max_x, min_y, max_y) = Self::bounds(self.corner_a, self.corner_b);

        edges.iter().any(|edge| {
            let (e_min_x, e_max_x, e_min_y, e_max_y) = Self::bounds(edge.corner_a, edge.corner_b);
            min_x < e_max_x && max_x > e_min_x && min_y < e_max_y && max_y > e_min_y
        })
    }

    fn bounds(p1: Position, p2: Position) -> (isize, isize, isize, isize) {
        let min_x = p1.x.min(p2.x);
        let max_x = p1.x.max(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_y = p1.y.max(p2.y);
        (min_x, max_x, min_y, max_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_bounds_of_two_positions() {
        let p1 = Position::new(1, 10);
        let p2 = Position::new(20, 2);
        let (min_x, max_x, min_y, max_y) = Surface::bounds(p1, p2);

        assert_eq!((min_x, max_x, min_y, max_y), (1, 20, 2, 10));
    }

    #[test]
    fn intersects_when_edge_crosses_surface() {
        let surface = Surface::new(Position::new(0, 0), Position::new(2, 2), 1);
        let edge = Edge::new(Position::new(0, 0), Position::new(1, 1));

        assert!(surface.intersected_by(&[edge]));
    }

    #[test]
    fn does_not_intersect_when_edge_touches_surface() {
        let surface = Surface::new(Position::new(1, 1), Position::new(2, 2), 1);
        let edge = Edge::new(Position::new(0, 0), Position::new(1, 1));

        assert!(!surface.intersected_by(&[edge]));
    }
}
