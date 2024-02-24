use std::iter::Peekable;
use DoubleCharOperator::*;
use Operator::*;
use SingleCharOperator::*;

#[derive(Debug, PartialEq)]
pub enum Operator {
    SingleChar(SingleCharOperator),
    DoubleChar(DoubleCharOperator),
}

#[derive(Debug, PartialEq)]
pub enum SingleCharOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Percent,
    Equal,
    Less,
    Greater,
    Ampersand,
    Pipe,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum DoubleCharOperator {
    PlusEquals,
    MinusEquals,
    MultiplyEquals,
    NotEquals,
    DividEquals,
    ModEquals,
    EqualEquals,
}

pub trait MatchTrait<T>
where
    T: Iterator<Item = char>,
{
    fn next_if_matches(&mut self, c: char) -> bool;
}

impl<T> MatchTrait<T> for Peekable<T>
where
    T: Iterator<Item = char>,
{
    fn next_if_matches(&mut self, c: char) -> bool {
        if let Some(&next_char) = self.peek() {
            if next_char == c {
                self.next();
                return true;
            }
        }
        false
    }
}

impl Operator {
    pub fn new<T>(stream: &mut Peekable<T>) -> Self
    where
        T: Iterator<Item = char>,
    {
        match stream.next().unwrap() {
            '+' if stream.next_if_matches('=') => DoubleChar(PlusEquals),
            '-' if stream.next_if_matches('=') => DoubleChar(MinusEquals),
            '*' if stream.next_if_matches('=') => DoubleChar(MultiplyEquals),
            '!' if stream.next_if_matches('=') => DoubleChar(NotEquals),
            '/' if stream.next_if_matches('=') => DoubleChar(DividEquals),
            '%' if stream.next_if_matches('=') => DoubleChar(ModEquals),
            '=' if stream.next_if_matches('=') => DoubleChar(EqualEquals),

            '+' => SingleChar(Plus),
            '-' => SingleChar(Minus),
            '*' => SingleChar(Multiply),
            '!' => SingleChar(Not),
            '/' => SingleChar(Divide),
            '%' => SingleChar(Percent),
            '=' => SingleChar(Equal),

            '<' => SingleChar(Less),
            '>' => SingleChar(Greater),
            '&' => SingleChar(Ampersand),
            '|' => SingleChar(Pipe),
            _ => unreachable!(),
        }
    }
}
