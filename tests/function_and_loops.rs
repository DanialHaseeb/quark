use quark::compiler::Compile;
const HEADER: &str = "import numpy as np\n";

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
fn hello(name) {
    let x = \"1\";
    var y = \"2\";
    y = 1;
    echo name, \" \", x + y
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
fn hello(name) {
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