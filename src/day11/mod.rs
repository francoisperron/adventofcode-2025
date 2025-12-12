use crate::day11::cache::Cache;
use crate::day11::connections::Connections;

mod cache;
mod connections;

fn part1(input: &str) -> usize {
    let connections = Connections::from(input);
    connect(&connections, "you", true, true, &mut Cache::default())
}

fn part2(input: &str) -> usize {
    let connections = Connections::from(input);
    connect(&connections, "svr", false, false, &mut Cache::default())
}

fn connect(connections: &Connections, current: &str, dac: bool, fft: bool, cache: &mut Cache) -> usize {
    if current == "out" {
        return if dac && fft { 1 } else { 0 };
    }

    if let Some(&sum) = cache.get(&(current.to_string(), dac, fft)) {
        return sum;
    }

    let new_dac = dac || current == "dac";
    let new_fft = fft || current == "fft";

    let sum = connections
        .get(current)
        .map(|outputs| outputs.iter().map(|output| connect(connections, output, new_dac, new_fft, cache)).sum())
        .unwrap();

    cache.put((current.to_string(), new_dac, new_fft), sum);

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolbox::daily::DailyInput;

    #[test]
    fn solves_part1_example() {
        assert_eq!(part1(EXAMPLE_PART1), 5)
    }

    #[test]
    fn solves_part1() {
        let input = DailyInput::get(11);
        assert_eq!(part1(&input), 662)
    }

    #[test]
    fn solves_part2_example() {
        assert_eq!(part2(EXAMPLE_PART2), 2)
    }

    #[test]
    fn solves_part2() {
        let input = DailyInput::get(11);
        assert_eq!(part2(&input), 429399933071120)
    }

    const EXAMPLE_PART1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const EXAMPLE_PART2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
}
