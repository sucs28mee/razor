use std::{iter::Peekable, ops::Deref};

use crate::{
    expr_tree::ExprTree,
    lexer::{AssignmentKind, BraceKind, Keyword, LiteralKind, Token},
    util::{Span, Spanned},
};

#[derive(Debug, Clone)]
pub struct Item {
    public: bool,
    ident: Span<String>,
    kind: ItemKind,
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    Struct {
        fields: Vec<Field>,
    },
    Fn {
        args: Vec<Arg>,
        return_ty: Option<Span<String>>,
        expr: Expr,
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

#[derive(Debug, Clone)]
pub struct Expr {
    statements: Vec<Statement>,
    trailing_expr: ExprTree<Value, BinaryOperator>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableInit {
        ident: Span<String>,
        expr: ExprTree<Value, BinaryOperator>,
    },
    Expr(ExprTree<Value, BinaryOperator>),
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
}

impl AsRef<str> for BinaryOperator {
    fn as_ref(&self) -> &str {
        match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Sub => "-",
            BinaryOperator::Mul => "*",
            BinaryOperator::Div => "/",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Literal { value: String, kind: LiteralKind },
    Void,
    None,
}

#[derive(Debug, Clone)]
pub struct ParseError(String);

impl Item {
    /// Returns the parsed [`Item`] and a length of consumed tokens.
    fn parse_next<I: Iterator<Item = Span<Token>>>(
        first: Span<Token>,
        tokens: &mut Peekable<I>,
    ) -> Result<Span<Self>, Span<ParseError>> {
        fn parse_expr<I: Iterator<Item = Span<Token>>>(
            first: Span<Token>,
            tokens: &mut Peekable<I>,
        ) -> Result<Span<Expr>, Span<ParseError>> {
            let statements = Vec::new();
            loop {
                let (start, len) = (first.start(), first.len());
                match first.value {
                    Token::Ident(_) => {}
                    Token::Brace { open, kind } => todo!(),
                    Token::QuestionMark => {
                        return Ok(Expr {
                            statements,
                            trailing_expr: ExprTree::Value(Value::None),
                        }
                        .spanned(start, len))
                    }
                    Token::Colon => todo!(),
                    Token::Dot => todo!(),
                    Token::Ampersand => todo!(),
                    Token::Comma => todo!(),
                    Token::Literal { value, kind } => {
                        return Ok(Expr {
                            statements,
                            trailing_expr: ExprTree::Value(Value::Literal { value, kind }),
                        }
                        .spanned(start, len))
                    }
                    Token::Keyword(_) => todo!(),
                    token => todo!(),
                }
            }
        }

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
                    Some(token) if matches!(*token, Token::Keyword(Keyword::Fn)) => {
                        match tokens.next() {
                            Some(token)
                                if matches!(
                                    *token,
                                    Token::Brace {
                                        open: true,
                                        kind: BraceKind::Smooth
                                    }
                                ) => {}
                            Some(token) => {
                                return Err(ParseError(format!(
                                    "Expected a \"{{\" found {token:?}"
                                ))
                                .spanned(token.start(), token.len()))
                            }
                            None => {
                                return Err(ParseError(format!("Expected a \"{{\" found nothing."))
                                    .spanned(token.start(), token.len()))
                            }
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
                                            None => {
                                                return Err(ParseError(format!(
                                                    "Expected an identifier found nothing."
                                                ))
                                                .spanned(token.start(), token.len()))
                                            }
                                        };
                                    }
                                    _ => {
                                        return Err(ParseError(format!(
                                            "Expected a \",\" found {token:?}"
                                        ))
                                        .spanned(token.start(), token.len()))
                                    }
                                }
                            }

                            let (start, len) = (token.start(), token.len());
                            let ident = match token.value {
                                Token::Ident(ident) => ident.spanned(start, len),
                                _ => {
                                    return Err(ParseError(format!(
                                        "Expected an identifier found {token:?}."
                                    ))
                                    .spanned(token.start(), token.len()))
                                }
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
                                Some(token) => {
                                    return Err(ParseError(format!(
                                        "Expected a \"(\" found {token:?}"
                                    ))
                                    .spanned(token.start(), token.len()))
                                }
                                None => {
                                    return Err(ParseError(format!(
                                        "Expected a \"(\" found nothing"
                                    ))
                                    .spanned(start, len))
                                }
                            };

                            let ty = match tokens.next() {
                                Some(token) => {
                                    let (start, len) = (token.start(), token.len());
                                    match token.value {
                                        Token::Ident(ident) => ident.spanned(start, len),
                                        _ => {
                                            return Err(ParseError(format!(
                                                "Expected a type found {token:?}"
                                            ))
                                            .spanned(token.start(), token.len()))
                                        }
                                    }
                                }
                                None => {
                                    return Err(ParseError(format!(
                                        "Expected a type found nothing"
                                    ))
                                    .spanned(start, len))
                                }
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
                                    return Err(ParseError(format!(
                                        "Expected a \")\" found nothing"
                                    ))
                                    .spanned(token.start(), token.len()));
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
                            Some(token) => {
                                return Err(ParseError(format!("Expected a \"(\" found {token:?}"))
                                    .spanned(token.start(), token.len()))
                            }
                            None => {
                                return Err(ParseError(format!("Expected a \")\" found nothing"))
                                    .spanned(token.start(), token.len()))
                            }
                        };

                        Ok(Item {
                            public: false,
                            ident: ident.spanned(start, end - start),
                            kind: ItemKind::Fn {
                                args,
                                return_ty: None,
                                expr: todo!(),
                            },
                        }
                        .spanned(start, last.end() - start))
                    }
                    Some(token) if matches!(*token, Token::Keyword(Keyword::Struct)) => todo!(),
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
