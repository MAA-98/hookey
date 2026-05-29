use std::fmt;
use ropey::Rope;
use crate::domain::{BufInt, BufRange, LineCount, LineIndex};

/// A thin wrapper around Ropey’s `Rope`.
/// https://docs.rs/ropey/latest/ropey/struct.Rope.html
///
/// This is the low-level text storage type, with typed return values and errors.
/// Important note:
/// Ropey works with *character indices* for many operations.
/// That means this buffer is not using byte offsets like `String` does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Buffer {
    rope: Rope,
}

/// Errors that can happen while editing the buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferError {
    /// A buffer character index was outside the valid range.
    InvalidBufferIndex {
        index: BufInt,
        len: BufInt,
    },

    /// A line index was outside the valid range.
    InvalidLineIndex {
        index: LineIndex,
        len: LineCount,
    },

    /// The range itself was malformed.
    ///
    /// Example:
    /// - `start > end`
    /// - `10..3`
    MalformedCharRange {
        range: BufRange,
    },

    /// The range was well-formed, but not fully inside the buffer.
    ///
    /// Example:
    /// - `0..9999` when the buffer is shorter than that
    OutOfBoundsCharRange {
        range: BufRange,
        len: BufInt,
    },
}

impl fmt::Display for BufferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BufferError::InvalidBufferIndex { index, len } => {
                write!(
                    f,
                    "invalid buffer index: {} (buffer length: {})",
                    index.raw(),
                    len.raw()
                )
            }
            BufferError::InvalidLineIndex { index, len } => {
                write!(
                    f,
                    "invalid line index: {} (line count: {})",
                    index.raw(),
                    len.raw()
                )
            }
            BufferError::MalformedCharRange { range } => {
                write!(
                    f,
                    "malformed char range: {}..{} (start must be <= end)",
                    range.start.raw(),
                    range.end.raw()
                )
            }
            BufferError::OutOfBoundsCharRange { range, len } => {
                write!(
                    f,
                    "out of bounds char range: {}..{} (buffer length: {})",
                    range.start.raw(),
                    range.end.raw(),
                    len.raw()
                )
            }
        }
    }
}

impl std::error::Error for BufferError {}

impl Buffer {
    // ----- INITIALIZING -----

    /// Creates an empty buffer.
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
        }
    }

    /// Create a buffer from an existing string slice.
    pub fn from_str(s: &str) -> Self {
        Self {
            rope: Rope::from_str(s),
        }
    }

    // ------ QUERYING ------

    /// Return the number of characters in the buffer.
    pub fn len_chars(&self) -> BufInt {
        BufInt::new(self.rope.len_chars())
    }

    /// Get the character at a given character index.
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn char_at(&self, char_index: BufInt) -> Option<char> {
        if char_index >= self.len_chars() {
            return None;
        }

        Some(self.rope.char(char_index.raw()))
    }

    /// Return the number of lines in the buffer.
    pub fn len_lines(&self) -> LineCount {
        LineCount::new(self.rope.len_lines())
    }

    /// Convert a character index to the line containing it.
    pub fn char_to_line(&self, char_index: BufInt) -> Result<LineIndex, BufferError> {
        let len = self.len_chars();

        if char_index.raw() > len.raw() {
            return Err(BufferError::InvalidBufferIndex {
                index: char_index,
                len,
            });
        }

        Ok(LineIndex::new(self.rope.char_to_line(char_index.raw())))
    }

    /// Convert a line index to the character index at the start of that line.
    pub fn line_to_char(&self, line_index: LineIndex) -> Result<BufInt, BufferError> {
        let len = self.len_lines();

        if line_index.raw() > len.raw() {
            return Err(BufferError::InvalidLineIndex {
                index: line_index,
                len,
            });
        }

        Ok(BufInt::new(self.rope.line_to_char(line_index.raw())))
    }

    // ----- MODIFYING -----
    // Exposing of `ropey::Rope` API with custom errors, nothing extra.

    /// Insert text at a character index.
    pub fn insert_str(&mut self, char_index: BufInt, text: &str) -> Result<(), BufferError> {
        let len = self.len_chars();

        if char_index.raw() > len.raw() {
            return Err(BufferError::InvalidBufferIndex {
                index: char_index,
                len,
            });
        }

        self.rope.insert(char_index.raw(), text);
        Ok(())
    }

    /// Append text to the end of the buffer.
    pub fn append_str(&mut self, text: &str) {
        self.rope.append(Rope::from(text));
    }

    /// Remove a range of characters from the buffer.
    ///
    /// The range is character-based and half-open:
    /// - `start` is included
    /// - `end` is excluded
    pub fn delete_chars(&mut self, range: BufRange) -> Result<(), BufferError> {
        let length = self.len_chars();

        // A range is malformed if the start comes after the end.
        if range.start > range.end {
            return Err(BufferError::MalformedCharRange { range });
        }

        // For a half-open range, `end` may be equal to `len`,
        // but it may not be greater than `len`.
        if range.end > length {
            return Err(BufferError::OutOfBoundsCharRange { range, len: length });
        }

        self.rope.remove(range.into_std());
        Ok(())
    }

    /// Convert the whole buffer into a `String`.
    ///
    /// Only use for tests, since it's inefficient.
    pub fn as_string(&self) -> String {
        self.rope.to_string()
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
