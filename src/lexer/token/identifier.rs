use self::Keyword::*;
use std::iter::Peekable;
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
    pub fn new<T>(stream: &mut Peekable<T>) -> Self
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
            "in" => Keyword(In),
            "break" => Keyword(Break),
            "continue" => Keyword(Continue),
            "if" => Keyword(If),
            "else" => Keyword(Else),
            "true" => Keyword(True),
            "false" => Keyword(False),
            "return" => Keyword(Return),
            _ => Variable(lexeme),
        }
    }
}
