use dial::Dial;
use rotations::Rotations;

mod dial;
mod rotations;

fn part1(input: &str) -> usize {
    let rotations = Rotations::from(input);

    Dial::new(50).turn_all(&rotations).filter(|dial| dial.position == 0).count()
}

fn part2(input: &str) -> usize {
    let rotations = Rotations::from(input);

    Dial::new(50).turn_all(&rotations).map(|dial| dial.zero_position_count).sum()
}

#[cfg(test)]
mod tests {
    use crate::day01::{part1, part2};
    use crate::toolbox::daily::DailyInput;

    #[test]
    fn solves_part1_example() {
        assert_eq!(part1(EXAMPLE), 3)
    }

    #[test]
    fn solves_part1() {
        let input = DailyInput::get(1);
        assert_eq!(part1(&input), 1018)
    }

    #[test]
    fn solves_part2_example() {
        assert_eq!(part2(EXAMPLE), 6)
    }

    #[test]
    fn solves_part2() {
        let input = DailyInput::get(1);
        assert_eq!(part2(&input), 5815)
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
