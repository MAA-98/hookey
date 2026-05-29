use crate::buffer::{Buffer, BufferError};
use crate::cursor::Cursor;
use crate::domain::{BufInt, BufRange};
use crate::actions::Action;

/// A high-level operation that can be applied by the editor.
///
/// This is the domain language layer for the editor core.
///
/// Instead of exposing raw buffer/cursor mutation everywhere, the editor can
/// accept operations that describe *what kind of change* should happen.
///
/// In the current version, operations are just thin wrappers around the
/// primitive `Buffer` and `Cursor` methods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Buffer(BufferOp),
    Cursor(CursorOp),
}

/// Primitive operations that mutate the buffer.
///
/// These map directly to methods on `Buffer`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferOp {
    /// Insert text at a specific character index.
    InsertStr {
        char_index: BufInt,
        text: String,
    },

    /// Append text to the end of the buffer.
    AppendStr {
        text: String,
    },

    /// Delete a half-open character range from the buffer.
    DeleteChars {
        range: BufRange,
    },
}


/// Primitive operations that mutate the cursor.
///
/// These map directly to methods on `Cursor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorOp {
    /// Set the cursor to an exact character index.
    ///
    /// This does not clamp automatically.
    SetCharIndex(BufInt),

    /// Offset the cursor.
    ///
    /// Positive values move right.
    /// Negative values move left.
    ///
    /// This also does not clamp automatically.
    OffsetCharIndex { offset: usize, positive: bool },

    /// Clamp the cursor to the current buffer bounds.
    ///
    /// This is the explicit "make the cursor valid again" operation.
    ClampToBuffer,
}

/// A callback that runs after an action has been applied successfully.
///
/// The hook receives immutable access to the editor state.
///
/// This keeps the hook simple for now:
/// - it can inspect the buffer
/// - it can inspect the cursor
/// - it cannot mutate editor state directly
///
/// If you later want a more powerful hook system, you can expand this into
/// a trait or a pipeline.
type PostActionHook = Box<dyn FnMut(&Buffer, &Cursor)>;

/// The main editor core.
pub struct Editor {
    buffer: Buffer,
    cursor: Cursor,

    /// Optional post-action hook.
    ///
    /// If present, this is called after `apply_action()` completes
    /// successfully.
    post_action_hook: Option<PostActionHook>,
}

impl Editor {
    /// Create a new editor from an existing buffer.
    ///
    /// The cursor starts at index 0 by default.
    pub fn new(buffer: Buffer) -> Self {
        Self {
            buffer,
            cursor: Cursor::default(),
            post_action_hook: None,
        }
    }

    /// Create a new editor from an existing buffer and cursor.
    pub fn with_cursor(buffer: Buffer, cursor: Cursor) -> Self {
        Self {
            buffer,
            cursor,
            post_action_hook: None,
        }
    }

    /// Get an immutable reference to the buffer.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Get an immutable reference to the cursor.
    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    /// Set a hook that runs after each successfully applied action.
    ///
    /// This replaces any existing hook.
    pub fn set_post_action_hook<F>(&mut self, hook: F)
    where
        F: FnMut(&Buffer, &Cursor) + 'static,
    {
        self.post_action_hook = Some(Box::new(hook));
    }

    /// Remove the current post-action hook.
    pub fn clear_post_action_hook(&mut self) {
        self.post_action_hook = None;
    }

    /// Apply a high-level action.
    ///
    /// This is the layer above primitive operations.
    /// For now, we simply:
    /// - convert the Action into a list of Operations
    /// - apply each operation in order
    /// - run the post-action hook if the action succeeded
    pub fn apply_action(&mut self, action: Action) -> Result<(), BufferError> {
        let operations = action.into_operations();

        for operation in operations {
            self.apply_operation(operation)?;
        }

        if let Some(hook) = self.post_action_hook.as_mut() {
            hook(&self.buffer, &self.cursor);
        }

        Ok(())
    }

    /// Apply a primitive editor operation.
    ///
    /// This is the main entry point for the editor's low-level operation API.
    ///
    /// Buffer operations may fail, so this returns `Result`.
    /// Cursor operations are infallible.
    pub fn apply_operation(&mut self, operation: Operation) -> Result<(), BufferError> {
        match operation {
            Operation::Buffer(buffer_op) => self.apply_buffer_op(buffer_op),
            Operation::Cursor(cursor_op) => {
                self.apply_cursor_op(cursor_op);
                Ok(())
            }
        }
    }


    /// Apply a primitive buffer operation.
    ///
    /// Buffer operations only mutate the buffer.
    /// They do not implicitly clamp or adjust the cursor.
    fn apply_buffer_op(&mut self, op: BufferOp) -> Result<(), BufferError> {
        match op {
            BufferOp::InsertStr { char_index, text } => {
                self.buffer.insert_str(char_index, &text)?;
                Ok(())
            }

            BufferOp::AppendStr { text } => {
                self.buffer.append_str(&text);
                Ok(())
            }

            BufferOp::DeleteChars { range } => {
                self.buffer.delete_chars(range)?;
                Ok(())
            }
        }
    }

    /// Apply a primitive cursor operation.
    fn apply_cursor_op(&mut self, op: CursorOp) {
        match op {
            CursorOp::SetCharIndex(char_index) => {
                self.cursor.set_char_index(char_index);
            }

            CursorOp::OffsetCharIndex { offset, positive } => {
                self.cursor.offset_char_index(offset, positive);
            }

            CursorOp::ClampToBuffer => {
                self.cursor.clamp_to_buffer(&self.buffer);
            }
        }
    }
}