use crate::toolbox::{Direction, Grid};
use std::collections::HashMap;

fn solve(input: &str) -> (usize, usize) {
    let grid = Grid::from(input);

    let initial_beam = grid.position_of('S').unwrap();
    let mut timelines = HashMap::from([(initial_beam, 1)]);
    let mut split_count = 0;

    for line in 1..grid.max_y {
        for (beam, times) in timelines.clone().iter().filter(|(p, _)| p.y == line - 1) {
            let down = beam.move_towards(&Direction::Down);

            if grid.element_at(&down).eq(&'^') {
                split_count += 1;

                let left = down.move_towards(&Direction::Left);
                timelines.insert(left, timelines.get(&left).unwrap_or(&0) + times);

                let right = down.move_towards(&Direction::Right);
                timelines.insert(right, timelines.get(&right).unwrap_or(&0) + times);
            } else {
                timelines.insert(down, timelines.get(&down).unwrap_or(&0) + times);
            }
        }
    }

    let timelines_count = timelines.iter().filter(|(p, _)| p.y == grid.max_y - 1).map(|(_, v)| v).sum();

    (split_count, timelines_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        assert_eq!(solve(EXAMPLE).0, 21)
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(7);
        assert_eq!(solve(&input).0, 1602)
    }

    #[test]
    fn solves_part2_example() {
        assert_eq!(solve(EXAMPLE).1, 40)
    }

    #[test]
    fn solves_part2() {
        let input = daily_input(7);
        assert_eq!(solve(&input).1, 135656430050438)
    }

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
}
