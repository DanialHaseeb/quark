/// The error message when the input file has the wrong extension.
pub const SOURCE_EXTENSION: &str = "Source file must have a `.q` extension.";

/// The error message when the output file has the wrong extension.
pub const TARGET_EXTENSION: &str = "Target file must have a `.py` extension.";

/// The error message when the project directory cannot not be created.
pub const CREATE_DIRECTORY: &str = "Failed to create project directory 📁";

/// Error message when the Quark source file cannot be created.
pub const CREATE_SOURCE: &str = "Failed to create Quark source file 📄";

/// Error message when the README file cannot be created.
pub const CREATE_README: &str = "Failed to create README file 📄";

/// Error message when the output file cannot be created.
pub const CREATE_TARGET: &str = "Failed to create target file 📄";

/// Error message when the input file cannot be read.
pub const READ_SOURCE: &str = "Failed to read the source file 📄";

/// Error message when the target Python file cannot be run.
pub const RUN_TARGET: &str = "Failed to run the target Python file. 🐍";

/// Error message when the output cannot be written to the console.
pub const OUTPUT: &str = "Failed to write to the output to standard output. 📝";
