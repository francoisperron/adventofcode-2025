use crate::day10::machine::Machine;

mod buttons;
mod joltages;
mod machine;

fn part1(input: &str) -> usize {
    let machines = input.lines().map(Machine::from).collect::<Vec<_>>();
    machines.iter().map(|m| m.configure_lights()).sum()
}

fn part2(input: &str) -> usize {
    let machines = input.lines().map(Machine::from).collect::<Vec<_>>();
    machines.iter().map(|m| m.configure_joltage()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolbox::daily::DailyInput;

    #[test]
    fn solves_part1_example() {
        assert_eq!(part1(EXAMPLE), 7)
    }

    #[test]
    fn solves_part1() {
        let input = DailyInput::get(10);
        assert_eq!(part1(&input), 491);
    }

    #[test]
    fn solves_part2_example() {
        assert_eq!(part2(EXAMPLE), 33)
    }

    #[test]
    fn solves_part2() {
        let input = DailyInput::get(10);
        assert_eq!(part2(&input), 20617);
    }

    const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
}
