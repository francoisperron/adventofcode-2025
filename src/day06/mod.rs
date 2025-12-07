use itertools::Itertools;

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

fn part2(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let max_column_index = lines.iter().map(|line| line.len()).max().unwrap();

    let columns = (0..max_column_index)
        .rev()
        .map(|column| lines.iter().map(|line| line.chars().nth(column).unwrap_or(' ')).join(""))
        .filter(|c| !c.trim().is_empty())
        .collect::<Vec<_>>();

    let mut total = 0;
    let mut numbers = vec![];

    for column in columns.iter() {
        let number = column.replace(['+', '*'], "").trim().parse::<usize>().unwrap();
        numbers.push(number);

        let maybe_operator = column.chars().find(|&c| c == '+' || c == '*');
        if let Some(operator) = maybe_operator {
            total += if operator == '+' { numbers.iter().sum::<usize>() } else { numbers.iter().product::<usize>() };
            numbers.clear();
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolbox::daily::DailyInput;

    #[test]
    fn solves_part1_example() {
        assert_eq!(part1(EXAMPLE), 4277556)
    }

    #[test]
    fn solves_part1() {
        let input = DailyInput::get(6);
        assert_eq!(part1(&input), 4076006202939)
    }

    #[test]
    fn solves_part2_example() {
        assert_eq!(part2(EXAMPLE), 3263827)
    }

    #[test]
    fn solves_part2() {
        let input = DailyInput::get(6);
        assert_eq!(part2(&input), 7903168391557)
    }

    const EXAMPLE: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
}
