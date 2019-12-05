use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    start: CodePoint,
    end: CodePoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodePoint {
    row: u32,
    col: u32,
}

impl CodePoint {
    pub fn new_unchecked(row: u32, col: u32) -> Self {
        Self { row, col }
    }

    pub fn span(self, end: Self) -> Span {
        let start = self;

        assert!(start < end, "end must come after start");

        Span { start, end }
    }

    pub fn row(self) -> u32 {
        self.row
    }

    pub fn col(self) -> u32 {
        self.col
    }
}

impl Span {
    pub fn merge(self, other: Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    pub fn start(&self) -> CodePoint {
        self.start
    }

    pub fn end(&self) -> CodePoint {
        self.end
    }
}

impl PartialOrd for CodePoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CodePoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.row.cmp(&other.row).then(self.col.cmp(&other.col))
    }
}

#[test]
fn span_merge() {
    let start = CodePoint::new_unchecked(1, 3);
    let end = CodePoint::new_unchecked(2, 3);

    let first = start.span(end);

    let start = CodePoint::new_unchecked(2, 0);
    let end = CodePoint::new_unchecked(5, 10);

    let second = start.span(end);

    let combine = first.merge(second);

    assert_eq!(
        combine,
        Span {
            start: CodePoint { row: 1, col: 3 },
            end: CodePoint { row: 5, col: 10 }
        }
    )
}
