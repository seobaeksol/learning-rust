pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod ranges {

    use std::ops::Range;

    /// Return true if two ranges overlap.
    ///
    ///     assert_eq!(doc_tests::ranges::overlap(0..7, 3..10), true);
    ///     assert_eq!(doc_tests::ranges::overlap(1..5, 101..105), false);
    ///
    /// If either range is empty, they don't count as overlapping.
    ///
    ///     assert_eq!(doc_tests::ranges::overlap(0..0, 0..10), false);
    pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
        r1.start < r1.end && r2.start < r2.end && r1.start < r2.end && r2.start < r1.end
    }
}
