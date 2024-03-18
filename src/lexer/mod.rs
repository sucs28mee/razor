pub mod token;

use crate::util::{Span, Spanned};
use std::iter::Peekable;
use token::{Assignment, BraceKind, LiteralKind, Operator, Token};

pub fn tokenize<I, B>(bytes: B) -> TokenIter<I>
where
    I: Iterator<Item = u8>,
    B: IntoIterator<IntoIter = I>,
{
    TokenIter {
        bytes: bytes.into_iter().peekable(),
        index: 0,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LexerError {
    UnexpectedCharacter(u8),
    NonUtf8Bytes,
}

pub struct TokenIter<I: Iterator<Item = u8>> {
    bytes: Peekable<I>,
    index: usize,
}

impl<I: Iterator<Item = u8>> TokenIter<I> {
    fn peek_byte(&mut self) -> Option<u8> {
        self.bytes.peek().map(|byte| byte).copied()
    }

    /// Returns the next byte in the byte iterator and adds to the index.
    fn next_byte(&mut self) -> Option<u8> {
        let next = self.bytes.next()?;
        self.index += 1;

        Some(next)
    }

    /// Collects bytes into a [`Vec`] until `f` returns `false`.
    fn collect_bytes<F: FnMut(u8) -> bool>(&mut self, mut vec: Vec<u8>, mut f: F) -> Vec<u8> {
        while let Some(byte) = self.peek_byte() {
            if !f(byte) {
                break;
            }

            _ = self.next_byte();
            vec.push(byte);
        }

        vec
    }

    /// Parses the next [`Token`].
    fn next_token(&mut self, byte: u8) -> Result<Token, LexerError> {
        match byte {
            b'a'..=b'z' | b'A'..=b'Z' => {
                let bytes = self.collect_bytes(
                    vec![byte],
                    |byte| matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'),
                );

                // The bytes here can never be non UTF8 because of the checks above.
                let string = unsafe { String::from_utf8_unchecked(bytes) };

                if let Ok(keyword) = string.parse() {
                    return Ok(Token::Keyword(keyword));
                }

                Ok(Token::Ident(string))
            }
            // TODO: Add support for escape characters.
            b'"' => {
                let bytes = self.collect_bytes(vec![], |byte| byte != b'"');
                // Skip another '"'
                _ = self.next_byte();

                let Ok(value) = String::from_utf8(bytes) else {
                    return Err(LexerError::NonUtf8Bytes);
                };

                Ok(Token::Literal {
                    value,
                    kind: LiteralKind::String,
                })
            }
            b'0'..=b'9' | b'.' | b'-' => {
                // This checks if `byte` or the next byte is numeric.
                // If so it proceeds to parse a number otherwise parses the other possible tokens.
                match (byte, self.peek_byte()) {
                    (_, Some(b'0'..=b'9')) => {}
                    (b'.', _) => {
                        return Ok(Token::Dot);
                    }
                    (b'-', _) => {
                        return if let Some(b'>') = self.peek_byte() {
                            _ = self.next_byte();
                            Ok(Token::Arrow)
                        } else {
                            Ok(Token::Operator(Operator::Minus))
                        };
                    }
                    _ => {}
                }

                let mut dot = byte == b'.';
                let bytes = self.collect_bytes(vec![byte], |byte| match byte {
                    b'0'..=b'9' => true,
                    b'.' => {
                        if dot {
                            return false;
                        }

                        dot = true;
                        true
                    }
                    _ => false,
                });

                // The bytes here can never be non UTF8 because of the checks above.
                let value = unsafe { String::from_utf8_unchecked(bytes) };

                if dot {
                    return Ok(Token::Literal {
                        value,
                        kind: LiteralKind::Float,
                    });
                }

                Ok(Token::Literal {
                    value,
                    kind: LiteralKind::Int,
                })
            }
            b'{' => Ok(Token::Brace {
                open: true,
                kind: BraceKind::Curly,
            }),
            b'}' => Ok(Token::Brace {
                open: false,
                kind: BraceKind::Curly,
            }),
            b'(' => Ok(Token::Brace {
                open: true,
                kind: BraceKind::Smooth,
            }),
            b')' => Ok(Token::Brace {
                open: false,
                kind: BraceKind::Smooth,
            }),
            b'[' => Ok(Token::Brace {
                open: true,
                kind: BraceKind::Square,
            }),

            b']' => Ok(Token::Brace {
                open: false,
                kind: BraceKind::Square,
            }),
            // Other:
            // -------------------------------------@
            b':' => {
                if let Some(b'=') = self.peek_byte() {
                    _ = self.next_byte();
                    Ok(Token::Assignment(Assignment::Normal))
                } else {
                    Ok(Token::Colon)
                }
            }
            b'?' => {
                if let Some(b'=') = self.peek_byte() {
                    _ = self.next_byte();
                    Ok(Token::Assignment(Assignment::Optional))
                } else {
                    Ok(Token::QuestionMark)
                }
            }
            b';' => Ok(Token::SemiColon),
            b'&' => Ok(Token::Ampersand),
            b',' => Ok(Token::Comma),
            b'=' => Ok(Token::Eq),
            b'+' => Ok(Token::Operator(Operator::Plus)),
            b'*' => Ok(Token::Operator(Operator::Star)),
            b'/' => Ok(Token::Operator(Operator::Slash)),
            _ => Err(LexerError::UnexpectedCharacter(byte)),
        }
    }
}

impl<I: Iterator<Item = u8>> Iterator for TokenIter<I> {
    type Item = Result<Spanned<Token>, Spanned<LexerError>>;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip any whitespace.
        while self.peek_byte()?.is_ascii_whitespace() {
            _ = self.next_byte();
        }

        // Skip comments.
        loop {
            if self.peek_byte()? != b'#' {
                break;
            }
            _ = self.next_byte();

            match self.next_byte()? {
                b'!' => loop {
                    if self.next_byte()? != b'!' {
                        continue;
                    }

                    if self.peek_byte()? == b'#' {
                        break;
                    }
                },
                b'\n' => {}
                _ => while self.next_byte()? != b'\n' {},
            }

            // Skip any whitespace after comments.
            while self.peek_byte()?.is_ascii_whitespace() {
                _ = self.next_byte();
            }
        }

        let start = self.index;
        let byte = self.next_byte()?;
        Some(match self.next_token(byte) {
            Ok(token) => Ok(token.span(start..self.index)),
            Err(error) => Err(error.span(start..self.index)),
        })
    }
}
