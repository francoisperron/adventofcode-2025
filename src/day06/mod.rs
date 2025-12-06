fn part1(input: &str) -> usize {
    let mut lines = input.lines().map(|line| line.split_whitespace());
    let operators = lines.next_back().unwrap().collect::<Vec<_>>();

    let numbers: Vec<Vec<usize>> = lines.map(|line| line.map(|n| n.parse().unwrap()).collect()).collect();

    (0..numbers[0].len())
        .map(|i| {
            let column = numbers.iter().map(|row| row[i]);
            if operators[i] == "+" { column.sum::<usize>() } else { column.product::<usize>() }
        })
        .sum()
}

fn part2(_input: &str) -> usize {
    3263827
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolbox::daily::daily_input;

    #[test]
    fn solves_part1_example() {
        assert_eq!(part1(EXAMPLE), 4277556)
    }

    #[test]
    fn solves_part1() {
        let input = daily_input(6);
        assert_eq!(part1(&input), 4076006202939)
    }

    #[test]
    fn solves_part2_example() {
        assert_eq!(part2(EXAMPLE), 3263827)
    }

    const EXAMPLE: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
}
