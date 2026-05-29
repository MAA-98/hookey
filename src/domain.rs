use std::ops::Range;

/// A typed integer for characters in the buffer.
///
/// This is **not** a byte index.
/// When used as an index, it is a position in character
/// space.
///
/// Note: this is not the same as a displayed terminal column,
/// and it is not the same as a grapheme cluster index.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufInt(usize);

impl BufInt {
    /// Create a new character index.
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    /// Get the raw `usize` value.
    pub fn raw(self) -> usize {
        self.0
    }
}

/// A half-open range of character positions.
///
/// Half-open means:
/// - `start` is included
/// - `end` is excluded
///
/// So `5..8` means 5, 6, and 7.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BufRange {
    pub start: BufInt,
    pub end: BufInt,
}

impl BufRange {
    /// Create a new typed character range.
    pub fn new(start: BufInt, end: BufInt) -> Self {
        Self { start, end }
    }

    /// Convert into a normal Rust range of `usize` values.
    pub fn into_std(self) -> Range<usize> {
        self.start.raw()..self.end.raw()
    }
}

/// A typed line index.
///
/// Line indices are usually zero-based:
/// - first line = 0
/// - second line = 1
/// - etc.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineIndex(usize);

impl LineIndex {
    /// Create a new line index.
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    /// Get the raw `usize` value back out.
    pub fn raw(self) -> usize {
        self.0
    }
}

/// A typed line count.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineCount(usize);

impl LineCount {
    /// Create a new line count.
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    /// Get the raw `usize` value back out.
    pub fn raw(self) -> usize {
        self.0
    }
}
