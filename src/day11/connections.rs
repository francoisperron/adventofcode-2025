use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug)]
pub struct Connections(HashMap<String, Vec<String>>);

impl Connections {
    pub fn from(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    let mut parts = line.split(": ");
                    let from = parts.next().unwrap().to_string();
                    let to = parts.next().unwrap().split_whitespace().map(|s| s.to_string()).collect();
                    (from, to)
                })
                .collect(),
        )
    }
}

impl Deref for Connections {
    type Target = HashMap<String, Vec<String>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_input() {
        let input = "aaa: you hhh";
        let connections = Connections::from(input);

        assert_eq!(connections.get("aaa").unwrap(), &vec!["you".to_string(), "hhh".to_string()]);
    }
}
