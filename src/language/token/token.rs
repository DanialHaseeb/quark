use std::fmt::{Display, Formatter, Result};

use super::*;

/// A token in a source file.
///
/// A token is a word or symbol that represents a unit of meaning in the
/// language.
#[derive(Clone, Debug, PartialEq)]
pub struct Token
{
	/// The span of the token in the source file.
	pub span: Span,

	/// The kind of the token.
	pub kind: Kind,
}

impl Display for Token
{
	fn fmt(&self, formatter: &mut Formatter) -> Result
	{
		let Self { span, kind } = self;
		write!(formatter, "{span}: {kind:?}")
	}
}
