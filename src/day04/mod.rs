use crate::toolbox::Grid;

fn part1(input: &str) -> usize {
    let grid = Grid::from(input);
    grid.elements
        .iter()
        .filter(|&(&p, &v)| v == '@' && p.around_me().filter(|&p2| grid.element_at(&p2).eq(&'@')).count() < 4)
        .count()
}

fn part2(input: &str) -> usize {
    let mut grid = Grid::from(input);
    let mut rolls_count = 0;

    loop {
        let rolls: Vec<_> = grid
            .elements
            .iter()
            .filter(|&(&p, &v)| v == '@' && p.around_me().filter(|&p2| grid.element_at(&p2).eq(&'@')).count() < 4)
            .map(|(&p, _)| p)
            .collect();

        if rolls.is_empty() {
            break;
        }

        rolls.iter().for_each(|&p| grid.set_element_at(&p, '.'));
        rolls_count += rolls.len();
    }

    rolls_count
}

#[cfg(test)]
mod tests {
    use crate::day04::{part1, part2};
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

    #[test]
    fn solves_part2_example() {
        assert_eq!(part2(EXAMPLE), 43)
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(4);
        assert_eq!(part2(&input), 8310)
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
