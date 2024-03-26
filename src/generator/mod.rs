use crate::parser::Program;

pub trait CodeGenerator {
    fn generate(&self) -> String;
}

pub fn intermediate(program: Program) -> String {
    program.generate()
}

pub fn python(syntax: String) -> String {
    let mut indent_level = 0;
    let mut output = String::new();
    output.push_str("import numpy as np\n");

    for char in syntax.chars() {
        match char {
            '{' => {
                indent_level += 1;
                output.push_str(&format!("\n{}", "\t".repeat(indent_level)));
            }
            '}' => {
                indent_level -= 1;
                output.push_str(&format!("\n{}", "\t".repeat(indent_level)));
            }
            ';' => output.push_str(&format!("\n{}", "\t".repeat(indent_level))),
            _ => output.push(char),
        }
    }
    output
}
