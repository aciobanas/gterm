use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZeroDenominatorError;

impl fmt::Display for ZeroDenominatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ratio denominator cannot be zero")
    }
}

impl std::error::Error for ZeroDenominatorError {}
