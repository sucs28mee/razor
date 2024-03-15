use crate::util::{Span, Spanned};

#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    FunctionIdent(String),
    Assignment { mutable: bool },
    Brace { open: bool, kind: BraceKind },
    SemiColon,
    Literal(Literal),
}

#[derive(Debug, Clone)]
pub enum Literal {
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
    ExpectedCharacter { expected: char, found: Option<char> },
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
                                .filter(|byte| matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'))
                                .copied()
                        })
                        .collect::<Vec<_>>();

                    let len = bytes.len();
                    let Ok(string) = String::from_utf8(bytes) else {
                        return (Err(LexerError::NonUtf8Bytes), len);
                    };

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

                    (Ok(Token::Literal(Literal::String(string))), len)
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
                        return (Ok(Token::Literal(Literal::Int(int))), len);
                    }

                    if let Ok(float) = string.parse() {
                        return (Ok(Token::Literal(Literal::Float(float))), len);
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
                b':' => match bytes.get(1) {
                    Some(b'=') => (Ok(Token::Assignment { mutable: false }), 2),
                    _ => (
                        Err(LexerError::ExpectedCharacter {
                            expected: b'=' as char,
                            found: bytes.get(1).copied().map(|byte| byte as char),
                        }),
                        2,
                    ),
                },
                b';' => (Ok(Token::SemiColon), 1),
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
