use crate::language::grammar::expression::{Expression, Items};
use crate::language::lexicon::token::{Kind::*, Token};
use crate::language::utils::Span;
use anyhow::Result;

pub fn items<I>(
	stream: &mut std::iter::Peekable<I>,
	source: &[Vec<char>],
) -> Result<Option<Items>>
where
	I: Iterator<Item = Token>,
{
	if stream.peek().is_some_and(|token| token.is_closing())
	{
		Ok(None)
	}
	else
	{
		let mut expressions = Vec::new();

		let expression = Expression::try_from_stream(stream, source)?;

		let start = expression.span.start;
		let mut end = expression.span.end;

		expressions.push(expression);

		while stream.next_if(|token| token.kind == Comma).is_some()
		{
			let expression = Expression::try_from_stream(stream, source)?;
			end = expression.span.end;
			expressions.push(expression);
		}

		let span = Span { start, end };

		Ok(Some(Items { span, expressions }))
	}
}
