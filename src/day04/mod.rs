use crate::toolbox::Grid;

fn part1(input: &str) -> usize {
    let grid = Grid::from(input);
    grid.elements
        .iter()
        .filter(|&(&p, &v)| v == '@' && p.around_me().filter(|&p2| grid.element_at(&p2).eq(&'@')).count() < 4)
        .count()
}


#[cfg(test)]
mod tests {
    use crate::day04::part1;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        assert_eq!(part1(EXAMPLE), 13)
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(4);
        assert_eq!(part1(&input), 1457)
    }

    const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
}