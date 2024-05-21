/// An error message indicating a missing `;`.
pub const SEMICOLON: &str = "Expected `;` after this";

/// An error message indicating a missing identifier.
pub const IDENTIFIER: &str = "Expected an identifier after this";

/// An error message indicating a missing `=`.
pub const EQUAL: &str = "Expected `=` after this";

/// An error message indicating a missing expression.
pub const EXPRESSION: &str = "Expected an identifier, literal, or `(` here";

/// An error message indiciating unmatches parentheses.
pub const PARENTHESIS: &str = "This parenthesis is unmatched";

/// An error message indicating unmatched brackets.
pub const BRACKET: &str = "This bracket is unmatched. Try `]`, `]a`, or `]m`";

pub const MATRIX_BRACKET: &str =
	"This matrix bracket is unmatched. Try `]`, `]m`";
