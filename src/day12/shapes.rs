use crate::toolbox::Grid;

pub struct Shapes {}

impl Shapes {
    pub fn from(input: &[&str]) -> Vec<Grid> {
        input
            .iter()
            .map(|s| {
                let shape = s.lines().skip(1).collect::<Vec<_>>().join("\n");
                Grid::from(&shape)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_shape() {
        let input = "1:\n###\n##.\n.##";
        let shapes = Shapes::from(&[input]);

        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].print(), "###\n##.\n.##");
    }
}
