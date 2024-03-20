use itertools::PeekNth;
use std::fmt;
use IdentifierKind::*;
use KeywordKind::*;

#[derive(Debug, PartialEq, Clone)]
pub enum IdentifierKind {
    Variable(String),
    Keyword(KeywordKind),
}

#[derive(Debug, PartialEq, Clone)]
pub enum KeywordKind {
    While,
    For,
    In,
    Break,
    Continue,
    If,
    Else,
    True,
    False,
    Return,
    And,
    Or,
    Xor,
    Not,
}

pub trait IdentifierSymbol {
    fn is_identifier_symbol(&self) -> bool;
}

impl IdentifierSymbol for char {
    fn is_identifier_symbol(&self) -> bool {
        self.is_alphabetic() || *self == '_'
    }
}

impl IdentifierKind {
    pub fn new<T>(stream: &mut PeekNth<T>) -> Self
    where
        T: Iterator<Item = char>,
    {
        let mut lexeme = String::new();

        while let Some(&symbol) = stream.peek() {
            if symbol.is_identifier_symbol() {
                lexeme.push(symbol);
                stream.next();
            } else {
                break;
            }
        }

        match lexeme.as_str() {
            "while" => Keyword(While),
            "for" => Keyword(For),
            "break" => Keyword(Break),
            "continue" => Keyword(Continue),
            "in" => Keyword(In),
            "if" => Keyword(If),
            "else" => Keyword(Else),
            "true" => Keyword(True),
            "false" => Keyword(False),
            "return" => Keyword(Return),
            "and" => Keyword(And),
            "or" => Keyword(Or),
            "xor" => Keyword(Xor),
            "not" => Keyword(Not),
            _ => Variable(lexeme),
        }
    }
}

impl fmt::Display for IdentifierKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdentifierKind::Variable(name) => write!(f, "Variable({})", name),
            IdentifierKind::Keyword(keyword) => write!(f, "{}", keyword),
        }
    }
}

impl fmt::Display for KeywordKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeywordKind::While => write!(f, "While"),
            KeywordKind::For => write!(f, "For"),
            KeywordKind::In => write!(f, "In"),
            KeywordKind::Break => write!(f, "Break"),
            KeywordKind::Continue => write!(f, "Continue"),
            KeywordKind::If => write!(f, "If"),
            KeywordKind::Else => write!(f, "Else"),
            KeywordKind::True => write!(f, "True"),
            KeywordKind::False => write!(f, "False"),
            KeywordKind::Return => write!(f, "Return"),
            KeywordKind::And => write!(f, "And"),
            KeywordKind::Or => write!(f, "Or"),
            KeywordKind::Xor => write!(f, "Xor"),
            KeywordKind::Not => write!(f, "Not"),
        }
    }
}
