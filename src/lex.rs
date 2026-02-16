use crate::utils::*;
use alloc::{collections::vec_deque::VecDeque, string::String};

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
    chr.is_ascii_alphanumeric() || chr == '_'
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
                    let mut prev = '\0';

                    for chr in chars.by_ref() {
                        if chr == '\n' {
                            span.end.new_line();
                        } else {
                            span.end.new_col();
                        }

                        if chr == '"' && prev != '\\' {
                            prev = chr;
                            break;
                        }

                        prev = chr;
                        content.push(chr);
                    }

                    if prev != '"' {
                        return Err(Error::lexing(LexingError::UnterminatedString, span, src));
                    }

                    LexemeKind::String(content)
                }
                _ if tok.is_ascii_numeric() || tok == '-' || tok == '+' || tok == '.' => {
                    let mut content = String::default();
                    content.push(tok);

                    let mut dot = tok == '.';

                    while let Some(&chr) = chars.peek() {
                        if chr == '.' {
                            if dot {
                                return Err(Error::lexing(LexingError::MalformedNumber, span, src));
                            }

                            dot = true;
                        }

                        if !chr.is_ascii_numeric() && chr != '.' {
                            break;
                        }

                        chars.next();
                        content.push(chr);
                        span.end.new_col();
                    }

                    if dot {
                        LexemeKind::Float(
                            content.parse::<f64>().map_err(|_| {
                                Error::lexing(LexingError::MalformedNumber, span, src)
                            })?,
                        )
                    } else {
                        LexemeKind::Integer(
                            content.parse::<i64>().map_err(|_| {
                                Error::lexing(LexingError::MalformedNumber, span, src)
                            })?,
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
                _ => return Err(Error::lexing(LexingError::UnrecognizedToken, span, src)),
            },
            span,
        ));
    }

    Ok(lexemes)
}
