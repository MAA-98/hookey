use super::*;
use crate::domain::{BufInt, BufRange, LineCount, LineIndex};

#[test]
fn new_buffer_len_chars() {
    let buf = Buffer::new();
    assert_eq!(buf.len_chars().raw(), 0);
}

#[test]
fn can_create_from_str() {
    let buf = Buffer::from_str("hello");
    assert_eq!(buf.as_string(), "hello");
}

#[test]
fn len_lines_for_single_line_without_newline() {
    let buf = Buffer::from_str("hello");
    assert_eq!(buf.len_lines(), LineCount::new(1));
}

#[test]
fn len_lines_for_single_line_with_trailing_newline() {
    let buf = Buffer::from_str("hello\n");

    // A trailing newline means Ropey counts an extra empty line at the end.
    assert_eq!(buf.len_lines(), LineCount::new(2));
}

#[test]
fn len_lines_for_two_lines_without_trailing_newline() {
    let buf = Buffer::from_str("hello\nworld");
    assert_eq!(buf.len_lines(), LineCount::new(2));
}

#[test]
fn len_lines_for_two_lines_with_trailing_newline() {
    let buf = Buffer::from_str("hello\nworld\n");

    // Two explicit lines plus the trailing empty line.
    assert_eq!(buf.len_lines(), LineCount::new(3));
}

#[test]
fn len_lines_for_multiple_empty_lines() {
    let buf = Buffer::from_str("\n\n");

    // Two newline characters means three lines total:
    // 1. empty line before first newline
    // 2. empty line between newlines
    // 3. empty line after the last newline
    assert_eq!(buf.len_lines(), LineCount::new(3));
}

#[test]
fn insert_works() {
    let mut buf = Buffer::from_str("heo");

    // Insert "ll" at character index 2.
    buf.insert_str(BufInt::new(2), "ll").unwrap();

    assert_eq!(buf.as_string(), "hello");
    assert_eq!(buf.len_chars(), BufInt::new(5));
    assert_eq!(buf.len_lines(), LineCount::new(1));
}

#[test]
fn delete_chars_works() {
    let mut buf = Buffer::from_str("hello");

    // Delete characters at indices 1, 2, and 3.
    // Because the range is half-open, `1..4` means:
    // start at 1, stop before 4.
    buf.delete_chars(BufRange::new(BufInt::new(1), BufInt::new(4)))
        .unwrap();

    assert_eq!(buf.as_string(), "ho");
    assert_eq!(buf.len_chars(), BufInt::new(2));
    assert_eq!(buf.len_lines(), LineCount::new(1));
}

#[test]
fn deleting_newline_changes_line_count() {
    let mut buf = Buffer::from_str("hello\nworld");

    assert_eq!(buf.len_lines(), LineCount::new(2));

    // Remove the newline character.
    buf.delete_chars(BufRange::new(BufInt::new(5), BufInt::new(6)))
        .unwrap();

    assert_eq!(buf.as_string(), "helloworld");
    assert_eq!(buf.len_lines(), LineCount::new(1));
}

#[test]
fn inserting_newline_changes_line_count() {
    let mut buf = Buffer::from_str("helloworld");

    assert_eq!(buf.len_lines(), LineCount::new(1));

    // Insert a newline between hello and world.
    buf.insert_str(BufInt::new(5), "\n").unwrap();

    assert_eq!(buf.as_string(), "hello\nworld");
    assert_eq!(buf.len_lines(), LineCount::new(2));
}

#[test]
fn delete_chars_rejects_malformed_range() {
    let mut buf = Buffer::from_str("hello");

    // Malformed means start > end.
    let err = buf
        .delete_chars(BufRange::new(BufInt::new(4), BufInt::new(1)))
        .unwrap_err();

    assert_eq!(
        err,
        BufferError::MalformedCharRange {
            range: BufRange::new(BufInt::new(4), BufInt::new(1)),
        }
    );
}

