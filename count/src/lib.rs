//! Utilities for counting items in buffered input.
//!
//! This module exposes helpers that operate on types implementing `std::io::BufRead`.
//! The primary function `count_lines` consumes a buffered reader and returns the
//! number of lines produced by its iterator.


use std::io::BufRead;

/// Count the number of lines in `input`.
///
/// The function accepts any `T` implementing `BufRead` and returns the number of
/// elements produced by the iterator returned from `BufRead::lines()`.
/// Note that this function consumes the reader. Any I/O errors produced while
/// iterating are not propagated — the function simply counts the iterator's
/// items and returns that count as a `usize`.
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// use count::count_lines;
/// let input = Cursor::new("a\nb\nc\n");
/// assert_eq!(count_lines(input), 3);
/// ```
pub fn count_lines<T: BufRead>(input: T) -> usize {
    input.lines().count()
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn count_lines_returns_expected_line_count() {
        let input = Cursor::new("a\nb\nc\n");
        let lc = count_lines(input);
        let expect_lc = 3;
        assert_eq!(lc, expect_lc, "wrong line count - expected {expect_lc}, got {lc}");
    }
}
