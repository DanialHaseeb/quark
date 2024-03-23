use std::fmt;
use Delimiter::*;
use SeparatorKind::*;

#[derive(Debug, PartialEq)]
pub enum SeparatorKind {
    Left(Delimiter),
    Right(Delimiter),
    Comma,
    Dot,
    Semicolon,
}

#[derive(Debug, PartialEq)]
pub enum Delimiter {
    Parenthesis,
    Bracket,
    Brace,
}

impl SeparatorKind {
    pub fn new<T>(stream: &mut T) -> Self
    where
        T: Iterator<Item = char>,
    {
        match stream.next().unwrap() {
            '(' => Left(Parenthesis),
            ')' => Right(Parenthesis),
            '[' => Left(Bracket),
            ']' => Right(Bracket),
            '{' => Left(Brace),
            '}' => Right(Brace),
            ',' => Comma,
            '.' => Dot,
            ';' => Semicolon,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for SeparatorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SeparatorKind::Left(delimiter) => write!(f, "Left({})", delimiter),
            SeparatorKind::Right(delimiter) => write!(f, "Right({})", delimiter),
            SeparatorKind::Comma => write!(f, ","),
            SeparatorKind::Dot => write!(f, "."),
            SeparatorKind::Semicolon => write!(f, ";"),
        }
    }
}

impl fmt::Display for Delimiter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Delimiter::Parenthesis => write!(f, "Parenthesis"),
            Delimiter::Bracket => write!(f, "Bracket"),
            Delimiter::Brace => write!(f, "Brace"),
        }
    }
}
