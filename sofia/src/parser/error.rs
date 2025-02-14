use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexerError {
    /// The lexer encountered a ! symbol without a = symbol right after it
    #[error("! symbol does not contain a = right after it")]
    NotEqualError,

    /// The lexer encountered unmatched quotes
    #[error("Unmatched quotes defining a string")]
    UnmatchedQuotesError,
}

#[derive(Debug, Clone)]
pub enum ParserError {}
