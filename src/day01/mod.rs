mod rotations;
mod dial;

#[cfg(test)]
mod tests {
    use crate::day01::dial::Dial;
    use crate::day01::rotations::Rotations;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        let rotations = Rotations::from(EXAMPLE);
        assert_eq!(rotations.rotations, vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]);

        let dial = Dial::new(rotations);
        assert_eq!(dial.password(), 3)
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(1);
        let rotations = Rotations::from(&input);
        let dial = Dial::new(rotations);

        assert_eq!(dial.password(), 1018)
    }

    const EXAMPLE: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
}