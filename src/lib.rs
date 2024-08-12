//! This library provides a set of generic components that could be used in a game.
//!

pub mod decimal_attribute;
pub mod enums;
pub mod errors;
pub mod integer_attribute;
pub mod statistic;
pub mod traits;

pub use decimal_attribute::DecimalAttribute;
pub use enums::BaseStat;
pub use enums::Stat;
pub use enums::StatModifier;
pub use enums::TypeCategory;
pub use errors::AttributeError;
pub use integer_attribute::IntegerAttribute;
pub use statistic::Statistic;
