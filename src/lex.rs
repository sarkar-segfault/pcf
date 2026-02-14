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

    pub fn lines(&self) -> str::Lines<'_> {
        self.content.lines()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    MalformedNumber,
    UnrecognizedToken,
    UnterminatedString,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "encountered {}",
            match self {
                Self::MalformedNumber => "malformed number",
                Self::UnrecognizedToken => "unrecognized token",
                Self::UnterminatedString => "unterminated string",
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
        write!(f, "[{}{}] {}", self.src.file, self.span, self.kind)
    }
}

impl<'a> Error<'a> {
    pub fn new(kind: ErrorKind, span: Span, src: &'a Source<'a>) -> Self {
        Self { kind, span, src }
    }
}

pub type Result<'a, T> = core::result::Result<T, Error<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub enum LexemeKind {
    String(String),
    Ident(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    LBrack,
    RBrack,
    LBrace,
    RBrace,
    Equal,
    Comma,
}

#[derive(Debug, Clone, PartialEq)]
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

pub fn is_identifier(chr: char) -> bool {
    chr.is_alphanumeric() || chr == '_'
}

pub fn is_numeric_or_symbol(chr: char) -> bool {
    chr.is_numeric() || chr == '-' || chr == '+' || chr == '.'
}

pub fn lex<'a>(src: &'a Source<'a>) -> Result<'a, LexemeStream> {
    let mut lexemes = LexemeStream::default();
    let mut span = Span::default();
    let mut chars = src.chars();

    while let Some(tok) = chars.next() {
        span.begin = span.end;
        span.end.new_col();

        lexemes.push_back(Lexeme::new(
            match tok {
                '=' => LexemeKind::Equal,
                ',' => LexemeKind::Comma,
                '[' => LexemeKind::LBrack,
                ']' => LexemeKind::RBrack,
                '{' => LexemeKind::LBrace,
                '}' => LexemeKind::RBrace,
                '"' => {
                    let mut content = String::default();
                    let mut closed = false;

                    for chr in chars.by_ref() {
                        span.end.new_col();

                        if chr == '"' {
                            closed = true;
                            break;
                        }

                        content.push(chr);
                    }

                    if !closed {
                        return Err(Error::new(ErrorKind::UnterminatedString, span, src));
                    }

                    LexemeKind::String(content)
                }
                _ if is_numeric_or_symbol(tok) => {
                    let mut content = String::default();
                    content.push(tok);

                    let mut dot = tok == '.';

                    while let Some(&chr) = chars.peek() {
                        if chr == '.' {
                            if dot {
                                return Err(Error::new(ErrorKind::MalformedNumber, span, src));
                            }

                            dot = true;
                        }

                        if !chr.is_numeric() && chr != '.' {
                            break;
                        }

                        chars.next();
                        content.push(chr);
                        span.end.new_col();
                    }

                    if dot {
                        LexemeKind::Float(
                            content
                                .parse::<f64>()
                                .map_err(|_| Error::new(ErrorKind::MalformedNumber, span, src))?,
                        )
                    } else {
                        LexemeKind::Integer(
                            content
                                .parse::<i64>()
                                .map_err(|_| Error::new(ErrorKind::MalformedNumber, span, src))?,
                        )
                    }
                }
                _ if is_identifier(tok) => {
                    let mut content = String::default();
                    content.push(tok);

                    while let Some(&chr) = chars.peek() {
                        if !is_identifier(chr) {
                            break;
                        }

                        chars.next();
                        span.end.new_col();
                        content.push(chr);
                    }

                    match content.as_str() {
                        "true" => LexemeKind::Bool(true),
                        "false" => LexemeKind::Bool(false),
                        _ => LexemeKind::Ident(content),
                    }
                }
                '#' => {
                    for chr in chars.by_ref() {
                        if chr == '\n' {
                            span.end.new_line();
                            break;
                        }

                        span.end.new_col();
                    }

                    continue;
                }
                '\n' => {
                    span.end.new_line();
                    continue;
                }
                _ if tok.is_whitespace() => {
                    continue;
                }
                _ => return Err(Error::new(ErrorKind::UnrecognizedToken, span, src)),
            },
            span,
        ));
    }

    Ok(lexemes)
}
