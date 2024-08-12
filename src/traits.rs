//! Traits exposed by the library.
//!
//! These are used to help display the data in a more human-readable format.

use crate::Stat;

/// Trait that exposes a "human-readable" name, description and value for a component.
pub trait DescriptiveComponent {
    /// Get the human-readable name of the component.
    fn name(&self) -> String;

    /// Get the human-readable description of the component.
    fn description(&self) -> String;
}

/// Trait that exposes a "human-readable" value and percentage for an attribute.
pub trait DescriptiveAttribute
where
    Self: DescriptiveComponent,
{
    /// Get the human-readable value of the attribute.
    fn value(&self) -> String;

    /// Get the human-readable percentage of the attribute.
    fn percentage(&self) -> String;
}

/// Trait for matching a skill to its affected `Stat`s.
pub trait SkillToStats {
    /// Get the affected `Stat`s
    fn affected_stat(&self) -> Vec<Stat>;
}
