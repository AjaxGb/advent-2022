#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IdRange {
    min: u32,
    max: u32,
}

impl IdRange {
    fn parse(text: &str) -> Self {
        let (min, max) = text.split_once('-').unwrap();
        let min = min.parse().unwrap();
        let max = max.parse().unwrap();
        assert!(
            min <= max,
            "minimum ({min}) was greater than maximum ({max})"
        );
        Self { min, max }
    }

    fn contains(self, other: IdRange) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(self, other: IdRange) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}

fn main() {
    let mut fully_contains_count = 0;
    let mut overlaps_count = 0;

    for line in include_str!("input.txt").lines() {
        let (a, b) = line.split_once(',').unwrap();
        let a = IdRange::parse(a);
        let b = IdRange::parse(b);
        if a.contains(b) || b.contains(a) {
            fully_contains_count += 1;
        }
        if a.overlaps(b) {
            overlaps_count += 1;
        }
    }

    println!("Pairs where one fully contains the other: {fully_contains_count}");
    println!("Pairs where one overlaps the other: {overlaps_count}");
}
