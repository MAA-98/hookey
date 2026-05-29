use crate::domain::{BufInt, BufRange};
use crate::editor::{BufferOp, CursorOp, Operation};

/// High-level editor intents.
///
/// This is the layer where you describe *what you want to happen*,
/// without directly manipulating the buffer or cursor.
///
/// Think of Actions as:
/// - user-facing intents
/// - editor commands at the semantic level
/// - things that can be turned into one or more primitive operations
///
/// Important:
/// `into_operations()` is intentionally pure.
/// It does not inspect the current buffer or cursor state.
/// It simply maps an Action into the operations it represents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Insert text at a specific character index.
    ///
    /// This is a direct intent:
    /// "put this text at this exact spot".
    InsertText {
        char_index: BufInt,
        text: String,
    },

    /// Append text to the end of the buffer.
    ///
    /// This is a useful editor intent because many actions naturally
    /// become appends.
    AppendText {
        text: String,
    },

    /// Delete a range of characters.
    ///
    /// The range is half-open:
    /// - start is included
    /// - end is excluded
    DeleteRange {
        range: BufRange,
    },

    /// Replace a range of characters with new text.
    ///
    /// This is intentionally represented as a high-level intent,
    /// even though it expands into lower-level operations.
    ReplaceRange {
        range: BufRange,
        text: String,
    },

    /// Move the cursor one character left.
    ///
    /// This does not inspect the current cursor position.
    /// It simply describes the intent as a relative movement.
    MoveCursorLeft,

    /// Move the cursor one character right.
    MoveCursorRight,

    /// Set the cursor to an exact character index.
    SetCursor {
        char_index: BufInt,
    },
}

impl Action {
    /// Convert this high-level action into primitive editor operations.
    ///
    /// This function is deliberately pure:
    /// - no buffer input
    /// - no cursor input
    /// - no state lookup
    ///
    /// If an action cannot be expressed using the current `Operation`
    /// types alone, it should not be modeled here yet.
    pub(crate) fn into_operations(self) -> Vec<Operation> {
        match self {
            Action::InsertText { char_index, text } => {
                vec![Operation::Buffer(BufferOp::InsertStr { char_index, text })]
            }

            Action::AppendText { text } => {
                vec![Operation::Buffer(BufferOp::AppendStr { text })]
            }

            Action::DeleteRange { range } => {
                vec![Operation::Buffer(BufferOp::DeleteChars { range })]
            }

            Action::ReplaceRange { range, text } => {
                // `ReplaceRange` is expanded into primitive operations.
                //
                // This keeps the action layer expressive while still allowing
                // the editor core to work with simple buffer primitives.
                //
                // If the replacement text is empty, this becomes a delete.
                if text.is_empty() {
                    vec![Operation::Buffer(BufferOp::DeleteChars { range })]
                } else if range.start == range.end {
                    // Empty range means this is really just an insertion.
                    vec![Operation::Buffer(BufferOp::InsertStr {
                        char_index: range.start,
                        text,
                    })]
                } else {
                    // General case:
                    // 1. delete the old range
                    // 2. insert the replacement text at the start
                    vec![
                        Operation::Buffer(BufferOp::DeleteChars { range }),
                        Operation::Buffer(BufferOp::InsertStr {
                            char_index: range.start,
                            text,
                        }),
                    ]
                }
            }

            Action::MoveCursorLeft => {
                vec![Operation::Cursor(CursorOp::OffsetCharIndex {
                    offset: 1,
                    positive: false,
                })]
            }

            Action::MoveCursorRight => {
                vec![Operation::Cursor(CursorOp::OffsetCharIndex {
                    offset: 1,
                    positive: true,
                })]
            }

            Action::SetCursor { char_index } => {
                vec![Operation::Cursor(CursorOp::SetCharIndex(char_index))]
            }
        }
    }
}