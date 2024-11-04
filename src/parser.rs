use crate::lexer::{Lexem, Paired};

pub enum ValidateResult<T> {
    Take,
    InsertBefore(T),
    Drop,
    DropBefore,
}

pub struct Token {
    pub lexem: Lexem,
    pub whitespace_before: String,
}

impl Token {
    pub fn new(lexem: Lexem, whitespace_before: impl Into<String>) -> Self {
        Self {
            lexem,
            whitespace_before: whitespace_before.into(),
        }
    }
}

impl From<Lexem> for Token {
    fn from(value: Lexem) -> Self {
        Self::new(value, String::new())
    }
}

impl Default for Token {
    fn default() -> Self {
        Lexem::Open(Paired::File).into()
    }
}

impl From<&Token> for String {
    fn from(value: &Token) -> Self {
        value.whitespace_before.clone() + &Into::<String>::into(&value.lexem)
    }
}

pub fn validate(previous: &Lexem, lexem: &Lexem) -> ValidateResult<Lexem> {
    match (previous, lexem) {
        (Lexem::Comma, Lexem::Close(_)) => ValidateResult::DropBefore,
        (Lexem::Colon, Lexem::Close(_)) => ValidateResult::DropBefore,
        (Lexem::Open(_), Lexem::Colon) => ValidateResult::Drop,
        (Lexem::Open(_), Lexem::Comma) => ValidateResult::Drop,
        (Lexem::Colon, Lexem::Colon) => ValidateResult::Drop,
        (Lexem::Comma, Lexem::Comma) => ValidateResult::Drop,
        (Lexem::Colon, Lexem::Comma) => ValidateResult::DropBefore,
        (Lexem::Comma, Lexem::Colon) => ValidateResult::Drop,
        (Lexem::Close(_), Lexem::Close(_)) => ValidateResult::Take,
        (Lexem::Close(_), Lexem::Comma) => ValidateResult::Take,
        (Lexem::Close(_), Lexem::Colon) => ValidateResult::Drop,
        (Lexem::Close(_), _) => ValidateResult::InsertBefore(Lexem::Comma),
        (Lexem::Colon, Lexem::Open(_)) => ValidateResult::Take,
        (Lexem::Comma, Lexem::Open(_)) => ValidateResult::Take,
        (Lexem::Open(_), Lexem::Open(_)) => ValidateResult::Take,
        (_, Lexem::Open(_)) => ValidateResult::InsertBefore(Lexem::Comma),
        (Lexem::String(_), Lexem::String(_)) => ValidateResult::InsertBefore(Lexem::Comma),
        (Lexem::Else(_), Lexem::String(_)) => ValidateResult::InsertBefore(Lexem::Comma),
        (Lexem::String(_), Lexem::Else(_)) => ValidateResult::InsertBefore(Lexem::Comma),
        (Lexem::Else(_), Lexem::Else(_)) => ValidateResult::InsertBefore(Lexem::Comma),
        _ => ValidateResult::Take,
    }
}