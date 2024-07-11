use bevy_ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

use crate::AttributeError;

/// An integer attribute that can be used to represent things like health, mana, etc.
///
/// When used with modifiers, it will handle rounding and clamping to the min/max values.
///
/// If it is desired to have a decimal attribute, use the `FloatAttribute` instead.
#[derive(
    Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Component, Resource, Hash, Default,
)]
pub struct IntegerAttribute {
    /// The maximum value of the attribute.
    pub max: i32,
    /// The minimum value of the attribute.
    ///
    /// This is usually 0, but can be negative.
    pub min: i32,
    /// The current value of the attribute.
    ///
    /// Clamped between `min` and `max`. This should usually be accessed through the `current_value` method, or implicitly through the `Deref` implementation.
    pub current: i32,
}

impl IntegerAttribute {
    /// Create a new integer attribute with the given values.
    ///
    /// # Errors
    ///
    /// Returns an error if the minimum value is greater than the maximum value.
    pub fn new(min: i32, max: i32, current: i32) -> Result<Self, AttributeError> {
        if min > max {
            return Err(AttributeError::MinGreaterThanMax(min, max));
        }

        Ok(Self {
            min,
            max,
            current: current.clamp(min, max),
        })
    }

    /// Set the minimum value of the attribute at instantiation.
    ///
    /// # Errors
    ///
    /// Returns an error if the minimum value is greater than the maximum value.
    ///
    /// If this is set before setting the maximum value, the maximum value will be set to the same value.
    ///
    /// # Example
    ///
    /// ```
    /// use nwest_shared_component_library::IntegerAttribute;
    ///
    /// let attribute = IntegerAttribute::default();
    /// // Set the minimum value to 10.
    /// let attribute = attribute.set_min(10).unwrap();
    /// // The minimum value is now 10.
    /// assert_eq!(attribute.min, 10);
    /// // The maximum value is now 10.
    /// assert_eq!(attribute.max, 10);
    /// ```
    pub fn set_min(mut self, min: i32) -> Result<Self, AttributeError> {
        // If we are uninitialized, set the max to the min.
        if self.max = 0 && self.min = 0 {
            self.max = min;
        }

        if min > self.max {
            return Err(AttributeError::MinGreaterThanMax(min, self.max));
        }

        self.min = min;
        self.current = self.current.clamp(self.min, self.max);

        Ok(self)
    }

    /// Get the current value of the attribute.
    ///
    /// This will be clamped between `min` and `max`.
    pub fn current_value(&self) -> i32 {
        self.current.clamp(self.min, self.max)
    }

    /// Set the current value of the attribute.
    ///
    /// This will be clamped between `min` and `max`.
    pub fn set_current_value(&mut self, value: i32) {
        self.current = value.clamp(self.min, self.max);
    }

    /// Add a value to the current value of the attribute.
    ///
    /// This will be clamped between `min` and `max`.
    pub fn add_to_current_value(&mut self, value: i32) {
        self.current = (self.current + value).clamp(self.min, self.max);
    }

    /// Subtract a value from the current value of the attribute.
    ///
    /// This will be clamped between `min` and `max`.
    pub fn subtract_from_current_value(&mut self, value: i32) {
        self.current = (self.current - value).clamp(self.min, self.max);
    }

    /// Multiply the current value of the attribute by a value.
    ///
    /// This will be clamped between `min` and `max`.
    pub fn multiply_current_value(&mut self, value: i32) {
        self.current = (self.current * value).clamp(self.min, self.max);
    }

    /// Divide the current value of the attribute by a value.
    ///
    /// This will be clamped between `min` and `max`.
    pub fn divide_current_value(&mut self, value: i32) {
        self.current = (self.current / value).clamp(self.min, self.max);
    }
}
