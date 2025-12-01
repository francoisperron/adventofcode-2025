use crate::day01::rotations::Rotations;

pub struct Dial{
    pub rotations: Rotations,
}

impl Dial {
    pub fn new(rotations: Rotations) -> Self {
        Dial { rotations }
    }

    pub fn password(&self) -> usize {
        self.rotations.rotations
            .iter()
            .scan(50, |position, &rotation| {
                *position = (*position + rotation) % 100;
                Some(*position)
            })
            .filter(|&pos| pos == 0)
            .count()
    }
}