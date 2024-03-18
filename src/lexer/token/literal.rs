use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(Number),
    String(QuarkString),
}

#[derive(Debug, PartialEq)]
pub struct QuarkString(String);

#[derive(Debug, PartialEq)]
pub enum Number {
    Natural(u64),
    Integer(i64),
    Real(f64),
}

impl Literal {
    pub fn new<T>(stream: &mut Peekable<T>) -> Self
    where
        T: Iterator<Item = char>,
    {
        match stream.peek() {
            Some(&'"') => Self::String(QuarkString::new(stream)),
            Some(&symbol) if symbol.is_numeric() => Self::Number(Number::new(stream)),
            _ => unreachable!(),
        }
    }
}

impl QuarkString {
    fn new<T>(stream: &mut Peekable<T>) -> Self
    where
        T: Iterator<Item = char>,
    {
        let mut string = String::new();
        stream.next(); // consume the opening quote

        while let Some(&symbol) = stream.peek() {
            match symbol {
                '"' => {
                    stream.next();
                    break;
                }
                _ => string.push(stream.next().unwrap()),
            }
        }
        QuarkString(string)
    }
}

impl Number {
    pub fn new<T>(stream: &mut Peekable<T>) -> Self
    where
        T: Iterator<Item = char>,
    {
        let mut number = String::new();

        while let Some(&symbol) = stream.peek() {
            match symbol {
                _ if symbol.is_numeric() => number.push(stream.next().unwrap()),
                _ => break,
            }
        }

        match number.parse::<u64>() {
            Ok(natural) => Self::Natural(natural),
            Err(_) => match number.parse::<i64>() {
                Ok(integer) => Self::Integer(integer),
                Err(_) => Self::Real(number.parse::<f64>().unwrap()),
            },
        }
    }
}
