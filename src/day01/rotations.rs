use std::ops::Deref;

pub struct Rotations(Vec<Rotation>);

impl From<&str> for Rotations {
    fn from(input: &str) -> Self {
        let rotations = input.lines().map(Rotation::from).collect();
        Rotations(rotations)
    }
}

impl Deref for Rotations {
    type Target = Vec<Rotation>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rotation(isize);

impl Rotation {
    pub fn new(distance: isize) -> Self {
        Rotation(distance)
    }
}

impl From<&str> for Rotation {
    fn from(input: &str) -> Self {
        let sign = if input.starts_with('L') { -1 } else { 1 };
        let distance = input[1..].parse::<isize>().unwrap();
        Rotation(distance * sign)
    }
}

impl Deref for Rotation {
    type Target = isize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_from_string() {
        assert_eq!(Rotation::from("L1"), Rotation::new(-1));
        assert_eq!(Rotation::from("R22"), Rotation::new(22));
    }
}
