use crate::toolbox::Position;

pub struct Corners {}

impl Corners {
    pub fn from(input: &str) -> Vec<Position> {
        input.lines().map(Position::from).collect()
    }
}
