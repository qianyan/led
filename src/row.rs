use std::cmp;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            string: String::from(slice),
            len: 0,
        };
        row.update_len();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        #[allow(clippy::arithmetic_side_effects)]
        self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .map(|grapheme| if grapheme == "\t" { " " } else { grapheme })
            .collect()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count();
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
        } else {
            let mut result: String = self.string[..].graphemes(true).take(at).collect();
            let remainer: String = self.string[..].graphemes(true).skip(at).collect();
            result.push(c);
            result.push_str(&remainer);
            self.string = result;
        }
        self.update_len();
    }

    #[allow(clippy::arithmetic_side_effects)]
    pub fn delete(&mut self, at: usize) {
        if at > self.len() {
            return;
        }
        let mut result: String = self.string[..].graphemes(true).take(at).collect();
        let remainer: String = self.string[..].graphemes(true).skip(at + 1).collect();
        result.push_str(&remainer);
        self.string = result;
        self.update_len();
    }

    pub fn append(&mut self, next_row: Row) {
        self.string = format!("{}{}", self.string, next_row.string);
        self.update_len()
    }

    pub fn split(&mut self, at: usize) -> Row {
        let begin: String = self.string[..].graphemes(true).take(at).collect();
        let remainder: String = self.string[..].graphemes(true).skip(at).collect();
        self.string = begin;
        self.update_len();
        Self::from(&remainder[..])
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }
}
