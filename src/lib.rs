//! Hello
//!

pub mod decimal_attribute;
pub mod errors;
pub mod integer_attribute;
pub mod stat_enum;
pub mod statistic;
pub mod traits;

pub use decimal_attribute::DecimalAttribute;
pub use errors::AttributeError;
pub use integer_attribute::IntegerAttribute;
pub use stat_enum::BaseStat;
pub use stat_enum::DamageCategory;
pub use stat_enum::Stat;
pub use stat_enum::StatModifier;
pub use statistic::Statistic;
