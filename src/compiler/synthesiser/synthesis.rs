use crate::compiler::parser::parse::Tree;

/// Types that can be synthesised into the target language.
///
/// This trait is used to translate Quark's linguistic structures into the
/// target language.
pub trait Synthesis
{
	/// Creates a valid string representation of this structure into the target
	/// language.
	///
	/// # Returns
	/// * A valid string representation of this structure in the target language.
	fn synthesise(self) -> String;
}

impl Synthesis for Tree
{
	fn synthesise(self) -> String
	{
		let Tree(programme) = self;
		programme.synthesise()
	}
}
