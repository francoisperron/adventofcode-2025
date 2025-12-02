use std::ops::{Deref, RangeInclusive};

pub struct IdsRanges(Vec<IdsRange>);

impl From<&str> for IdsRanges {
    fn from(input: &str) -> Self {
        let ranges = input.split(',').map(IdsRange::from).collect();
        IdsRanges(ranges)
    }
}

impl Deref for IdsRanges {
    type Target = Vec<IdsRange>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IdsRanges {
    pub fn sum_of_invalids(&self) -> usize {
        self.iter().map(|range| range.invalid_ids().sum::<usize>()).sum::<usize>()
    }
}

pub struct IdsRange(RangeInclusive<usize>);

impl From<&str> for IdsRange {
    fn from(input: &str) -> Self {
        let range = input.split('-').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        IdsRange(range[0]..=range[1])
    }
}

impl IdsRange {
    pub fn invalid_ids(&self) -> impl Iterator<Item = usize> + '_ {
        self.0.clone().filter(|&id| {
            let id_string = id.to_string();
            let digit_count = id_string.len();

            if digit_count % 2 == 1 {
                return false;
            }

            let mid = digit_count / 2;
            let (start, end) = id_string.split_at(mid);
            start == end
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_input() {
        let ids = IdsRanges::from("11-22,95-115,998-1012");

        assert_eq!(ids.len(), 3);
        assert_eq!(ids[0].0, RangeInclusive::new(11, 22));
    }

    #[test]
    fn finds_invalid_ids_in_simple_range() {
        let range = IdsRange::from("11-22");

        assert_eq!(range.invalid_ids().collect::<Vec<usize>>(), vec![11, 22]);
    }

    #[test]
    fn finds_invalid_ids_in_big_range() {
        let range = IdsRange::from("1188511880-1188511890");

        assert_eq!(range.invalid_ids().collect::<Vec<usize>>(), vec![1188511885]);
    }
}
