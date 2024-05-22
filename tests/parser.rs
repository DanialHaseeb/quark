use quark::compiler::Compile;

#[cfg(test)]
mod tests
{
	use super::*;
	const HEADER: &str = "import numpy as np\n";

	#[test]
	fn comments_work()
	{
		let input1 = "
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

		let input2 = "
let y = 1;

let answer = x + y;
let answer = [1, 2];
let monkey = 1 + 2 / 2 / 2;
"
		.to_string();

		let output1 = input1.compile().unwrap();
		let output2 = input2.compile().unwrap();
		assert_eq!(output2, output1);
	}

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

		let expected = "y = 1
answer = x + y
answer = [1, 2]
monkey = 1 + 2 / 2 / 2";

		let output = input.compile().unwrap();
		assert_eq!(output, format!("{}{}", HEADER, expected));
	}

	#[test]
	fn testing_new_array_expression()
	{
		let input = "
let x = [1, 2, 3]a;
"
		.to_string();

		let expected = "x = [1, 2, 3]".to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, format!("{}{}", HEADER, expected));
	}

	#[test]
	fn testing_array_expression()
	{
		let input = "
let x = [1, 2, 3];
"
		.to_string();

		let expected = "x = [1, 2, 3]".to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, format!("{}{}", HEADER, expected));
	}

	#[test]
	fn testing_default_array_expression()
	{
		let input = "
let x = [];
"
		.to_string();

		let expected = "x = []".to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, format!("{}{}", HEADER, expected));
	}

	#[test]
	fn testing_empty_array_expression()
	{
		let input = "
let x = []a;
"
		.to_string();

		let expected = "x = []".to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, format!("{}{}", HEADER, expected));
	}

	#[test]
	fn testing_empty_matrix_expression()
	{
		let input = "
let x = []m;
"
		.to_string();

		let expected = "x = np.array([[],])".to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, format!("{}{}", HEADER, expected));
	}

	#[test]
	fn testing_new_matrix_expression()
	{
		let input = "
let x = [1, 2, 3]m;
"
		.to_string();

		let expected = "x = np.array([[1, 2, 3],])".to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, format!("{}{}", HEADER, expected));
	}

	#[test]
	fn testing_new_matrix_expression_without_annotation()
	{
		let input = "
let x = [1, 2, 3 | 1, 2, 3 || 1, 2, 3];
"
		.to_string();
		let expected = "x = np.array([[1, 2, 3],[1, 2, 3],[1, 2, 3],])".to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, format!("{}{}", HEADER, expected));
	}

	#[test]
	fn testing_while_loop()
	{
		let input = "while 1 {
    let x = 1;
    let y = 2;
}
"
		.to_string();

		let expected = "while 1:
    x = 1
    y = 2"
			.to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, format!("{}{}", HEADER, expected));
	}

	#[test]
	fn testing_function()
	{
		let input = "
fn hello(name: Int) {
    let x = 1;
    var y = 2;
}
"
		.to_string();

		let expected = "def hello(name):
    x = 1
    y = 2"
			.to_string();

		let output = input.compile().unwrap();
		assert_eq!(output, format!("{}{}", HEADER, expected));
	}
	#[test]
	fn testing_function_with_return_type()
	{
		let input = "
fn hello(name: Int) -> Int {
    let x = 1;
    var y = 2;
    return x + y
}
"
		.to_string();

		let expected = "def hello(name):
    x = 1
    y = 2
    return x"
			.to_string();
		let output = input.compile().unwrap();
		assert_eq!(output, format!("{}{}", HEADER, expected));
	}
}
