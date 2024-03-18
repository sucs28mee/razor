pub mod item;

use crate::{
    lexer::token::{Assignment, BraceKind, Keyword, Token},
    util::{Span, Spanned},
};
use item::Item;
use std::iter::Peekable;

use self::item::{Block, FnArg, ItemKind, Ty};

pub fn parse<I, T>(tokens: T) -> ItemIter<I>
where
    I: Iterator<Item = Spanned<Token>>,
    T: IntoIterator<IntoIter = I>,
{
    ItemIter {
        index: 0,
        tokens: tokens.into_iter().peekable(),
    }
}

#[derive(Debug, Clone)]
pub enum ParseError {
    /// Temporary lazy error.
    Lazy(Spanned<String>),
}

pub struct ItemIter<I: Iterator<Item = Spanned<Token>>> {
    index: usize,
    tokens: Peekable<I>,
}

impl<I: Iterator<Item = Spanned<Token>>> ItemIter<I> {
    fn peek_token<'a>(&'a mut self) -> Option<&'a Spanned<Token>> {
        self.tokens.peek()
    }

    fn next_token(&mut self) -> Option<Spanned<Token>> {
        let token = self.tokens.next()?;
        self.index = match self.peek_token() {
            Some(Spanned { start, .. }) => *start,
            None => token.end,
        };

        Some(token)
    }

    fn next_ty(&mut self) -> Result<Ty, ParseError> {
        let ident = match self.next_token() {
            Some(Spanned {
                start,
                end,
                value: Token::Ident(ident),
            }) => ident.span(start..end),
            _ => {
                return Err(ParseError::Lazy(
                    "Expected a type."
                        .to_owned()
                        .span(self.index..self.index + 1),
                ))
            }
        };

        let optional = if matches!(
            self.peek_token(),
            Some(Spanned {
                value: Token::QuestionMark,
                ..
            })
        ) {
            _ = self.next_token();
            true
        } else {
            false
        };

        Ok(Ty { ident, optional })
    }

    fn next_item(&mut self, token: Spanned<Token>) -> Result<Item, ParseError> {
        match token {
            Spanned {
                start,
                end,
                value: Token::Ident(ident),
            } => {
                let ident = ident.span(start..end);
                let Some(Spanned {
                    end,
                    value: Token::Assignment(Assignment::Normal),
                    ..
                }) = self.next_token()
                else {
                    return Err(ParseError::Lazy(
                        "Expected \":=\".".to_owned().span(end..end + 1),
                    ));
                };

                let Some(Spanned {
                    end,
                    value: Token::Keyword(Keyword::Fn),
                    ..
                }) = self.next_token()
                else {
                    return Err(ParseError::Lazy(
                        "Expected \"fn\".".to_owned().span(end..end + 1),
                    ));
                };

                let Some(Spanned {
                    end,
                    value:
                        Token::Brace {
                            open: true,
                            kind: BraceKind::Smooth,
                        },
                    ..
                }) = self.next_token()
                else {
                    return Err(ParseError::Lazy(
                        "Expected \"(\".".to_owned().span(end..end + 1),
                    ));
                };

                // Parse fn arguments
                // --------------------------------------------@
                let mut args = Vec::new();
                loop {
                    if let Some(Spanned {
                        value:
                            Token::Brace {
                                open: false,
                                kind: BraceKind::Smooth,
                            },
                        ..
                    }) = self.peek_token()
                    {
                        break;
                    }

                    let ident = match self.next_token() {
                        Some(Spanned {
                            start,
                            end,
                            value: Token::Ident(ident),
                        }) => {
                            if !args.is_empty() {
                                return Err(ParseError::Lazy(
                                    "Expected \",\".".to_owned().span(start..end),
                                ));
                            }

                            ident.span(start..end)
                        }
                        Some(Spanned {
                            start,
                            end,
                            value: Token::Comma,
                        }) => {
                            if args.is_empty() {
                                return Err(ParseError::Lazy(
                                    "Expected identifier or \")\".".to_owned().span(start..end),
                                ));
                            }

                            match self.next_token() {
                                Some(Spanned {
                                    start,
                                    end,
                                    value: Token::Ident(ident),
                                }) => ident.span(start..end),
                                Some(Spanned {
                                    value:
                                        Token::Brace {
                                            open: false,
                                            kind: BraceKind::Smooth,
                                        },
                                    ..
                                }) => break,
                                _ => {
                                    return Err(ParseError::Lazy(
                                        "Expected identifier or \")\"."
                                            .to_owned()
                                            .span(end..end + 1),
                                    ))
                                }
                            }
                        }
                        _ => {
                            return Err(ParseError::Lazy(
                                "Expected identifier or \")\"."
                                    .to_owned()
                                    .span(end..end + 1),
                            ))
                        }
                    };

                    let Some(Spanned {
                        value: Token::Colon,
                        ..
                    }) = self.next_token()
                    else {
                        return Err(ParseError::Lazy(
                            "Expected \":\".".to_owned().span(end..end + 1),
                        ));
                    };

                    args.push(FnArg {
                        ident,
                        ty: self.next_ty()?,
                    })
                }

                let Some(Spanned {
                    end,
                    value:
                        Token::Brace {
                            open: false,
                            kind: BraceKind::Smooth,
                        },
                    ..
                }) = self.next_token()
                else {
                    return Err(ParseError::Lazy(
                        "Expected identifier or \")\"."
                            .to_owned()
                            .span(end..end + 1),
                    ));
                };

                let ty = if let Some(Spanned {
                    value: Token::Arrow,
                    ..
                }) = self.peek_token()
                {
                    _ = self.next_token();
                    Some(self.next_ty()?)
                } else {
                    None
                };

                let Some(Spanned {
                    end,
                    value:
                        Token::Brace {
                            open: true,
                            kind: BraceKind::Curly,
                        },
                    ..
                }) = self.next_token()
                else {
                    return Err(ParseError::Lazy(
                        "Expected \"{\".".to_owned().span(end..end + 1),
                    ));
                };

                loop {
                    if let Some(Spanned {
                        value:
                            Token::Brace {
                                open: false,
                                kind: BraceKind::Curly,
                            },
                        ..
                    }) = self.peek_token()
                    {
                        break;
                    }

                    // TODO: Make this actually parse statements.
                    // Right now it just ignores everything...
                    self.next_token();
                }

                let Some(Spanned {
                    value:
                        Token::Brace {
                            open: false,
                            kind: BraceKind::Curly,
                        },
                    ..
                }) = self.next_token()
                else {
                    return Err(ParseError::Lazy(
                        "Expected a statement or \"}\"."
                            .to_owned()
                            .span(end..end + 1),
                    ));
                };

                Ok(Item {
                    ident,
                    kind: ItemKind::Fn {
                        args,
                        block: Block {
                            statements: Vec::new(),
                            trailing_expr: None,
                        },
                        ty,
                    },
                })
            }
            Spanned { start, end, .. } => Err(ParseError::Lazy(
                "Expected an identifier.".to_owned().span(start..end),
            )),
        }
    }
}

impl<I: Iterator<Item = Spanned<Token>>> Iterator for ItemIter<I> {
    type Item = Result<Spanned<Item>, Spanned<ParseError>>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.index;
        let token = self.next_token()?;
        Some(match self.next_item(token) {
            Ok(item) => Ok(item.span(start..self.index)),
            Err(error) => Err(error.span(start..self.index)),
        })
    }
}
