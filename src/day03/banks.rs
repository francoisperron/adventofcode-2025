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
    pub fn max(&self) -> usize {
        let largest = *self.batteries[..self.batteries.len() - 1].iter().max().unwrap();
        let index = self.batteries.iter().position(|&b| b == largest).unwrap();
        let second_largest = *self.batteries[index + 1..].iter().max().unwrap();

        (largest * 10 + second_largest) as usize
    }
}
