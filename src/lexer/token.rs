use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    Assignment(Assignment),
    Eq,
    Brace { open: bool, kind: BraceKind },
    QuestionMark,
    SemiColon,
    Colon,
    Dot,
    Ampersand,
    Comma,
    Literal { value: String, kind: LiteralKind },
    Keyword(Keyword),
    Arrow,
    Operator(Operator),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, Clone)]
pub enum Keyword {
    Struct,
    Fn,
    For,
    Pub,
    If,
    Else,
    Get,
    As,
}

#[derive(Debug, Clone, Copy)]
pub struct UnknownKeywordError;

impl FromStr for Keyword {
    type Err = UnknownKeywordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "struct" => Ok(Keyword::Struct),
            "fn" => Ok(Keyword::Fn),
            "for" => Ok(Keyword::For),
            "pub" => Ok(Keyword::Pub),
            "if" => Ok(Keyword::If),
            "else" => Ok(Keyword::Else),
            "get" => Ok(Keyword::Get),
            "as" => Ok(Keyword::As),
            _ => Err(UnknownKeywordError),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Assignment {
    Normal,
    Optional,
}

#[derive(Debug, Clone)]
pub enum LiteralKind {
    String,
    Int,
    Float,
}

#[derive(Debug, Clone, Copy)]
pub enum BraceKind {
    Curly,
    Square,
    Smooth,
}
