use std::{iter::Peekable, ops::Deref};

use crate::{
    lexer::{AssignmentKind, BraceKind, KeywordKind, Token},
    util::{Span, Spanned},
};

#[derive(Debug, Clone)]
pub struct Item {
    public: bool,
    ident: Span<String>,
    kind: ItemKind,
}

#[derive(Debug, Clone)]
pub struct ParseError(String);

impl Item {
    /// Returns the parsed [`Item`] and a length of consumed tokens.
    fn parse_next<I: Iterator<Item = Span<Token>>>(
        first: Span<Token>,
        tokens: &mut Peekable<I>,
    ) -> Result<Span<Self>, Span<ParseError>> {
        let (start, end) = (first.start(), first.end());
        match first.value {
            Token::Ident(ident) => {
                let token = match tokens.next() {
                    Some(token) if matches!(*token, Token::Assignment(AssignmentKind::Normal)) => {
                        token
                    }
                    Some(token) => todo!(),
                    None => todo!(),
                };

                match tokens.next() {
                    Some(token) if matches!(*token, Token::Keyword(KeywordKind::Fn)) => {
                        let token = match tokens.next() {
                            Some(token)
                                if matches!(
                                    *token,
                                    Token::Brace {
                                        open: true,
                                        kind: BraceKind::Smooth
                                    }
                                ) =>
                            {
                                token
                            }
                            Some(token) => todo!(),
                            None => todo!(),
                        };

                        let mut args = Vec::new();
                        while let Some(mut token) = tokens.next_if(|token| {
                            !matches!(
                                **token,
                                Token::Brace {
                                    open: false,
                                    kind: BraceKind::Smooth
                                }
                            )
                        }) {
                            // If some args were already parsed we need to check if there is a comma.
                            if !args.is_empty() {
                                match &*token {
                                    Token::Comma => {
                                        token = match tokens.next() {
                                            Some(token) => token,
                                            None => todo!(),
                                        };
                                    }
                                    token => todo!(),
                                }
                            }

                            let (start, len) = (token.start(), token.len());
                            let ident = match token.value {
                                Token::Ident(ident) => ident.spanned(start, len),
                                token => todo!(),
                            };

                            let token = match tokens.next() {
                                Some(token)
                                    if matches!(
                                        *token,
                                        Token::Brace {
                                            open: true,
                                            kind: BraceKind::Smooth
                                        }
                                    ) =>
                                {
                                    token
                                }
                                Some(token) => todo!(),
                                None => todo!(),
                            };

                            let ty = match tokens.next() {
                                Some(token) => {
                                    let (start, len) = (token.start(), token.len());
                                    match token.value {
                                        Token::Ident(ident) => ident.spanned(start, len),
                                        _ => todo!(),
                                    }
                                }
                                None => todo!(),
                            };

                            args.push(Arg { ident, ty })
                        }

                        let last = match tokens.next() {
                            Some(token) if matches!(*token, Token::Comma) => {
                                let Some(token) = tokens.next_if(|token| {
                                    matches!(
                                        **token,
                                        Token::Brace {
                                            open: false,
                                            kind: BraceKind::Smooth
                                        }
                                    )
                                }) else {
                                    todo!()
                                };

                                token
                            }
                            Some(token)
                                if matches!(
                                    *token,
                                    Token::Brace {
                                        open: false,
                                        kind: BraceKind::Smooth
                                    }
                                ) =>
                            {
                                token
                            }
                            Some(token) => todo!(),
                            None => todo!(),
                        };

                        Ok(Item {
                            public: false,
                            ident: ident.spanned(start, end - start),
                            kind: ItemKind::Fn {
                                args,
                                return_ty: None,
                            },
                        }
                        .spanned(start, last.end() - start))
                    }
                    Some(token) if matches!(*token, Token::Keyword(KeywordKind::Struct)) => todo!(),
                    Some(token) => todo!(),
                    None => todo!(),
                }
            }
            // TODO: "pub" modifier
            Token::Keyword(_) => todo!(),
            _ => Err(
                ParseError(format!("Expected \"pub\" or an identifier found {first:?}"))
                    .spanned(first.start(), first.len()),
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    Struct {
        fields: Vec<Field>,
    },
    Fn {
        args: Vec<Arg>,
        return_ty: Option<Span<String>>,
    },
}

#[derive(Debug, Clone)]
pub enum FieldModifier {
    Pub,
    PubGet,
}

#[derive(Debug, Clone)]
pub struct Field {
    modifier: Option<FieldModifier>,
    ident: Span<String>,
    ty: Span<String>,
}

#[derive(Debug, Clone)]
pub struct Arg {
    ident: Span<String>,
    ty: Span<String>,
}

enum Expr {}

pub struct Parser<I: Iterator> {
    tokens: Peekable<I>,
}

impl<I: Iterator<Item = Span<Token>>> Parser<I> {
    pub fn new<T: IntoIterator<IntoIter = I>>(tokens: T) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }
}

impl<I: Iterator<Item = Span<Token>>> Iterator for Parser<I> {
    type Item = Result<Span<Item>, Span<ParseError>>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Item::parse_next(self.tokens.next()?, &mut self.tokens))
    }
}
