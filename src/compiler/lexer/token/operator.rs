use itertools::PeekNth;
use std::fmt;

use DoubleCharKind::*;
use OperatorKind::*;
use SingleCharKind::*;

#[derive(Debug, PartialEq)]
pub enum OperatorKind
{
	SingleChar(SingleCharKind),
	DoubleChar(DoubleCharKind),
}

#[derive(Debug, PartialEq)]
pub enum SingleCharKind
{
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
pub enum DoubleCharKind
{
	PlusEqual,
	MinusEqual,
	LessEqual,
	GreaterEqual,
	AsteriskEqual,
	BangEqual,
	SlashEqual,
	PercentEqual,
	EqualEqual,
	SlashSlash,
}

pub trait Match<T>
where T: Iterator<Item = char>
{
	fn consume_if_matches(&mut self, c: char) -> bool;
}

impl<T> Match<T> for PeekNth<T>
where T: Iterator<Item = char>
{
	fn consume_if_matches(&mut self, c: char) -> bool
	{
		if let Some(&next_char) = self.peek()
		{
			if next_char == c
			{
				self.next();
				return true;
			}
		}
		false
	}
}

impl OperatorKind
{
	pub fn new<T>(stream: &mut PeekNth<T>) -> Self
	where T: Iterator<Item = char>
	{
		match stream.next().unwrap()
		{
			'!' if stream.consume_if_matches('=') => DoubleChar(BangEqual),
			'<' if stream.consume_if_matches('=') => DoubleChar(LessEqual),
			'>' if stream.consume_if_matches('=') => DoubleChar(GreaterEqual),
			'=' if stream.consume_if_matches('=') => DoubleChar(EqualEqual),

			'+' if stream.consume_if_matches('=') => DoubleChar(PlusEqual),
			'-' if stream.consume_if_matches('=') => DoubleChar(MinusEqual),
			'*' if stream.consume_if_matches('=') => DoubleChar(AsteriskEqual),
			'/' if stream.consume_if_matches('=') => DoubleChar(SlashEqual),
			'%' if stream.consume_if_matches('=') => DoubleChar(PercentEqual),

			'/' if stream.consume_if_matches('/') => DoubleChar(SlashSlash),

			'<' => SingleChar(Less),
			'>' => SingleChar(Greater),
			'&' => SingleChar(Ampersand),
			'!' => SingleChar(Bang),

			'+' => SingleChar(Plus),
			'-' => SingleChar(Minus),
			'*' => SingleChar(Asterisk),
			'/' => SingleChar(Slash),
			'%' => SingleChar(Percent),

			'=' => SingleChar(Equal),
			'|' => SingleChar(Pipe),
			'#' => SingleChar(Hash),
			_ => unreachable!(),
		}
	}
}

impl fmt::Display for OperatorKind
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			OperatorKind::SingleChar(op) => write!(f, "{}", op),
			OperatorKind::DoubleChar(op) => write!(f, "{}", op),
		}
	}
}

impl fmt::Display for SingleCharKind
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		let symbol = match self
		{
			SingleCharKind::Plus => "+",
			SingleCharKind::Minus => "-",
			SingleCharKind::Asterisk => "*",
			SingleCharKind::Slash => "/",
			SingleCharKind::Percent => "%",
			SingleCharKind::Hash => "#",
			SingleCharKind::Equal => "=",
			SingleCharKind::Less => "<",
			SingleCharKind::Greater => ">",
			SingleCharKind::Ampersand => "&",
			SingleCharKind::Pipe => "|",
			SingleCharKind::Bang => "!",
		};
		write!(f, "{}", symbol)
	}
}

impl fmt::Display for DoubleCharKind
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		let symbol = match self
		{
			DoubleCharKind::PlusEqual => "+=",
			DoubleCharKind::MinusEqual => "-=",
			DoubleCharKind::AsteriskEqual => "*=",
			DoubleCharKind::BangEqual => "!=",
			DoubleCharKind::SlashEqual => "/=",
			DoubleCharKind::PercentEqual => "%=",
			DoubleCharKind::EqualEqual => "==",
			DoubleCharKind::SlashSlash => "//",
			DoubleCharKind::LessEqual => "<=",
			DoubleCharKind::GreaterEqual => ">=",
		};

		write!(f, "{}", symbol)
	}
}
