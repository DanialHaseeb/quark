/// An error message indicating a missing `;`.
pub const SEMICOLON_AFTER: &str = "Expected `;` after this";

/// An error message indicating a missing identifier.
pub const IDENTIFIER_AFTER: &str = "Expected an identifier after this";

pub const EXPECTED_RETURN_TYPE: &str = "Expected a return type after this";

/// An error message indicating a wrong token.
pub const IDENTIFIER: &str = "Expected an identifier here";

/// An error message indicating a missing `=`.
pub const EQUALS_AFTER: &str = "Expected `=` after this";

pub const BLOCK_AFTER: &str = "Expected a block `{` `}` after this";

pub const CONDITION_AFTER: &str = "Expected a bool expression after this";

pub const FUNCTION_NAME: &str = "Expected a function name after this";

/// An error message indicating a missing expression.
pub const EXPRESSION: &str =
	"Expected an expression starting with an identifier, literal, or `(` here";

pub const EXPRESSION_AFTER: &str =
	"Expected an expression starting with an identifier, literal, or `(` after this";

/// An error message indiciating unmatches parentheses.
pub const PARENTHESIS: &str = "This parenthesis is unmatched";

pub const EXPECT_PARENTHESIS: &str = "Expected a parenthesis after this";

pub const BRACE: &str = "This brace is unmatched";

/// An error message indicating unmatched brackets.
pub const BRACKET: &str = "This bracket is unmatched. Try `]`, `]a`, or `]m`";

pub const MATRIX_BRACKET: &str =
	"This matrix bracket is unmatched. Try `]`, `]m`";

pub const PARAMS_AFTER: &str = "Expected function params `(` after this";

pub const COMMA: &str = "Expected `,` here";
