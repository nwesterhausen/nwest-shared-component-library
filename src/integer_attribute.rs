//! # Integer Attribute
//!
//! This module contains the implementation of the `IntegerAttribute` struct and its methods and implementations.
//!
//! The `IntegerAttribute` struct is a simple attribute that holds an integer value. It has a minimum and maximum value that it can be clamped to.
//! The most common use case for `IntegerAttribute` is to represent a character's health, mana, or other similar values.

use bevy_ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

use crate::AttributeError;

/// An integer attribute that can be used to represent things like health, mana, etc.
///
/// When used with modifiers, it will handle rounding and clamping to the min/max values.
///
/// If it is desired to have a decimal attribute, use the `DecimalAttribute` instead.
///
/// # Example
///
/// Here's an example of creating a new attribute that we want to use for health.
///
/// ```rust
/// use nwest_shared_component_library::IntegerAttribute;
///
/// // Create a new attribute with a minimum value of 0, a maximum value of 100, and a current value of 100.
/// let mut health = IntegerAttribute::new(100);
/// ```
#[derive(Serialize, Deserialize, Clone, Copy, Component, Resource, Default)]
pub struct IntegerAttribute {
    /// The maximum value of the attribute.
    ///
    /// # Note
    ///
    /// Setting this value directly can result in the maximum becoming less than the minimum. If you need to set the maximum value, use the `set_max` method.
    pub max: i32,
    /// The minimum value of the attribute.
    ///
    /// This is usually 0, but can be negative.
    ///
    /// # Note
    ///
    /// Setting this value directly can result in the minimum becoming greater than the maximum. If you need to set the minimum value, use the `set_min` method.
    pub min: i32,
    /// The current value of the attribute.
    ///
    /// Clamped between `min` and `max`. This should usually be accessed through the `current_value` method, or implicitly, treating `IntegerAttribute` as an `i32`.
    pub current: i32,
}

impl IntegerAttribute {
    /// Create a new integer value with the given maximum.
    ///
    /// The minimum value will be set to 0, and the current value will be set to the maximum value.
    ///
    /// If a negative maximum is provided, minimum will be clamped to the maximum value.
    #[must_use]
    pub fn new(max: i32) -> Self {
        Self {
            min: 0.clamp(i32::MIN, max),
            max,
            current: max,
        }
    }

    /// Create a new integer attribute with the given values.
    ///
    /// # Errors
    ///
    /// Returns an error if the minimum value is greater than the maximum value.
    pub fn new_as_defined(min: i32, max: i32, current: i32) -> Result<Self, AttributeError> {
        if min > max {
            return Err(AttributeError::MinGreaterThanMax(min, max));
        }

        Ok(Self {
            min,
            max,
            current: current.clamp(min, max),
        })
    }

    /// Wrapper for `new_as_defined` that sets the current value to the maximum value.
    ///
    /// # Errors
    ///
    /// Returns an error if the minimum value is greater than the maximum value.
    pub fn with_min_max_and_current(
        min: i32,
        max: i32,
        current: i32,
    ) -> Result<Self, AttributeError> {
        Self::new_as_defined(min, max, current)
    }

    /// Create a new attribute with a defined maximum and minimum value.
    ///
    /// The current value will be set to the maximum value.
    ///
    /// # Errors
    ///
    /// Returns an error if the minimum value is greater than the maximum value.
    pub fn with_min_and_max(min: i32, max: i32) -> Result<Self, AttributeError> {
        Self::new_as_defined(min, max, max)
    }

    /// Set the current value of the attribute at instantiation. It will be clamped between `min` and `max`.
    ///
    /// Typically the current value will be set to the maximum value, but this allows for a different value to be set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nwest_shared_component_library::IntegerAttribute;
    ///
    /// // Create a new attribute with a maximum value of 10 and a current value of 10.
    /// let mut mana = IntegerAttribute::new(10);
    ///
    /// // Set our current value to 5.
    /// mana.set_value(5);
    /// ```
    pub fn set_value(&mut self, current: i32) {
        self.current = current.clamp(self.min, self.max);
    }

    /// Get the current value of the attribute.
    ///
    /// This will be clamped between `min` and `max`.
    #[must_use]
    pub fn current_value(&self) -> i32 {
        self.current.clamp(self.min, self.max)
    }

    /// Get the percentage of the current value between the minimum and maximum values.
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn current_percentage(&self) -> f32 {
        (self.current - self.min) as f32 / (self.max - self.min) as f32
    }

    /// Set the max value of the attribute.
    ///
    /// # Errors
    ///
    /// Returns an error if the maximum value is less than the minimum value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nwest_shared_component_library::IntegerAttribute;
    ///
    /// let mut mana = IntegerAttribute::default();
    ///
    /// // The current value is 0.
    /// assert_eq!(mana.current_value(), 0);
    /// assert_eq!(mana, 0);
    ///
    /// // Set the max value to 10.
    /// mana.set_max(10).expect("Failed to set max value.");
    /// mana.set_value(10);
    ///
    /// // The current value is now 10.
    /// assert_eq!(mana.current_value(), 10);
    /// assert_eq!(mana, 10);
    ///
    /// // Set the current value to 5.
    /// mana.set_value(5);
    ///
    /// // The current value is now 5.
    /// assert_eq!(mana.current_value(), 5);
    /// assert_eq!(mana, 5);
    /// ```
    pub fn set_max(&mut self, value: i32) -> Result<(), AttributeError> {
        if value < self.min {
            return Err(AttributeError::MaxLessThanMin(value, self.min));
        }

        self.max = value;
        self.current = self.current.clamp(self.min, self.max);

        Ok(())
    }

