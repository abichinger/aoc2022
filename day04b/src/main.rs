struct Range {
    start: i32,
    end: i32,
}

impl Range {
    pub fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        self.contains(&Range::new(other.start, other.start)) ||
        self.contains(&Range::new(other.end, other.end))
    }

    pub fn new(start: i32, end: i32) -> Range {
        Range {
            start: start,
            end: end
        }
    }

    pub fn from_str(s: &str) -> Range {
        let (start, end) = s.split_once("-").unwrap();
        return Range::new(start.parse().unwrap(), end.parse().unwrap());
    }
}

fn main() {
    let lines = include_str!("../input.txt").lines();
    let res = lines
        .map(|l| l.split_once(",").unwrap())
        .map(|(a, b)| (Range::from_str(a), Range::from_str(b)))
        .filter(|(a, b)| a.overlaps(b) || b.overlaps(a))
        .count();

    print!("{}", res);
}
