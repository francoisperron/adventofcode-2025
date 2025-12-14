use crate::day12::regions::Regions;
use crate::day12::shapes::Shapes;

mod regions;
mod shapes;

fn part1(input: &str) -> usize {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let [begin @ .., last] = parts.as_slice() else { unreachable!() };

    let shapes = Shapes::from(begin);
    let regions = Regions::from(last);

    regions.iter().filter(|region| region.fit(&shapes)).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolbox::daily::DailyInput;

    #[test]
    fn solves_part1_example() {
        assert_eq!(part1(EXAMPLE), 3)
    }

    #[test]
    fn solves_part1() {
        let input = DailyInput::get(12);
        assert_eq!(part1(&input), 497)
    }

    const EXAMPLE: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
}
