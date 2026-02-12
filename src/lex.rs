use alloc::string::String;
use std::str;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

impl Default for Location {
    fn default() -> Self {
        Self { line: 1, col: 1 }
    }
}

impl Location {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Span {
    pub begin: Location,
    pub end: Location,
}

impl Span {
    pub fn new(begin: Location, end: Location) -> Self {
        Self { begin, end }
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Source<'a> {
    pub file: &'a str,
    pub content: String,
}

impl<'a> Source<'a> {
    #[cfg(feature = "std")]
    pub fn from_file(file: &'a str) -> std::io::Result<Self> {
        Ok(Self {
            file,
            content: std::fs::read_to_string(file)?,
        })
    }

    pub fn new(file: &'a str, content: String) -> Self {
        Self { file, content }
    }

    pub fn chars(&self) -> std::iter::Peekable<str::Chars<'_>> {
        self.content.chars().peekable()
    }

    pub fn lines(&self) -> str::Lines<'_> {
        self.content.lines()
    }
}
