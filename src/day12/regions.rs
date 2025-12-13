use crate::toolbox::Grid;

pub struct Regions {}

impl Regions {
    pub fn from(input: &str) -> Vec<Region> {
        input
            .lines()
            .map(|s| {
                let (size, shapes_count) = s.split_once(": ").unwrap();

                let (max_x, max_y) = size.split_once("x").unwrap();
                let max_x = max_x.parse::<isize>().unwrap();
                let max_y = max_y.parse::<isize>().unwrap();

                let shapes = shapes_count.split_whitespace().map(|c| c.parse::<usize>().unwrap()).collect();

                Region { grid: Grid::new(max_x, max_y), shapes }
            })
            .collect()
    }
}

pub struct Region {
    pub grid: Grid,
    pub shapes: Vec<usize>,
}

impl Region {
    pub fn fit(&self, _shapes: &[Grid]) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_region() {
        let input = "12x5: 1 0 1 0 2 2";
        let regions = Regions::from(input);

        assert_eq!(regions[0].grid.max_x, 12);
        assert_eq!(regions[0].grid.max_y, 5);
        assert_eq!(regions[0].shapes, vec![1, 0, 1, 0, 2, 2]);
    }
}
