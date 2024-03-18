use crate::{
    expr_tree::ExprTree,
    lexer::token::{LiteralKind, Operator},
    util::Spanned,
};

#[derive(Debug, Clone)]
pub struct Item {
    pub ident: Spanned<String>,
    pub kind: ItemKind,
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    Fn {
        args: Vec<FnArg>,
        ty: Option<Spanned<String>>,
        block: Block,
    },
}

#[derive(Debug, Clone)]
pub struct FnArg {
    pub ident: Spanned<String>,
    pub ty: Spanned<String>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub trailing_expr: Option<ExprTree<Value, Operator>>,
}

#[derive(Debug, Clone)]
pub enum Statement {}

#[derive(Debug, Clone)]
pub enum Value {
    Ident(String),
    Literal { value: String, kind: LiteralKind },
}
