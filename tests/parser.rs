use quark::compiler::Compile;

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn it_works()
	{
		let input = "
let y = 1;

// let y = [1, 2|
/*       |1, 2];
 let x = [1, 2| 1, 2]; */
let answer = x + y;
let answer = [1, 2];
let monkey = 1 + 2 / 2 / 2;
//print(answer);
"
		.to_string();

		let output = input.compile().unwrap();
		assert_eq!(
			output,
			"y = 1
answer = x + y
answer = [1, 2]
monkey = 1 + 2 / 2 / 2
"
			.to_string()
		);
	}
	#[test]
	fn testing_new_array_expression()
	{
		let input = "
let x = [1, 2, 3]a;
"
		.to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, "x = [1, 2, 3]\n".to_string());
	}

	#[test]
	fn testing_array_expression()
	{
		let input = "
let x = [1, 2, 3];
"
		.to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, "x = [1, 2, 3]\n".to_string());
	}

	#[test]
	fn testing_default_array_expression()
	{
		let input = "
let x = [];
"
		.to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, "x = []\n".to_string());
	}

	#[test]
	fn testing_empty_array_expression()
	{
		let input = "
let x = []a;
"
		.to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, "x = []\n".to_string());
	}

	#[test]
	fn testing_empty_matrix_expression()
	{
		let input = "
let x = []m;
"
		.to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, "x = np.array([[]])\n".to_string());
	}

	#[test]
	fn testing_new_matrix_expression()
	{
		let input = "
let x = [1, 2, 3]m;
"
		.to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, "x = np.array([[1, 2, 3]])\n".to_string());
	}

	#[test]
	fn testing_new_matrix_expression_without_annotation()
	{
		let input = "
let x = [1, 2, 3 | 1, 2, 3];
"
		.to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, "x = np.array([[1, 2, 3][1, 2, 3]])\n".to_string());
	}
}
