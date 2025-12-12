use crate::day11::connections::Connections;

mod connections;

fn part1(input: &str) -> usize {
    let connections = Connections::from(input);
    connect(&connections, "you")
}

fn connect(connections: &Connections, current: &str) -> usize {
    if current == "out" {
        return 1;
    }
    connections
        .get(current)
        .map(|outputs| outputs.iter().map(|output| connect(connections, output)).sum())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolbox::daily::DailyInput;

    #[test]
    fn solves_part1_example() {
        assert_eq!(part1(EXAMPLE), 5)
    }

    #[test]
    fn solves_part1() {
        let input = DailyInput::get(11);
        assert_eq!(part1(&input), 662)
    }

    const EXAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
}
