use itertools::PeekNth;
use std::fmt;
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

impl<T> Match<T> for PeekNth<T>
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
    pub fn new<T>(stream: &mut PeekNth<T>) -> Self
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

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::SingleChar(op) => write!(f, "{}", op),
            Operator::DoubleChar(op) => write!(f, "{}", op),
        }
    }
}

impl fmt::Display for SingleCharOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            SingleCharOperator::Plus => "+",
            SingleCharOperator::Minus => "-",
            SingleCharOperator::Asterisk => "*",
            SingleCharOperator::Slash => "/",
            SingleCharOperator::Percent => "%",
            SingleCharOperator::Hash => "#",
            SingleCharOperator::Equal => "=",
            SingleCharOperator::Less => "<",
            SingleCharOperator::Greater => ">",
            SingleCharOperator::Ampersand => "&",
            SingleCharOperator::Pipe => "|",
            SingleCharOperator::Bang => "!",
        };
        write!(f, "{}", symbol)
    }
}

impl fmt::Display for DoubleCharOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            DoubleCharOperator::PlusEqual => "+=",
            DoubleCharOperator::MinusEqual => "-=",
            DoubleCharOperator::AsteriskEqual => "*=",
            DoubleCharOperator::BangEqual => "!=",
            DoubleCharOperator::SlashEqual => "/=",
            DoubleCharOperator::PercentEqual => "%=",
            DoubleCharOperator::EqualEqual => "==",
            DoubleCharOperator::SlashSlash => "//",
        };

        write!(f, "{}", symbol)
    }
}
