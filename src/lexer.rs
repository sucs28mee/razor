use std::str::FromStr;

use crate::util::{Span, Spanned};

#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    Assignment(AssignmentKind),
    Eq,
    Brace { open: bool, kind: BraceKind },
    QuestionMark,
    SemiColon,
    Colon,
    Ampersand,
    Comma,
    Literal(LiteralKind),
    Keyword(KeywordKind),
}

#[derive(Debug, Clone)]
pub enum KeywordKind {
    Struct,
    Fn,
}

#[derive(Debug, Clone, Copy)]
pub struct UnknownKeywordError;

impl FromStr for KeywordKind {
    type Err = UnknownKeywordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "struct" => Ok(KeywordKind::Struct),
            "fn" => Ok(KeywordKind::Fn),
            _ => Err(UnknownKeywordError),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AssignmentKind {
    Normal,
    Optional,
}

#[derive(Debug, Clone)]
pub enum LiteralKind {
    String(String),
    Int(i32),
    Float(f32),
}

#[derive(Debug, Clone, Copy)]
pub enum BraceKind {
    Curly,
    Square,
    Smooth,
}

pub struct Lexer<'a> {
    len: usize,
    bytes: &'a [u8],
}

impl<'a> Lexer<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            len: bytes.len(),
            bytes,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LexerError {
    UnexpectedCharacter(char),
    NumberParseError,
    NonUtf8Bytes,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Span<Result<Token, LexerError>>;

    fn next(&mut self) -> Option<Self::Item> {
        fn parse(first: u8, bytes: &[u8]) -> (Result<Token, LexerError>, usize) {
            match first {
                // Identifiers and keywords:
                // --------------------------@
                b'a'..=b'z' | b'A'..=b'Z' => {
                    let bytes = (0..)
                        .map_while(|i| {
                            bytes
                                .get(i)
                                .filter(|byte| matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'.'))
                                .copied()
                        })
                        .collect::<Vec<_>>();

                    let len = bytes.len();
                    let Ok(string) = String::from_utf8(bytes) else {
                        return (Err(LexerError::NonUtf8Bytes), len);
                    };

                    if let Ok(keyword) = string.parse() {
                        return (Ok(Token::Keyword(keyword)), len);
                    }

                    (Ok(Token::Ident(string)), len)
                }
                // Literals:
                // --------@
                b'"' => {
                    // TODO: Add support for escape characters.
                    let bytes = (1..)
                        .map_while(|i| bytes.get(i).filter(|byte| **byte != b'"').copied())
                        .collect::<Vec<_>>();

                    let len = bytes.len() + 2;
                    let Ok(string) = String::from_utf8(bytes) else {
                        return (Err(LexerError::NonUtf8Bytes), len);
                    };

                    (Ok(Token::Literal(LiteralKind::String(string))), len)
                }
                b'0'..=b'9' | b'.' => {
                    let bytes = (0..)
                        .map_while(|i| {
                            bytes
                                .get(i)
                                .filter(|byte| matches!(byte, b'0'..=b'9' | b'.'))
                                .copied()
                        })
                        .collect::<Vec<_>>();

                    let len = bytes.len();
                    let Ok(string) = String::from_utf8(bytes) else {
                        return (Err(LexerError::NonUtf8Bytes), len);
                    };

                    if let Ok(int) = string.parse() {
                        return (Ok(Token::Literal(LiteralKind::Int(int))), len);
                    }

                    if let Ok(float) = string.parse() {
                        return (Ok(Token::Literal(LiteralKind::Float(float))), len);
                    }

                    (Err(LexerError::NumberParseError), len)
                }
                // Braces:
                //---------------------@
                b'{' => (
                    Ok(Token::Brace {
                        open: true,
                        kind: BraceKind::Curly,
                    }),
                    1,
                ),
                b'}' => (
                    Ok(Token::Brace {
                        open: false,
                        kind: BraceKind::Curly,
                    }),
                    1,
                ),
                b'(' => (
                    Ok(Token::Brace {
                        open: true,
                        kind: BraceKind::Smooth,
                    }),
                    1,
                ),
                b')' => (
                    Ok(Token::Brace {
                        open: false,
                        kind: BraceKind::Smooth,
                    }),
                    1,
                ),
                b'[' => (
                    Ok(Token::Brace {
                        open: true,
                        kind: BraceKind::Square,
                    }),
                    1,
                ),
                b']' => (
                    Ok(Token::Brace {
                        open: false,
                        kind: BraceKind::Square,
                    }),
                    1,
                ),
                // Other:
                // -------------------------------------@
                b':' => {
                    if let Some(b'=') = bytes.get(1) {
                        (Ok(Token::Assignment(AssignmentKind::Normal)), 2)
                    } else {
                        (Ok(Token::Colon), 1)
                    }
                }
                b'?' => {
                    if let Some(b'=') = bytes.get(1) {
                        (Ok(Token::Assignment(AssignmentKind::Optional)), 2)
                    } else {
                        (Ok(Token::QuestionMark), 1)
                    }
                }
                b';' => (Ok(Token::SemiColon), 1),
                b'&' => (Ok(Token::Ampersand), 1),
                b',' => (Ok(Token::Comma), 1),
                b'=' => (Ok(Token::Eq), 1),
                byte => (Err(LexerError::UnexpectedCharacter(byte as char)), 1),
            }
        }

        // Skip any whitespace.
        while self.bytes.first()?.is_ascii_whitespace() {
            self.bytes = &self.bytes[1..];
        }

        // The starting index of the next token.
        let index = self.len - self.bytes.len();
        let (token, len) = parse(self.bytes.first().copied()?, self.bytes);

        self.bytes = &self.bytes[len.min(self.bytes.len())..];
        Some(token.spanned(index, len))
    }
}
