use crate::day03::banks::Banks;

mod banks;

fn part1(input: &str) -> usize {
    let banks = Banks::from(input);
    banks.iter().map(|bank| bank.max()).sum()
}

#[cfg(test)]
mod tests {
    use crate::day03::part1;
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

    const EXAMPLE: &str = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
}
