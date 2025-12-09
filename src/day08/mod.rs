use crate::day08::xzy::Xyz;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::HashSet;

mod xzy;

fn part1(input: &str, pairs_count: usize) -> usize {
    let boxes = input.lines().map(Xyz::from).collect::<Vec<_>>();

    let distances = boxes
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, a.euclidean_distance(b)))
        .sorted_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).unwrap())
        .collect::<Vec<_>>();

    let mut circuits: Vec<HashSet<Xyz>> = boxes.iter().map(|b| HashSet::from([*b])).collect();

    for distance in distances.iter().take(pairs_count) {
        let (a, b, _) = distance;

        let a_idx = circuits.iter().position(|circuit| circuit.contains(a)).unwrap();
        let b_idx = circuits.iter().position(|circuit| circuit.contains(b)).unwrap();

        if a_idx == b_idx {
            continue;
        }

        let b_circuit = circuits.remove(b_idx);
        let a_idx = if b_idx < a_idx { a_idx - 1 } else { a_idx };

        circuits[a_idx].extend(b_circuit);
    }

    circuits.sort_by_key(|circuit| Reverse(circuit.len()));
    circuits[0].len() * circuits[1].len() * circuits[2].len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolbox::daily::DailyInput;

    #[test]
    fn solves_part1_example() {
        assert_eq!(part1(EXAMPLE, 10), 40);
    }

    #[test]
    fn solves_part1() {
        let input = DailyInput::get(8);
        assert_eq!(part1(&input, 1000), 29406);
    }

    const EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
}
