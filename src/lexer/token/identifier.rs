use self::Keyword::*;
use itertools::PeekNth;
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
