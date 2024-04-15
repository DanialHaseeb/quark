//! # The Command-Line Interface
//!
//! The Quark CLI is a command-line interface that allows you to interact with
//! the Quark programming language.  You can use the CLI to create new projects,
//! compile Quark code to Python, and run your Quark code.

pub mod command;
pub mod error;
pub mod interface;

pub use command::*;
pub use interface::*;
