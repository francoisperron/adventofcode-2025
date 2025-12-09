#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Xyz {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Xyz {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn euclidean_distance(&self, other: &Xyz) -> f64 {
        let sum = (self.x - other.x).unsigned_abs().pow(2) + (self.y - other.y).unsigned_abs().pow(2) + (self.z - other.z).unsigned_abs().pow(2);
        (sum as f64).sqrt()
    }
}

impl From<&str> for Xyz {
    fn from(s: &str) -> Self {
        let mut coordinates = s.split(',').map(|s| s.parse().unwrap());
        Self::new(coordinates.next().unwrap(), coordinates.next().unwrap(), coordinates.next().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_euclidean_distance() {
        let a = Xyz::new(162, 817, 812);
        let b = Xyz::new(425, 690, 689);

        assert_eq!(a.euclidean_distance(&b), 100427f64.sqrt());
    }

    #[test]
    fn calculates_euclidean_distance2() {
        let a = Xyz::new(162, 817, 812);
        let b = Xyz::new(431, 825, 988);

        assert_eq!(a.euclidean_distance(&b), 103401f64.sqrt());
    }
}
