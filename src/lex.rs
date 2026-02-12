use alloc::{collections::vec_deque::VecDeque, string::String};
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

    pub fn lines(&self) -> str::Lines<'_> {
        self.content.lines()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    UnrecognizedToken,
    UnterminatedString,
    UnrecognizedScalar,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UnrecognizedToken => "encountered unrecognized token",
                Self::UnterminatedString => "encountered unterminated string",
                Self::UnrecognizedScalar => "encountered unrecognized top-level scalar",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error<'a> {
    pub span: Span,
    pub src: Source<'a>,
    pub kind: ErrorKind,
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}{}] {}", self.src.file, self.span, self.kind)
    }
}

impl<'a> Error<'a> {
    pub fn new(kind: ErrorKind, span: Span, src: Source<'a>) -> Self {
        Self { kind, span, src }
    }
}

pub type Result<'a, T> = core::result::Result<T, Error<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexemeKind {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lexeme {
    pub kind: LexemeKind,
    pub span: Span,
}

impl Lexeme {
    pub fn new(kind: LexemeKind, span: Span) -> Self {
        Self { kind, span }
    }
}

pub type LexemeStream = VecDeque<Lexeme>;

pub fn lex<'a>(src: Source<'a>) -> Result<LexemeStream> {
    todo!()
}
