//! # The Language Specification
//!
//! This module contains the language specification for Quark. The language is
//! defined by a lexicon, grammar, and semantics. The lexicon defines the words
//! and symbols that make up the language. The grammar defines the rules for
//! how the words and symbols can be combined to form valid sentences. The
//! semantics define the meaning of the sentences.

pub mod language;
pub mod symbol;
pub mod token;

pub use language::*;
pub use symbol::*;
pub use token::Token;
