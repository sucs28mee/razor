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
        ty: Option<Ty>,
        block: Block,
    },
}

#[derive(Debug, Clone)]
pub struct Ty {
    pub ident: Spanned<String>,
    pub optional: bool,
}

#[derive(Debug, Clone)]
pub struct FnArg {
    pub ident: Spanned<String>,
    pub ty: Ty,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub trailing_expr: Option<ExprTree<Value, Operator>>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableInit {
        ident: Spanned<String>,
        expr: ExprTree<Value, Operator>,
    },
}

#[derive(Debug, Clone)]
pub enum Value {
    Ident(Spanned<String>),
    Literal {
        value: Spanned<String>,
        kind: LiteralKind,
    },
}
