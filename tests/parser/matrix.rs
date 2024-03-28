#[test]
fn test_array_basic()
{
	let input = "[1, 2, 3, 4];";
	let tokens = quark::lexer::lex(input.to_string()).unwrap();
	let expression = quark::parser::parse(tokens).unwrap();
	assert_eq!(format!("{}", expression), "[1, 2, 3, 4]\n");
}

#[test]
fn test_matrices_basic()
{
	let input = "let A = [1, 2 || 1, 2];";
	let tokens = quark::lexer::lex(input.to_string()).unwrap();
	let expression = quark::parser::parse(tokens).unwrap();
	assert_eq!(
		format!("{}", expression),
		"let A = [1, 2|
|1, 2];\n"
	);
}

#[test]
fn test_matrices_expression_nesting()
{
	let input = "let A = [1 + 2, (3 + 2) || 3 + 4, 1 * (2/ 3)];";
	let tokens = quark::lexer::lex(input.to_string()).unwrap();
	let expression = quark::parser::parse(tokens).unwrap();
	assert_eq!(
		format!("{}", expression),
		"let A = [(+ 1 2), (group (+ 3 2))|
|(+ 3 4), (* 1 (group (/ 2 3)))];\n"
	);
}
