use crate::day01::rotations::Rotation;

#[derive(Clone, Copy)]
pub struct Dial {
    pub position: isize,
    pub zero_position_count: usize,
}

impl Dial {
    pub fn new(position: isize) -> Self {
        Dial { position, zero_position_count: 0 }
    }

    pub fn turn_all<'a>(&self, rotations: &'a [Rotation]) -> impl Iterator<Item = Dial> + 'a {
        rotations.iter().scan(*self, |dial, rotation| {
            *dial = dial.turn(rotation);
            Some(*dial)
        })
    }

    pub fn turn(&self, rotation: &Rotation) -> Dial {
        let new_position = self.position + **rotation;
        let zero_position_count = (new_position <= 0 && self.position != 0) as usize + (new_position.signum() * new_position / 100) as usize;

        Dial { position: new_position.rem_euclid(100), zero_position_count }
    }
}

#[cfg(test)]
mod tests {
    use crate::day01::dial::Dial;
    use crate::day01::rotations::Rotation;

    #[test]
    fn turns() {
        let mut dial = Dial::new(50);

        dial = dial.turn(&Rotation::new(-68));
        assert_eq!(dial.position, 82);

        dial = dial.turn(&Rotation::new(-30));
        assert_eq!(dial.position, 52);
    }

    #[test]
    fn counts_passing_by_zero_turning_left() {
        let dial = Dial::new(50);
        let new_dial = dial.turn(&Rotation::new(-68));

        assert_eq!(new_dial.zero_position_count, 1);
    }

    #[test]
    fn count_passing_by_zero_turning_right() {
        let dial = Dial::new(14);
        let new_dial = dial.turn(&Rotation::new(-82));

        assert_eq!(new_dial.zero_position_count, 1);
    }

    #[test]
    fn count_passing_by_zero_turning_multiple_times() {
        let dial = Dial::new(50);
        let new_dial = dial.turn(&Rotation::new(1000));

        assert_eq!(new_dial.zero_position_count, 10);
    }
}
