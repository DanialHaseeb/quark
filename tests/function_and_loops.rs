use quark::compiler::Compile;
const HEADER: &str = "import numpy as np\n";

#[test]
fn testing_while_loop()
{
	let input = "while true {
    let x = 1;
    let y = 2;
}
"
	.to_string();

	let expected = "while True:
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
func hello(name) -> String {
    let x = \"1\";
    var y = \"2\";
    y = 1;
    echo name, \" \", x + y;
    return x + y;
}
hello(\"Hi\");"
		.to_string();

	let expected = "def hello(name):
    x = '1'
    y = '2'
    y = 1
    print(name, ' ', x + y)
    return x + y
    
hello('Hi')"
		.to_string();

	let output = input.compile().unwrap();
	assert_eq!(output, format!("{}{}", HEADER, expected));
}
#[test]
fn testing_function_with_return_type()
{
	let input = "
func hello(name) -> Number {
    let x = 1;
    var y = 2;
    return x + y;
}"
	.to_string();

	let expected = "def hello(name):
    x = 1
    y = 2
    return x + y"
		.to_string();
	let output = input.compile().unwrap();
	assert_eq!(output, format!("{}{}", HEADER, expected));
}

#[test]
fn nasty_fibonacci()
{
	let input = "func fibonacci(n) -> Number {
    if n < 1 {
        return n;
    } else {
    let x = x(3, 3) + 3;
    return fibonacci(n - 1) + fibonacci(n - 2);
  }
}"
	.to_string();

	let expected = "def fibonacci(n):
    if n < 1:
        return n
        
    else:
        x = x(3, 3) + 3
        return fibonacci(n - 1) + fibonacci(n - 2)"
		.to_string();
	let output = input.compile().unwrap();
	assert_eq!(output, format!("{}{}", HEADER, expected));
}
