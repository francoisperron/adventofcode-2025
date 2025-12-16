use crate::day10::joltages::Joltages;
use itertools::Itertools;

pub struct Buttons(Vec<(Vec<usize>, usize)>);

impl Buttons {
    pub fn from(buttons: &[usize]) -> Self {
        let xor = buttons
            .iter()
            .copied()
            .powerset()
            .map(|buttons_combination| {
                let xor = buttons_combination.iter().fold(0, |acc, &button| acc ^ button);
                (buttons_combination, xor)
            })
            .collect();

        Self(xor)
    }

    // from https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
    // find all possible sets of buttons you can push so that the remaining voltages are even, and divide by 2 and recurse.
    pub fn fewest_presses(&self, joltages: &Joltages) -> Option<usize> {
        if joltages.all_set() {
            return Some(0);
        }

        let target = joltages.to_binary();

        self.0
            .iter()
            .filter(|(_, xor)| *xor == target)
            .filter_map(|(buttons, _)| {
                let new_joltages = joltages.apply(buttons);
                if new_joltages.0.iter().all(|&j| j >= 0) { self.fewest_presses(&new_joltages).map(|c| buttons.len() + 2 * c) } else { None }
            })
            .min()
    }
}
