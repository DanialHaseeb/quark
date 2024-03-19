use self::Keyword::*;
use itertools::PeekNth;
use std::fmt;
use Identifier::*;

#[derive(Debug, PartialEq)]
pub enum Identifier {
    Variable(String),
    Keyword(Keyword),
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
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

impl Identifier {
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

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Identifier::Variable(name) => write!(f, "Variable({})", name),
            Identifier::Keyword(keyword) => write!(f, "{}", keyword),
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::While => write!(f, "While"),
            Keyword::For => write!(f, "For"),
            Keyword::In => write!(f, "In"),
            Keyword::Break => write!(f, "Break"),
            Keyword::Continue => write!(f, "Continue"),
            Keyword::If => write!(f, "If"),
            Keyword::Else => write!(f, "Else"),
            Keyword::True => write!(f, "True"),
            Keyword::False => write!(f, "False"),
            Keyword::Return => write!(f, "Return"),
            Keyword::And => write!(f, "And"),
            Keyword::Or => write!(f, "Or"),
            Keyword::Xor => write!(f, "Xor"),
            Keyword::Not => write!(f, "Not"),
        }
    }
}
