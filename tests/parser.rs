use quark::compiler::Compile;

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn it_works()
	{
		let result = 2 + 2;
		assert_eq!(result, 4);
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
	fn testing_empty_array_expression()
	{
		let input = "
let x = [];
"
		.to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, "x = []\n".to_string());
	}

	#[test]
	fn testing_new_matrix_expression()
	{
		let input = "
let x = [1, 2, 3]m;
"
		.to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, "x = np.array([1, 2, 3])".to_string());
	}
}
