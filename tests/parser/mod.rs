mod expression;
mod matrix;

#[test]
fn test_numbers_parsing() {
    let input = "1 + 2;";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let expression = quark::parser::parse(tokens).unwrap();
    assert_eq!(format!("{}", expression), "(+ 1 2)\n");
}

#[test]
fn test_paranthesis_parsing() {
    let input = "-123 * (45.67);";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let expression = quark::parser::parse(tokens).unwrap();
    assert_eq!(format!("{}", expression), "(* (- 123) (group 45.67))\n");
}

#[test]
fn test_paranthesis_parsing_with_multiple_operators() {
    let input = "-123 * (45.67) / 2;";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let expression = quark::parser::parse(tokens).unwrap();
    assert_eq!(
        format!("{}", expression),
        "(* (- 123) (/ (group 45.67) 2))\n"
    );
}

#[test]
fn test_variable_parsing() {
    let input = "let x = 1;";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let expression = quark::parser::parse(tokens).unwrap();
    assert_eq!(format!("{}", expression), "let x = 1;\n");
}

#[test]
fn test_variable_paranthesis_parsing_with_multiple_operators() {
    let input = "let x = -123 * (45.67) / 2;";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let expression = quark::parser::parse(tokens).unwrap();
    assert_eq!(
        format!("{}", expression),
        "let x = (* (- 123) (/ (group 45.67) 2));\n"
    );
}

#[test]
fn test_logical_operators() {
    let input = "1 == 2 and 3 == 3 or 1 < 3;";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let expression = quark::parser::parse(tokens).unwrap();
    assert_eq!(
        format!("{}", expression),
        "(Or (And (== 1 2) (== 3 3)) (< 1 3))\n"
    );
}

#[test]
fn test_print_function() {
    let input = "print(1 == 2 and 3 == 3 or 1 < 3);";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let expression = quark::parser::parse(tokens).unwrap();
    assert_eq!(
        format!("{}", expression),
        "print((Or (And (== 1 2) (== 3 3)) (< 1 3)))\n"
    );
}

#[test]
#[should_panic]
fn test_print_panic_function() {
    let input = "print(1 == 2 and 3 == 3 or 1 < 3;);";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let expression = quark::parser::parse(tokens).unwrap();
    assert_eq!(
        format!("{}", expression),
        "print((Or (And (== 1 2) (== 3 3)) (< 1 3)))\n"
    );
}

#[test]
fn test_if_statement() {
    let input = "if 1 == 2 { print(1); if 1 == 2 { print(1); } }";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let expression = quark::parser::parse(tokens).unwrap();
    assert_eq!(
        format!("{}", expression),
        "if (== 1 2) {\nprint(1)\nif (== 1 2) {\nprint(1)\n}\n}\n"
    );
}

#[test]
fn test_while_statement() {
    let input = "while 1 == 2 { print(1); }";
    let tokens = quark::lexer::lex(input.to_string()).unwrap();
    let expression = quark::parser::parse(tokens).unwrap();
    assert_eq!(format!("{}", expression), "while (== 1 2) {\nprint(1)\n}\n");
}
