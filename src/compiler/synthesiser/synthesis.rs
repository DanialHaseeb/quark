use crate::compiler::parser::parse::Tree;

const TABS: &str = "    ";

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
		python(dbg!(programme.synthesise()))
	}
}

fn python(syntax: String) -> String
{
	let mut indent_level = 0;
	let mut output = String::new();
	output.push_str("import numpy as np\n");

	for char in syntax.chars()
	{
		match char
		{
			'{' =>
			{
				indent_level += 1;
				output.push_str(&format!("\n{}", TABS.repeat(indent_level)));
			}
			'}' =>
			{
				indent_level -= 1;
				output.push_str(&format!("\n{}", TABS.repeat(indent_level)));
			}
			';' => output.push_str(&format!("\n{}", TABS.repeat(indent_level))),
			_ => output.push(char),
		}
	}
	output.trim_end().to_string()
}
