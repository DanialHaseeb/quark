use self::Number::*;
use std::iter::Peekable;
use std::string::String;
use Literal::*;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(Number),
    String(String),
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Natural(u64),
    Integer(i64),
    Real(f64),
}

trait Lex {
    fn lex<T>(stream: &mut Peekable<T>) -> Self
    where
        T: Iterator<Item = char>;
}

impl Lex for String {
    fn lex<T>(stream: &mut Peekable<T>) -> Self
    where
        T: Iterator<Item = char>,
    {
        let mut string = String::new();
        stream.next();

        while let Some(&symbol) = stream.peek() {
            match symbol {
                '"' => {
                    stream.next();
                    break;
                }
                _ => string.push(stream.next().unwrap()),
            }
        }

        string
    }
}

impl Literal {
    pub fn new<T>(stream: &mut Peekable<T>) -> Self
    where
        T: Iterator<Item = char>,
    {
        match stream.peek() {
            Some(&'"') => String(String::lex(stream)),
            Some(&symbol) if symbol.is_numeric() => Number(Number::new(stream)),
            _ => unreachable!(),
        }
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
            Ok(natural) => Natural(natural),
            Err(_) => match number.parse::<i64>() {
                Ok(integer) => Integer(integer),
                Err(_) => Real(number.parse::<f64>().unwrap()),
            },
        }
    }
}
