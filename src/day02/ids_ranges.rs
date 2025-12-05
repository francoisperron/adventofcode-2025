use std::ops::{Deref, DerefMut, RangeInclusive};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdsRanges(pub(crate) Vec<IdsRange>);

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

impl DerefMut for IdsRanges {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdsRange(pub(crate) RangeInclusive<usize>);

impl From<&str> for IdsRange {
    fn from(input: &str) -> Self {
        let range = input.split('-').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        IdsRange(range[0]..=range[1])
    }
}

impl IdsRange {
    pub fn invalid_ids(&self) -> impl Iterator<Item = usize> {
        self.0.clone().filter(|&id| {
            let count = Self::count_digit(id);

            if count % 2 == 1 {
                return false;
            }

            let parts = Self::split_into_chunks(id, count / 2);
            parts.iter().all(|&part| part == parts[0])
        })
    }

    pub fn invalid_ids_sequence(&self) -> impl Iterator<Item = usize> {
        self.0.clone().filter(|&id| {
            Self::divisors(id).any(|chunk_size| {
                let parts = Self::split_into_chunks(id, chunk_size);
                parts.iter().all(|&part| part == parts[0])
            })
        })
    }

    fn divisors(id: usize) -> impl Iterator<Item = u32> {
        let count = Self::count_digit(id);
        (1..count).filter(move |&d| count.is_multiple_of(d))
    }

    fn count_digit(id: usize) -> u32 {
        id.checked_ilog10().unwrap_or(0) + 1
    }

    fn split_into_chunks(mut num: usize, chunk_size: u32) -> Vec<usize> {
        let divisor = 10_usize.pow(chunk_size);
        let mut chunks = Vec::new();

        while num > 0 {
            chunks.push(num % divisor);
            num /= divisor;
        }

        chunks
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

    #[test]
    fn finds_invalid_ids_sequence_simple_range() {
        let range = IdsRange::from("95-115");

        assert_eq!(range.invalid_ids_sequence().collect::<Vec<usize>>(), vec![99, 111]);
    }

    #[test]
    fn finds_invalid_ids_sequence_big_range() {
        let range = IdsRange::from("2121212118-2121212124");

        assert_eq!(range.invalid_ids_sequence().collect::<Vec<usize>>(), vec![2121212121]);
    }

    #[test]
    fn finds_divisors_of_number_of_digit_in_id() {
        let divisors = IdsRange::divisors(12345678);

        assert_eq!(divisors.collect::<Vec<u32>>(), vec![1, 2, 4]);
    }
}
