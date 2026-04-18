use std::{cmp::Ordering, fmt};

/// A single point in source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Location {
    /// 1-based line number
    pub line: usize,
    /// 1-based column number
    pub column: usize,
    /// 0-based byte offset from start of source
    pub offset: usize,
}

impl Location {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        self.offset.cmp(&other.offset)
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A span covering a range in source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Span {
    /// Start location of the span (inclusive)
    pub start: Location,
    /// End location of the span (exclusive)
    pub end: Location,
}

impl Span {
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }

    /// Create a zero-width span at a single location (for insertion points).
    pub fn at(location: Location) -> Self {
        Self { start: location, end: location }
    }

    /// Merge two spans into one that covers both.
    pub fn merge(&self, other: &Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start.line == self.end.line {
            write!(f, "{}:{}-{}", self.start.line, self.start.column, self.end.column)
        } else {
            write!(
                f,
                "{}:{}-{}:{}",
                self.start.line, self.start.column, self.end.line, self.end.column
            )
        }
    }
}

impl Ord for Span {
    fn cmp(&self, other: &Self) -> Ordering {
        let first = self.start.offset.cmp(&other.start.offset);
        if first == Ordering::Equal {
            self.end.offset.cmp(&other.end.offset)
        } else {
            first
        }
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
