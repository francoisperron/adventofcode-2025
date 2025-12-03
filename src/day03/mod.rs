use crate::day03::banks::Banks;

mod banks;

fn part1(input: &str) -> usize {
    let banks = Banks::from(input);
    banks.iter().map(|bank| bank.max(2)).sum()
}

fn part2(input: &str) -> usize {
    let banks = Banks::from(input);
    banks.iter().map(|bank| bank.max(12)).sum()
}

#[cfg(test)]
mod tests {
    use crate::day03::{part1, part2};
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        assert_eq!(part1(EXAMPLE), 357);
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(3);
        assert_eq!(part1(&input), 17087);
    }

    #[test]
    fn solves_part2_example() {
        assert_eq!(part2(EXAMPLE), 3121910778619);
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(3);
        assert_eq!(part2(&input), 169019504359949);
    }

    const EXAMPLE: &str = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
}
