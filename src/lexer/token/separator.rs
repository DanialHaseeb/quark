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
