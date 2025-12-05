mod ids_ranges;

use crate::day02::IdsRanges;

fn part1(input: &str) -> usize {
    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();
    let ranges = IdsRanges::from(ranges_str.replace("\n", ",").as_str());
    let ingredients = ingredients_str.lines().map(|line| line.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    ingredients.iter().filter(|&id| ranges.iter().any(|range| range.0.contains(id))).count()
}

fn part2(input: &str) -> usize {
    let (ranges_str, _ingredients_str) = input.split_once("\n\n").unwrap();
    let ranges = IdsRanges::from(ranges_str.replace("\n", ",").as_str());

    let combined = ranges.combine();
    combined.iter().map(|r| r.number_in_range()).sum()
}

#[cfg(test)]
mod tests {
    use crate::day05::{part1, part2};
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        assert_eq!(part1(EXAMPLE), 3)
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(5);
        assert_eq!(part1(&input), 874)
    }

    #[test]
    fn solves_part2_example() {
        assert_eq!(part2(EXAMPLE), 14)
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(5);
        assert_eq!(part2(&input), 348548952146313)
    }

    const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
}
