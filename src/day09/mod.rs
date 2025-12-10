mod corner;
mod edge;
mod surface;

use crate::day09::corner::Corners;
use crate::day09::edge::Edge;
use crate::day09::surface::Surface;

fn part1(input: &str) -> usize {
    let corners = Corners::from(input);
    let surfaces = Surface::from(&corners);

    surfaces[0].area
}

fn part2(input: &str) -> usize {
    let corners = Corners::from(input);
    let surfaces = Surface::from(&corners);
    let edges = Edge::from(&corners);

    surfaces.iter().find(|s| !s.intersected_by(&edges)).map(|s| s.area).unwrap()
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

    #[test]
    fn solves_part2_example() {
        assert_eq!(part2(EXAMPLE), 24);
    }

    #[test]
    fn solves_part2() {
        let input = DailyInput::get(9);
        assert_eq!(part2(&input), 1540192500);
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
