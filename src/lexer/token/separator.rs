use std::fmt;
use Delimiter::*;
use Separator::*;

#[derive(Debug, PartialEq)]
pub enum Separator {
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

impl Separator {
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

impl fmt::Display for Separator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Separator::Left(delimiter) => write!(f, "Left({})", delimiter),
            Separator::Right(delimiter) => write!(f, "Right({})", delimiter),
            Separator::Comma => write!(f, "Comma"),
            Separator::Dot => write!(f, "Dot"),
            Separator::Semicolon => write!(f, "Semicolon"),
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
