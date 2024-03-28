use quark::{
	compiler::lexer::token::{
		literal::{LiteralKind::*, NumberKind::*},
		operator::{OperatorKind::*, SingleCharKind::*},
		Token,
		TokenKind::*,
	},
	compiler::parser::declaration::expression::structs::{
		BinaryExprBody, ExpressionKind::*, GroupingExprBody, LiteralExprBody,
		UnaryExprBody,
	},
};

#[test]
fn test_expression_printing()
{
	let expression = BinaryExpr(BinaryExprBody {
		left: Box::new(UnaryExpr(UnaryExprBody {
			operator: Token {
				token_kind: Operator(SingleChar(Minus)),
			},
			expression: Box::new(LiteralExpr(LiteralExprBody {
				value: Token {
					token_kind: Literal(Number(Int(123))),
				},
			})),
		})),
		operator: Token {
			token_kind: Operator(SingleChar(Asterisk)),
		},
		right: Box::new(GroupingExpr(GroupingExprBody {
			expression: Box::new(LiteralExpr(LiteralExprBody {
				value: Token {
					token_kind: Literal(Number(Float(45.67))),
				},
			})),
		})),
	});

	assert_eq!(format!("{}", expression), "(* (- 123) (group 45.67))");
}
