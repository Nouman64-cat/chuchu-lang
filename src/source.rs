//! Source identity and UTF-8 byte positions shared by diagnostics and debug metadata.
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePosition {
    pub line: u32,
    /// One-based UTF-8 byte column. Tabs occupy one byte, not a display tab stop.
    pub column: u32,
}

#[derive(Debug, Clone)]
pub struct SourceFile {
    path: PathBuf,
    text: String,
    line_starts: Vec<usize>,
}

impl SourceFile {
    pub fn new(path: PathBuf, text: String) -> Self {
        let mut line_starts = vec![0];
        line_starts.extend(text.match_indices('\n').map(|(offset, _)| offset + 1));
        Self {
            path,
            text,
            line_starts,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Reject invalid UTF-8 boundaries, out-of-range offsets and metadata overflow.
    pub fn position(&self, offset: usize) -> Option<SourcePosition> {
        if !self.text.is_char_boundary(offset) {
            return None;
        }
        let index = self.line_starts.partition_point(|start| *start <= offset) - 1;
        Some(SourcePosition {
            line: u32::try_from(index.checked_add(1)?).ok()?,
            column: u32::try_from(
                offset
                    .checked_sub(self.line_starts[index])?
                    .checked_add(1)?,
            )
            .ok()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_bytes_crlf_tabs_unicode_and_eof() {
        let file = SourceFile::new("test.chuchu".into(), "a\r\n\té\n".into());
        assert_eq!(
            file.position(0),
            Some(SourcePosition { line: 1, column: 1 })
        );
        assert_eq!(
            file.position(3),
            Some(SourcePosition { line: 2, column: 1 })
        );
        assert_eq!(
            file.position(4),
            Some(SourcePosition { line: 2, column: 2 })
        );
        assert_eq!(file.position(5), None);
        assert_eq!(
            file.position(6),
            Some(SourcePosition { line: 2, column: 4 })
        );
        assert_eq!(
            file.position(7),
            Some(SourcePosition { line: 3, column: 1 })
        );
        assert_eq!(file.position(8), None);
        let empty = SourceFile::new("empty.chuchu".into(), String::new());
        assert_eq!(
            empty.position(0),
            Some(SourcePosition { line: 1, column: 1 })
        );
    }
}
