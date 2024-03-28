mod display;

use quark::compiler::parser::declaration::expression::grammar::expression;

#[test]
fn test_numbers_expresssion()
{
	let input = "1 + 2";
	let tokens = quark::compiler::lexer::lex(input.to_string()).unwrap();
	let mut tokens = tokens.into_iter().peekable();
	let expr = expression(&mut tokens).unwrap();
	assert_eq!(format!("{}", expr), "(+ 1 2)");
}

#[test]
fn test_numbers_grouping_expresssion()
{
	let input = "-123 * (45.67)";
	let tokens = quark::compiler::lexer::lex(input.to_string()).unwrap();
	let mut tokens = tokens.into_iter().peekable();
	let expr = expression(&mut tokens).unwrap();
	assert_eq!(format!("{}", expr), "(* (- 123) (group 45.67))");
}