    /// Set the min value of the attribute.
    ///
    /// # Errors
    ///
    /// Returns an error if the minimum value is greater than the maximum value.
    pub fn set_min(&mut self, value: i32) -> Result<(), AttributeError> {
        if value > self.max {
            return Err(AttributeError::MinGreaterThanMax(value, self.max));
        }

        self.min = value;
        self.current = self.current.clamp(self.min, self.max);

        Ok(())
    }
}

impl PartialEq for IntegerAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.current == other.current
    }
}

impl PartialEq<i32> for IntegerAttribute {
    fn eq(&self, other: &i32) -> bool {
        self.current == *other
    }
}

impl PartialEq<IntegerAttribute> for i32 {
    fn eq(&self, other: &IntegerAttribute) -> bool {
        *self == other.current
    }
}

impl Eq for IntegerAttribute {}

impl std::hash::Hash for IntegerAttribute {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.max.hash(state);
        self.min.hash(state);
        self.current.hash(state);
    }
}

impl std::fmt::Debug for IntegerAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IntegerAttribute")
            .field("min", &self.min)
            .field("max", &self.max)
            .field("current", &self.current)
            .field("current_percentage", &self.current_percentage())
            .finish()
    }
}

impl std::fmt::Display for IntegerAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({:.2}%)",
            self.current,
            self.current_percentage() * 100.0,
        )
    }
}

/// Allow conversion of `IntegerAttribute` to i32.
impl From<IntegerAttribute> for i32 {
    fn from(attribute: IntegerAttribute) -> Self {
        attribute.current
    }
}

/// Allow conversion of `IntegerAttribute` to u32 (using `TryFrom`)
///
/// # Errors
///
/// Returns an error if the value is negative.
impl TryFrom<IntegerAttribute> for u32 {
    type Error = AttributeError;

    #[allow(clippy::cast_sign_loss)]
    fn try_from(attribute: IntegerAttribute) -> Result<Self, Self::Error> {
        if attribute.current < 0 {
            Err(AttributeError::ConversionError(
                "Current value is negative when trying to convert to u32.".to_string(),
            ))
        } else {
            // We know the the current value is positive, but to convince the compiler we can use `as`.
            Ok(attribute.current as Self)
        }
    }
}

/// Allow integer addition of `IntegerAttribute` and `i32`.
impl std::ops::Add<i32> for IntegerAttribute {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self {
            min: self.min,
            max: self.max,
            current: (self.current.checked_add(rhs).unwrap_or(self.max)).clamp(self.min, self.max),
        }
    }
}

/// Allow integer addition of `i32` and `IntegerAttribute` with assignment.
impl std::ops::AddAssign<i32> for IntegerAttribute {
    fn add_assign(&mut self, rhs: i32) {
        self.current =
            (self.current.checked_add(rhs).unwrap_or(self.max)).clamp(self.min, self.max);
    }
}

/// Allow integer subtraction of `IntegerAttribute` and `i32`.
impl std::ops::Sub<i32> for IntegerAttribute {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self {
            min: self.min,
            max: self.max,
            current: (self.current.checked_sub(rhs).unwrap_or(self.min)).clamp(self.min, self.max),
        }
    }
}

/// Allow integer subtraction of `i32` and `IntegerAttribute` with assignment.
impl std::ops::SubAssign<i32> for IntegerAttribute {
    fn sub_assign(&mut self, rhs: i32) {
        self.current =
            (self.current.checked_sub(rhs).unwrap_or(self.min)).clamp(self.min, self.max);
    }
}

/// Allow multiplication of `IntegerAttribute` and `i32`.
impl std::ops::Mul<i32> for IntegerAttribute {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            min: self.min,
            max: self.max,
            current: (self.current * rhs).clamp(self.min, self.max),
        }
    }
}

/// Allow multiplication of `i32` and `IntegerAttribute` with assignment.
impl std::ops::MulAssign<i32> for IntegerAttribute {
    fn mul_assign(&mut self, rhs: i32) {
        self.current = (self.current * rhs).clamp(self.min, self.max);
    }
}

/// Allow division of `IntegerAttribute` and `i32`.
impl std::ops::Div<i32> for IntegerAttribute {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            min: self.min,
            max: self.max,
            current: (self.current / rhs).clamp(self.min, self.max),
        }
    }
}

/// Allow division of `i32` and `IntegerAttribute` with assignment.
impl std::ops::DivAssign<i32> for IntegerAttribute {
    fn div_assign(&mut self, rhs: i32) {
        self.current = (self.current / rhs).clamp(self.min, self.max);
    }
}

/// Allow negation of `IntegerAttribute`. This is still clamped to the min and max values, and just tries to make the value negative.
impl std::ops::Neg for IntegerAttribute {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            min: self.min,
            max: self.max,
            current: (-self.current).clamp(self.min, self.max),
        }
    }
}

/// Allow calculating remainder of `IntegerAttribute` and `i32`. This assigns the remainder as the current value.
impl std::ops::Rem<i32> for IntegerAttribute {
    type Output = Self;

    fn rem(self, rhs: i32) -> Self::Output {
        Self {
            min: self.min,
            max: self.max,
            current: (self.current % rhs).clamp(self.min, self.max),
        }
    }
}

/// Allow calculating remainder of `i32` and `IntegerAttribute` with assignment.
impl std::ops::RemAssign<i32> for IntegerAttribute {
    fn rem_assign(&mut self, rhs: i32) {
        self.current = (self.current % rhs).clamp(self.min, self.max);
    }
}

/// Range of `IntegerAttribute` values.
impl std::ops::RangeBounds<i32> for IntegerAttribute {
    fn start_bound(&self) -> std::ops::Bound<&i32> {
        std::ops::Bound::Included(&self.min)
    }

    fn end_bound(&self) -> std::ops::Bound<&i32> {
        std::ops::Bound::Included(&self.max)
    }
}
