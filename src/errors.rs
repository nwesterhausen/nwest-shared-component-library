//! Errors that can occur when using this library.
//!
//! This module contains the error types that can occur when using this library.

use thiserror::Error;

/// An error that can occur when using Attributes.
#[derive(Debug, Error, Clone)]
pub enum AttributeError {
    /// An error that can occur when using Attributes.
    #[error("Generic attribute error. {0}")]
    AttributeError(String),
    /// Try to create an attribute with a minimum value greater than the maximum value.
    #[error("Minimum value greater than maximum value. {0} > {1}")]
    MinGreaterThanMax(i32, i32),
    /// Try to create a `DecimalAttribute` with a minimum value greater than the maximum value.
    #[error("Minimum value greater than maximum value. {0} > {1}")]
    DecimalMinGreaterThanMax(f64, f64),
    /// Try to create an attribute with a maximum value less than the minimum value.
    #[error("Maximum value less than minimum value. {0} < {1}")]
    MaxLessThanMin(i32, i32),
    /// Try to create a `DecimalAttribute` with a maximum value less than the minimum value.
    #[error("Maximum value less than minimum value. {0} < {1}")]
    DecimalMaxLessThanMin(f64, f64),
    /// An error when converting an attribute to a type.
    #[error("Conversion error. {0}")]
    ConversionError(String),
}
