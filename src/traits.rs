//! Traits exposed by the library.
//!
//! These are used to help display the data in a more human-readable format.

/// Trait that exposes a "human-readable" name, description and value for a component.
pub trait DescriptiveComponent {
    /// Get the human-readable name of the component.
    fn name(&self) -> String;

    /// Get the human-readable description of the component.
    fn description(&self) -> String;

    /// Get the human-readable value of the component.
    ///
    /// This is the raw value of the component, and it is formatted to 2 decimal places (or left as an integer if it is an integer).
    fn value(&self) -> String;

    /// Get a human-readable percentage value of the component (value / max). This is formatted to 2 decimal places.
    ///
    /// This may produce unwanted results for components like `Visibility` or `Range` that are not meant to be displayed as percentages
    /// (as opposed to `Health` or `Mana` which are basically a quantity measurement).
    fn percentage(&self) -> String;
}
