
pub struct Rotations {
    pub rotations: Vec<isize>,
}

impl Rotations {
    pub fn from(input: &str) -> Self {
        let rotations = input.lines().map(|line| {
            let number = line[1..].parse::<isize>().unwrap();
            if line.starts_with('L') { -number } else { number }
        }).collect();

        Rotations { rotations }
    }
}