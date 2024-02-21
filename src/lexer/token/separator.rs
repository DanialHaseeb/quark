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
        match stream.next() {
            Some('(') => Left(Parenthesis),
            Some(')') => Right(Parenthesis),
            Some('[') => Left(Bracket),
            Some(']') => Right(Bracket),
            Some('{') => Left(Brace),
            Some('}') => Right(Brace),
            Some(',') => Comma,
            Some('.') => Dot,
            Some(';') => Semicolon,
            _ => unreachable!(),
        }
    }
}
