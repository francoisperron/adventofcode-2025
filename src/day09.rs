use crate::toolbox::Position;
use itertools::Itertools;

fn part1(input: &str) -> usize {
    let elements = input.lines().map(Position::from).collect::<Vec<_>>();

    let areas = elements
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.area(b))
        .sorted_by(|a, b| b.partial_cmp(a).unwrap())
        .collect::<Vec<_>>();

    areas[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolbox::daily::DailyInput;

    #[test]
    fn solves_part1_example() {
        assert_eq!(part1(EXAMPLE), 50);
    }

    #[test]
    fn solves_part1() {
        let input = DailyInput::get(9);
        assert_eq!(part1(&input), 4744899849);
    }

    const EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
}