#[test]
fn delete_chars_rejects_out_of_bounds_range() {
    let mut buf = Buffer::from_str("hello");

    // This range is well-formed, but it extends past the end of the buffer.
    let err = buf
        .delete_chars(BufRange::new(BufInt::new(0), BufInt::new(999)))
        .unwrap_err();

    assert_eq!(
        err,
        BufferError::OutOfBoundsCharRange {
            range: BufRange::new(BufInt::new(0), BufInt::new(999)),
            len: BufInt::new(5),
        }
    );
}

//
// ------------------------------------------------------------
// Tests for the new APIs:
// - char_to_line
// - line_to_char
// - char_at
// - append_str
// ------------------------------------------------------------
//

#[test]
fn char_to_line_maps_char_positions_across_multiple_lines() {
    // This buffer has 3 lines:
    // 1. "ab\n"
    // 2. "cd\n"
    // 3. "ef"
    //
    // Character positions are:
    // 0 = 'a'
    // 1 = 'b'
    // 2 = '\n'
    // 3 = 'c'
    // 4 = 'd'
    // 5 = '\n'
    // 6 = 'e'
    // 7 = 'f'
    let buf = Buffer::from_str("ab\ncd\nef");

    // Characters on the first line, including the newline, map to line 0.
    assert_eq!(
        buf.char_to_line(BufInt::new(0)).unwrap(),
        LineIndex::new(0)
    );
    assert_eq!(
        buf.char_to_line(BufInt::new(1)).unwrap(),
        LineIndex::new(0)
    );
    assert_eq!(
        buf.char_to_line(BufInt::new(2)).unwrap(),
        LineIndex::new(0)
    );

    // Characters on the second line, including the newline, map to line 1.
    assert_eq!(
        buf.char_to_line(BufInt::new(3)).unwrap(),
        LineIndex::new(1)
    );
    assert_eq!(
        buf.char_to_line(BufInt::new(4)).unwrap(),
        LineIndex::new(1)
    );
    assert_eq!(
        buf.char_to_line(BufInt::new(5)).unwrap(),
        LineIndex::new(1)
    );

    // Characters on the third line map to line 2.
    assert_eq!(
        buf.char_to_line(BufInt::new(6)).unwrap(),
        LineIndex::new(2)
    );
    assert_eq!(
        buf.char_to_line(BufInt::new(7)).unwrap(),
        LineIndex::new(2)
    );
}

#[test]
fn char_to_line_works_on_empty_buffer() {
    let buf = Buffer::new();

    // Ropey treats an empty buffer as one line.
    assert_eq!(
        buf.char_to_line(BufInt::new(0)).unwrap(),
        LineIndex::new(0)
    );
}

#[test]
fn char_to_line_rejects_out_of_bounds_index() {
    let buf = Buffer::from_str("hello");

    let err = buf.char_to_line(BufInt::new(999)).unwrap_err();

    assert_eq!(
        err,
        BufferError::InvalidBufferIndex {
            index: BufInt::new(999),
            len: BufInt::new(5),
        }
    );
}

#[test]
fn line_to_char_maps_lines_to_their_start_positions() {
    // Same buffer as the `char_to_line` test.
    let buf = Buffer::from_str("ab\ncd\nef");

    // Line 0 starts at character 0.
    assert_eq!(
        buf.line_to_char(LineIndex::new(0)).unwrap(),
        BufInt::new(0)
    );

    // Line 1 starts after "ab\n" => character 3.
    assert_eq!(
        buf.line_to_char(LineIndex::new(1)).unwrap(),
        BufInt::new(3)
    );

    // Line 2 starts after "ab\ncd\n" => character 6.
    assert_eq!(
        buf.line_to_char(LineIndex::new(2)).unwrap(),
        BufInt::new(6)
    );
}

