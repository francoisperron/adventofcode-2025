use crate::toolbox::Position;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Edge {
    pub corner_a: Position,
    pub corner_b: Position,
}

impl Edge {
    pub fn new(corner_a: Position, corner_b: Position) -> Edge {
        Edge { corner_a, corner_b }
    }

    pub fn from(corners: &[Position]) -> Vec<Edge> {
        corners.iter().circular_tuple_windows().map(|(a, b)| Edge::new(*a, *b)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_edges_from_corners() {
        let corners = vec![Position::new(0, 0), Position::new(0, 1), Position::new(1, 0)];
        let edges = Edge::from(&corners);

        assert_eq!(edges, vec![Edge::new(corners[0], corners[1]), Edge::new(corners[1], corners[2]), Edge::new(corners[2], corners[0])]);
    }
}
