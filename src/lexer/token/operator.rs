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
    Asterisk,
    Slash,
    Percent,
    Hash,
    Equal,
    Less,
    Greater,
    Ampersand,
    Pipe,
    Bang,
}

#[derive(Debug, PartialEq)]
pub enum DoubleCharOperator {
    PlusEqual,
    MinusEqual,
    AsteriskEqual,
    BangEqual,
    SlashEqual,
    PercentEqual,
    EqualEqual,
    SlashSlash,
}

pub trait Match<T>
where
    T: Iterator<Item = char>,
{
    fn next_if_matches(&mut self, c: char) -> bool;
}

impl<T> Match<T> for Peekable<T>
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
            '+' if stream.next_if_matches('=') => DoubleChar(PlusEqual),
            '-' if stream.next_if_matches('=') => DoubleChar(MinusEqual),
            '*' if stream.next_if_matches('=') => DoubleChar(AsteriskEqual),
            '!' if stream.next_if_matches('=') => DoubleChar(BangEqual),
            '/' if stream.next_if_matches('=') => DoubleChar(SlashEqual),
            '%' if stream.next_if_matches('=') => DoubleChar(PercentEqual),
            '=' if stream.next_if_matches('=') => DoubleChar(EqualEqual),
            '/' if stream.next_if_matches('/') => DoubleChar(SlashSlash),

            '+' => SingleChar(Plus),
            '-' => SingleChar(Minus),
            '#' => SingleChar(Hash),
            '*' => SingleChar(Asterisk),
            '!' => SingleChar(Bang),
            '/' => SingleChar(Slash),
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
