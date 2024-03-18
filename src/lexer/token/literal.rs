use itertools::structs::PeekNth;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(Number),
    String(QuarkString),
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Natural(u64),
    Integer(i64),
    Real(f64),
}

#[derive(Debug, PartialEq)]
pub struct QuarkString(String);

impl Literal {
    pub fn new<T>(stream: &mut PeekNth<T>) -> Self
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
    fn new<T>(stream: &mut PeekNth<T>) -> Self
    where
        T: Iterator<Item = char>,
    {
        let mut string = String::new();
        stream.next(); // consume the opening quote

        while let Some(&symbol) = stream.peek() {
            if symbol == '"' {
                stream.next();
                break;
            }
            string.push(stream.next().unwrap());
        }
        QuarkString(string)
    }
}

impl Number {
    pub fn new<T>(stream: &mut PeekNth<T>) -> Self
    where
        T: Iterator<Item = char>,
    {
        let mut number = String::new();

        while let Some(&symbol) = stream.peek() {
            if !symbol.is_numeric() {
                break;
            }
            number.push(stream.next().unwrap());
        }

        if let Ok(natural) = number.parse::<u64>() {
            Number::Natural(natural)
        } else if let Ok(integer) = number.parse::<i64>() {
            Number::Integer(integer)
        } else if let Ok(real) = number.parse::<f64>() {
            Number::Real(real)
        } else {
            panic!("Failed to parse number")
        }
    }
}

// PeekNext Utils

trait PeekNext<I: Iterator> {
    fn peek_next(&mut self) -> Option<&I::Item>;
}

impl<I: Iterator> PeekNext<I> for PeekNth<I> {
    fn peek_next(&mut self) -> Option<&<I as Iterator>::Item> {
        self.peek_nth(1)
    }
}
