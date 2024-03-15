use crate::util::Spanned;

#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    Assignment { mutable: bool },
    Brace { open: bool, kind: BraceKind },
    SemiColon,
    Literal(Literal),
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Int(i32),
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
    IntParseError,
    NonUtf8Bytes,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Spanned<Token>, Spanned<LexerError>>;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip any whitespace.
        while self.bytes.first()?.is_ascii_whitespace() {
            self.bytes = &self.bytes[1..];
        }

        // The starting index of the next token.
        let index = self.len - self.bytes.len();
        let token = match self.bytes.first()? {
            // Identifiers and keywords:
            // --------------------------@
            b'a'..=b'z' | b'A'..=b'Z' => {
                let bytes = (0..)
                    .map_while(|i| {
                        self.bytes
                            .get(i)
                            .filter(|byte| matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'))
                            .copied()
                    })
                    .collect::<Vec<_>>();

                let len = bytes.len();
                if let Ok(string) = String::from_utf8(bytes) {
                    Ok(Spanned::new(index, len, Token::Ident(string)))
                } else {
                    Err(Spanned::new(index, len, LexerError::NonUtf8Bytes))
                }
            }
            // Literals:
            // --------@
            b'"' => {
                // TODO: Add support for escape characters.
                let bytes = (1..)
                    .map_while(|i| self.bytes.get(i).filter(|byte| **byte != b'"').copied())
                    .collect::<Vec<_>>();

                let len = bytes.len() + 2;
                if let Ok(string) = String::from_utf8(bytes) {
                    Ok(Spanned::new(
                        index,
                        len,
                        Token::Literal(Literal::String(string)),
                    ))
                } else {
                    Err(Spanned::new(index, len, LexerError::NonUtf8Bytes))
                }
            }
            b'0'..=b'9' => {
                let bytes = (0..)
                    .map_while(|i| {
                        self.bytes
                            .get(i)
                            .filter(|byte| matches!(byte, b'0'..=b'9'))
                            .copied()
                    })
                    .collect::<Vec<_>>();

                let len = bytes.len();
                if let Ok(string) = String::from_utf8(bytes) {
                    if let Ok(int) = string.parse() {
                        Ok(Spanned::new(index, len, Token::Literal(Literal::Int(int))))
                    } else {
                        Err(Spanned::new(index, len, LexerError::IntParseError))
                    }
                } else {
                    Err(Spanned::new(index, len, LexerError::NonUtf8Bytes))
                }
            }
            // Braces:
            //---------------------@
            b'{' => Ok(Spanned::one(
                index,
                Token::Brace {
                    open: false,
                    kind: BraceKind::Curly,
                },
            )),
            b'}' => Ok(Spanned::one(
                index,
                Token::Brace {
                    open: false,
                    kind: BraceKind::Curly,
                },
            )),
            b'(' => Ok(Spanned::one(
                index,
                Token::Brace {
                    open: true,
                    kind: BraceKind::Smooth,
                },
            )),
            b')' => Ok(Spanned::one(
                index,
                Token::Brace {
                    open: false,
                    kind: BraceKind::Smooth,
                },
            )),
            b'[' => Ok(Spanned::one(
                index,
                Token::Brace {
                    open: true,
                    kind: BraceKind::Square,
                },
            )),
            b']' => Ok(Spanned::one(
                index,
                Token::Brace {
                    open: false,
                    kind: BraceKind::Square,
                },
            )),
            // Other:
            // -------------------------------------@
            b':' => match self.bytes.get(1) {
                Some(b'=') => Ok(Spanned::new(index, 2, Token::Assignment { mutable: false })),
                _ => Err(Spanned::one(
                    index + 1,
                    LexerError::ExpectedCharacter {
                        expected: b'=' as char,
                        found: self.bytes.get(1).copied().map(|byte| byte as char),
                    },
                )),
            },
            b';' => Ok(Spanned::one(index, Token::SemiColon)),
            byte => Err(Spanned::one(
                index,
                LexerError::UnexpectedCharacter(*byte as char),
            )),
        };

        if let Ok(Spanned { len, .. }) = token {
            // Skip the parsed token.
            self.bytes = &self.bytes[len.min(self.bytes.len())..];
        }

        Some(token)
    }
}
