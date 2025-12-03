use crate::day02::ids_ranges::IdsRanges;

mod ids_ranges;

fn part1(input: &str) -> usize {
    let ranges = IdsRanges::from(input);
    ranges.iter().map(|range| range.invalid_ids().sum::<usize>()).sum::<usize>()
}

fn part2(input: &str) -> usize {
    let ranges = IdsRanges::from(input);
    ranges.iter().map(|range| range.invalid_ids_sequence().sum::<usize>()).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use crate::day02::{part1, part2};
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        assert_eq!(part1(EXAMPLE), 1227775554)
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(2);
        assert_eq!(part1(&input), 28844599675)
    }

    #[test]
    fn solves_part2_example() {
        assert_eq!(part2(EXAMPLE), 4174379265)
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(2);
        assert_eq!(part2(&input), 48778605167)
    }

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
}
