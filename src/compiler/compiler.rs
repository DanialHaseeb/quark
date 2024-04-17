use anyhow::Result;

use super::*;
use lexer::Lex;

/// Types that can be compiled.
///
/// This trait is used to compile Quark code into Python code.
pub trait Compile
{
	/// Compiles the Quark code into Python code.
	///
	/// ### Returns
	/// * The output Python code.
	///
	/// ### Errors
	/// * If the Quark code cannot be compiled.
	fn compile(self) -> Result<String>;
}

impl Compile for String
{
	fn compile(self) -> Result<String>
	{
		let source: Vec<Vec<_>> = self
			.lines()
			.map(|line| format!("{line}\n").chars().collect())
			.collect();

		let tokens = self.lex(&source)?;

		for token in &tokens
		{
			eprintln!("{token:?}");
		}

		// self
		// 	.lex(&source)?
		// 	.parse(&source)?
		// 	.analyse(&source)?
		// 	.synthesise(&source)

		Ok("".to_string())
	}
}
