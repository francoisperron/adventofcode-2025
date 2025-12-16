pub struct Joltages(pub(crate) Vec<isize>);

impl Joltages {
    pub fn new(joltages: &[isize]) -> Self {
        Self(joltages.to_vec())
    }

    pub fn to_binary(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .fold(0usize, |acc, (i, &j)| if (j & 1) != 0 { acc | (1 << i) } else { acc })
    }

    pub fn apply(&self, buttons: &[usize]) -> Joltages {
        let joltages = (0..self.0.len())
            .map(|i| {
                let mask = 1usize << i;
                let pressed = buttons.iter().filter(|&&button| (button & mask) != 0).count() as isize;
                (self.0[i] - pressed) / 2
            })
            .collect();
        Joltages(joltages)
    }

    pub fn all_set(&self) -> bool {
        self.0.iter().all(|&j| j == 0)
    }
}
