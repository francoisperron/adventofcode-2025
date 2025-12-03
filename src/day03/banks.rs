use std::ops::Deref;

pub struct Banks(Vec<Bank>);

impl From<&str> for Banks {
    fn from(input: &str) -> Self {
        Self(input.lines().map(Bank::from).collect())
    }
}

impl Deref for Banks {
    type Target = Vec<Bank>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Bank {
    pub batteries: Vec<u32>,
}

impl From<&str> for Bank {
    fn from(input: &str) -> Self {
        Self { batteries: input.chars().map(|s| s.to_digit(10).unwrap()).collect::<Vec<u32>>() }
    }
}

impl Bank {
    pub fn max(&self, batteries_count: usize) -> usize {
        let mut nb = batteries_count;
        let mut batteries = self.batteries.clone();
        let mut joltage: usize = 0;

        while nb > 0 {
            let largest = *batteries[..batteries.len() - nb + 1].iter().max().unwrap();
            let largest_index = batteries.iter().position(|&b| b == largest).unwrap();

            batteries = batteries[largest_index + 1..].to_vec();

            joltage = (joltage * 10) + largest as usize;
            nb -= 1;
        }

        joltage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_2() {
        let bank = Bank::from("818181911112111");
        assert_eq!(bank.max(2), 92);
    }

    #[test]
    fn max_3() {
        let bank = Bank::from("818181911112111");
        assert_eq!(bank.max(3), 921);
    }

    #[test]
    fn max_12() {
        let bank = Bank::from("818181911112111");
        assert_eq!(bank.max(12), 888911112111);
    }
}
