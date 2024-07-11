//! Errors that can occur when using this library.
//!
//! This module contains the error types that can occur when using this library.

use thiserror::Error;

/// An error that can occur when using Attributes.
#[derive(Debug, Error)]
pub enum AttributeError {
    /// An error that can occur when using Attributes.
    #[error("Generic attribute error. {0}")]
    AttributeError(String),
    /// Try to create an attribute with a minimum value greater than the maximum value.
    #[error("Minimum value greater than maximum value. {0} > {1}")]
    MinGreaterThanMax(i32, i32),
}
