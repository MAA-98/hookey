use crate::buffer::Buffer;
use crate::domain::{BufInt, LineIndex};

/// A cursor position inside the buffer.
///
/// This cursor stores a character index, not a visual screen position.
/// We store the cursor as an absolute character index because:
/// - Ropey primarily works in character offsets
/// - line numbers and in-line positions are O(logN) anyway
/// - command internals can optimize later if needed
///
/// Important:
/// The cursor may temporarily be out of bounds after relative movement.
/// In that case, call `clamp_to_buffer()` to bring it back into range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    /// Absolute character index in the buffer.
    pub char_index: BufInt,
}

impl Cursor {
    // ----- INITIALIZING -----

    /// Create a cursor at a specific position.
    pub fn new(char_index: BufInt) -> Self {
        Self { char_index }
    }

    // ------ QUERYING ------

    /// Return the current absolute character index.
    pub fn char_index(&self) -> BufInt {
        self.char_index
    }

    /// Get the line containing the cursor.
    pub fn line_index(&self, buffer: &Buffer) -> LineIndex {
        buffer
            .char_to_line(self.char_index)
            .expect("cursor should be within buffer when asking for line index")
    }

    /// Get the character offset at the start of the cursor's line.
    pub fn line_start(&self, buffer: &Buffer) -> BufInt {
        let line = self.line_index(buffer);

        buffer
            .line_to_char(line)
            .expect("line index should be valid when asking for line start")
    }

    // ----- MODIFYING -----

    /// Set the cursor to a specific character index.
    ///
    /// This does not clamp automatically.
    /// If you want to ensure the cursor is valid for a buffer, call
    /// `clamp_to_buffer()` afterward.
    pub fn set_char_index(&mut self, char_index: BufInt) {
        self.char_index = char_index;
    }

    /// Offset the cursor by a number of character positions.
    ///
    /// If `positive` is `true`, the cursor moves right.
    /// If `positive` is `false`, the cursor moves left.
    ///
    /// This does not clamp to the buffer length, so the cursor may end up
    /// past the end of the buffer. Use `clamp_to_buffer()` afterward if needed.
    pub fn offset_char_index(&mut self, offset: usize, positive: bool) {
        if positive {
            self.char_index = BufInt::new(self.char_index.raw() + offset);
        } else {
            self.char_index = BufInt::new(self.char_index.raw().saturating_sub(offset));
        }
    }

    /// Clamp the cursor so it stays inside the valid buffer range.
    ///
    /// This is useful after edits, or after relative movement that may have
    /// moved the cursor beyond the end of the buffer.
    pub fn clamp_to_buffer(&mut self, buffer: &Buffer) {
        let len = buffer.len_chars();

        if self.char_index > len {
            self.char_index = len;
        }
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new(BufInt::new(0))
    }
}