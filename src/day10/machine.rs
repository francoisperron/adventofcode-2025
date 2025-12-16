use crate::day10::buttons::Buttons;
use crate::day10::joltages::Joltages;
use itertools::Itertools;

#[derive(Debug)]
pub struct Machine {
    pub lights: usize,
    pub buttons: Vec<usize>,
    pub joltages: Vec<isize>,
}

impl From<&str> for Machine {
    fn from(input: &str) -> Self {
        let parts: Vec<_> = input.split_whitespace().collect();
        let [first, middle @ .., last] = parts.as_slice() else { unreachable!() };

        let lights = first
            .trim_matches(['[', ']'])
            .chars()
            .enumerate()
            .fold(0, |acc, (i, c)| if c == '#' { acc | (1 << i) } else { acc });

        let buttons = middle
            .iter()
            .map(|b| {
                b.trim_matches(['(', ')'])
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .fold(0, |acc, n| acc | (1 << n))
            })
            .collect::<Vec<_>>();

        let joltages = last.trim_matches(['{', '}']).split(',').map(|n| n.parse::<isize>().unwrap()).collect::<Vec<_>>();

        Self { lights, buttons, joltages }
    }
}

impl Machine {
    pub fn configure_lights(&self) -> usize {
        self.buttons
            .iter()
            .powerset()
            .filter_map(|buttons_combination| {
                let lights_result = buttons_combination.iter().fold(0, |acc, &button| acc ^ button);
                if lights_result == self.lights { Some(buttons_combination.len()) } else { None }
            })
            .min()
            .unwrap()
    }

    pub fn configure_joltage(&self) -> usize {
        let buttons = Buttons::from(&self.buttons);
        let joltages = Joltages::new(&self.joltages);
        buttons.fewest_presses(&joltages).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_lights_pattern_to_int() {
        assert_eq!(Machine::from("[####] (1) {3,5,4,7}").lights, 15);
        assert_eq!(Machine::from("[....] (1) {3,5,4,7}").lights, 0);
    }

    #[test]
    fn parses_buttons_to_int() {
        assert_eq!(Machine::from("[....] (0) (0,1,2,3) {3,5,4,7}").buttons, vec![1, 15]);
    }

    #[test]
    fn parses_joltage() {
        assert_eq!(Machine::from("[....] (0) (0,1,2,3) {3,5,4,7}").joltages, vec![3, 5, 4, 7]);
    }

    #[test]
    fn finds_fewest_buttons_pressed() {
        assert_eq!(Machine::from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}").configure_lights(), 2);
        assert_eq!(Machine::from("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}").configure_lights(), 3);
        assert_eq!(Machine::from("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}").configure_lights(), 2);
    }
}
