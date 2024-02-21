use Operator::*;

#[derive(Debug, PartialEq)]
pub enum Operator {
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
    Bang,
}

impl Operator {
    pub fn new<T>(stream: &mut T) -> Self
    where
        T: Iterator<Item = char>,
    {
        // TODO: implement two-character operators like '==' and '!='
        match stream.next().unwrap() {
            '+' => Plus,
            '-' => Minus,
            '*' => Multiply,
            '/' => Divide,
            '%' => Percent,
            '=' => Equal,
            '<' => Less,
            '>' => Greater,
            '&' => Ampersand,
            '|' => Pipe,
            '!' => Bang,
            _ => unreachable!(),
        }
    }
}
