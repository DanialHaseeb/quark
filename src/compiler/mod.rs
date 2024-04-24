pub mod compiler;
mod error;
mod lexer;
mod parser;
mod semanter;
mod synthesiser;

pub use compiler::*;
use error::*;
