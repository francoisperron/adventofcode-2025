use crate::day02::{IdsRange, IdsRanges};

impl IdsRanges {
    pub fn combine(&self) -> IdsRanges {
        let mut ranges = self.0.clone();
        ranges.sort_by_key(|r| *r.0.start());

        let mut combined = Vec::new();
        let mut current = ranges[0].clone();

        for range in ranges.iter().skip(1) {
            if current.overlap_or_adjacent(range) {
                current = current.merge(range);
            } else {
                combined.push(current);
                current = range.clone();
            }
        }
        combined.push(current);

        IdsRanges(combined)
    }
}

impl IdsRange {
    pub fn overlap_or_adjacent(&self, r2: &IdsRange) -> bool {
        self.0.start() <= &r2.0.end().saturating_add(1) && r2.0.start() <= &self.0.end().saturating_add(1)
    }

    pub fn merge(&self, r2: &IdsRange) -> IdsRange {
        IdsRange(*self.0.start().min(r2.0.start())..=*self.0.end().max(r2.0.end()))
    }

    pub fn number_in_range(&self) -> usize {
        self.0.clone().count()
    }
}

#[cfg(test)]
mod tests {
    use crate::day02::IdsRanges;

    #[test]
    fn combines_ranges() {
        let ranges = IdsRanges::from("3-5,10-14,16-20,12-18");

        assert_eq!(ranges.combine(), IdsRanges::from("3-5,10-20"));
    }

    #[test]
    fn combines_adjacent() {
        let ranges = IdsRanges::from("1-2,3-4");

        assert_eq!(ranges.combine(), IdsRanges::from("1-4"));
    }
}