#[test]
fn line_to_char_works_on_empty_buffer() {
    let buf = Buffer::new();

    // Even an empty buffer has a single line, and that line starts at 0.
    assert_eq!(
        buf.line_to_char(LineIndex::new(0)).unwrap(),
        BufInt::new(0)
    );
}

#[test]
fn line_to_char_rejects_out_of_bounds_index() {
    let buf = Buffer::from_str("hello");

    // There is only one line, so line index 999 is invalid.
    let err = buf.line_to_char(LineIndex::new(999)).unwrap_err();

    assert_eq!(
        err,
        BufferError::InvalidLineIndex {
            index: LineIndex::new(999),
            len: LineCount::new(1),
        }
    );
}

#[test]
fn char_at_returns_expected_ascii_and_newline_chars() {
    let buf = Buffer::from_str("ab\ncd");

    // Check normal characters.
    assert_eq!(buf.char_at(BufInt::new(0)), Some('a'));
    assert_eq!(buf.char_at(BufInt::new(1)), Some('b'));

    // The newline is also a character in the buffer.
    assert_eq!(buf.char_at(BufInt::new(2)), Some('\n'));

    // Then the next line continues normally.
    assert_eq!(buf.char_at(BufInt::new(3)), Some('c'));
    assert_eq!(buf.char_at(BufInt::new(4)), Some('d'));
}

#[test]
fn char_at_handles_unicode_characters() {
    // This string includes:
    // - ASCII: 'a'
    // - Latin-1: 'é'
    // - Emoji: '🙂'
    //
    // This is a good sanity check that we're indexing by *character*,
    // not by byte.
    let buf = Buffer::from_str("aé🙂");

    assert_eq!(buf.len_chars(), BufInt::new(3));

    assert_eq!(buf.char_at(BufInt::new(0)), Some('a'));
    assert_eq!(buf.char_at(BufInt::new(1)), Some('é'));
    assert_eq!(buf.char_at(BufInt::new(2)), Some('🙂'));
}

#[test]
fn char_at_returns_none_for_out_of_bounds_index() {
    let buf = Buffer::from_str("hello");

    // Index 5 is one past the last valid character index.
    assert_eq!(buf.char_at(BufInt::new(5)), None);

    // A much larger index should also return None.
    assert_eq!(buf.char_at(BufInt::new(999)), None);
}

#[test]
fn append_str_appends_to_empty_buffer() {
    let mut buf = Buffer::new();

    // Start from empty.
    assert_eq!(buf.len_chars().raw(), 0);

    // Append some text.
    buf.append_str("hello");

    assert_eq!(buf.as_string(), "hello");
    assert_eq!(buf.len_chars(), BufInt::new(5));
    assert_eq!(buf.len_lines(), LineCount::new(1));
}

#[test]
fn append_str_appends_to_existing_buffer() {
    let mut buf = Buffer::from_str("hello");

    // Appending should preserve the existing contents and add to the end.
    buf.append_str(" world");

    assert_eq!(buf.as_string(), "hello world");
    assert_eq!(buf.len_chars(), BufInt::new(11));
    assert_eq!(buf.len_lines(), LineCount::new(1));
}

#[test]
fn append_str_can_append_newlines_and_change_line_count() {
    let mut buf = Buffer::from_str("hello");

    // Appending a newline should create a second line.
    buf.append_str("\nworld");

    assert_eq!(buf.as_string(), "hello\nworld");
    assert_eq!(buf.len_chars(), BufInt::new(11));
    assert_eq!(buf.len_lines(), LineCount::new(2));
}

#[test]
fn append_str_can_be_called_multiple_times() {
    let mut buf = Buffer::new();

    // Build up the buffer in steps.
    buf.append_str("hello");
    buf.append_str("\n");
    buf.append_str("world");

    assert_eq!(buf.as_string(), "hello\nworld");
    assert_eq!(buf.len_chars(), BufInt::new(11));
    assert_eq!(buf.len_lines(), LineCount::new(2));
}