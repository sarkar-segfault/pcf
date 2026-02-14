use alloc::string::String;
use core::{fmt, str};

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

    pub fn new_line(&mut self) {
        self.line += 1;
        self.col = 1;
    }

    pub fn new_col(&mut self) {
        self.col += 1;
    }
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Span {
    pub begin: Location,
    pub end: Location,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.begin.line == self.end.line {
            if self.begin.col == self.end.col {
                write!(f, ":{}:{}", self.begin.line, self.begin.col)
            } else {
                write!(
                    f,
                    ":{} {}..{}",
                    self.begin.line, self.begin.col, self.end.col
                )
            }
        } else {
            write!(
                f,
                " {}:{}..{}:{}",
                self.begin.line, self.begin.col, self.end.line, self.end.col
            )
        }
    }
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
    pub fn new(file: &'a str, content: String) -> Self {
        Self { file, content }
    }

    pub fn chars(&self) -> core::iter::Peekable<str::Chars<'_>> {
        self.content.chars().peekable()
    }

    pub fn extract_offset(&self, tar: Location) -> usize {
        let mut loc = Location::default();
        let mut offset: usize = 0;

        for chr in self.chars() {
            if loc.line == tar.line && loc.col == tar.col {
                break;
            }

            offset += chr.len_utf8();

            if chr == '\n' {
                loc.new_line();
            } else {
                loc.new_col();
            }
        }

        offset
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexingError {
    MalformedNumber,
    UnrecognizedToken,
    UnterminatedString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsingError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Lexing(LexingError),
    Parsing(ParsingError),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Lexing(l) => match l {
                    LexingError::MalformedNumber => "encountered malformed number during lexing",
                    LexingError::UnrecognizedToken =>
                        "encountered unrecognized token during lexing",
                    LexingError::UnterminatedString =>
                        "encountered unterminated string during lexing",
                },
                Self::Parsing(p) => match p {
                    _ => "",
                },
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error<'a> {
    pub span: Span,
    pub src: &'a Source<'a>,
    pub kind: ErrorKind,
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}{}] {}\n{}",
            self.src.file,
            self.span,
            self.kind,
            self.src
                .content
                .get(
                    self.src.extract_offset(self.span.begin)
                        ..self.src.extract_offset(self.span.end)
                )
                .unwrap_or("<failed to extract offsets of begin and end>")
        )
    }
}

impl<'a> core::error::Error for Error<'a> {}

impl<'a> Error<'a> {
    pub fn new(kind: ErrorKind, span: Span, src: &'a Source<'a>) -> Self {
        Self { kind, span, src }
    }

    pub fn lexing(kind: LexingError, span: Span, src: &'a Source<'a>) -> Self {
        Self::new(ErrorKind::Lexing(kind), span, src)
    }

    pub fn parsing(kind: ParsingError, span: Span, src: &'a Source<'a>) -> Self {
        Self::new(ErrorKind::Parsing(kind), span, src)
    }
}

pub type Result<'a, T> = core::result::Result<T, Error<'a>>;
