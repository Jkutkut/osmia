/// The current version of the package.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The characters that indicate the beginning of an Osmia statement.
pub const START_DELIMITER: &str = "{{";
/// The characters that indicate the end of an Osmia statement.
pub const END_DELIMITER: &str = "}}";

/// The location inside the context where the methods are stored.
pub const METHOD_CTX_LOCATION: &str = "_method";
