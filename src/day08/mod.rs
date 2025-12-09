use crate::day08::xzy::Xyz;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::HashSet;

mod xzy;

fn solve(input: &str, pairs_count: usize) -> (usize, usize) {
    let boxes = input.lines().map(Xyz::from).collect::<Vec<_>>();

    let distances = boxes
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, a.euclidean_distance(b)))
        .sorted_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).unwrap())
        .collect::<Vec<_>>();

    let mut circuits: Vec<HashSet<Xyz>> = boxes.iter().map(|b| HashSet::from([*b])).collect();
    let mut all_connected = 0;

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

        if circuits.len() == 1 {
            all_connected = (a.x * b.x) as usize;
            break;
        }
    }

    circuits.sort_by_key(|circuit| Reverse(circuit.len()));
    let three_circuits = if circuits.len() > 1 { circuits[0].len() * circuits[1].len() * circuits[2].len() } else { 0 };

    (three_circuits, all_connected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolbox::daily::DailyInput;

    #[test]
    fn solves_part1_example() {
        assert_eq!(solve(EXAMPLE, 10).0, 40);
    }

    #[test]
    fn solves_part1() {
        let input = DailyInput::get(8);
        assert_eq!(solve(&input, 1000).0, 29406);
    }

    #[test]
    fn solves_part2_example() {
        assert_eq!(solve(EXAMPLE, usize::MAX).1, 25272);
    }

    #[test]
    fn solves_part2() {
        let input = DailyInput::get(8);
        assert_eq!(solve(&input, usize::MAX).1, 7499461416);
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
