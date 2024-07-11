//! //! # Integer Attribute
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
/// If it is desired to have a decimal attribute, use the `FloatAttribute` instead.
///
/// # Example
///
/// Here's an example of creating a new attribute that we want to use for health.
///
/// ```rust
/// use nwest_shared_component_library::IntegerAttribute;
///
/// // Create a new attribute with a minimum value of 0, a maximum value of 100, and a current value of 100.
/// let health = IntegerAttribute::new(0, 100, 100).unwrap();
/// // Alternatively, you can create a new attribute by chaining. When setting a maximum this way, the current value will be set to the maximum value.
/// let health_2 = IntegerAttribute::default().set_max(100).unwrap();
///
/// // Get the current value of the attribute.
/// let current_health = health.current_value();
/// // The current health is 100.
/// assert_eq!(current_health, 100);
///
/// // Add 10 to the current health.
/// health += 10;
/// // The current health is now 100, as it is clamped to the maximum value.
/// assert_eq!(health.current_value(), 100);
///
/// // Subtract 15 from the current health.
/// health -= 15;
/// // The current health is now 85.
/// assert_eq!(health.current_value(), 85);
/// assert_eq!(health, 85);
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
    /// Clamped between `min` and `max`. This should usually be accessed through the `current_value` method, or implicitly through the `IntegerAttribute` struct.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nwest_shared_component_library::IntegerAttribute;
    ///
    /// let attribute = IntegerAttribute::default();
    ///
    /// // The current value is 0.
    /// assert_eq!(attribute.current_value(), 0);
    /// assert_eq!(attribute, 0);
    ///
    /// // Set the max value to 10. (This will also set the current value to 10.)
    /// attribute.set_max(10).unwrap();
    ///
    /// // The current value is now 10.
    /// assert_eq!(attribute.current_value(), 10);
    /// assert_eq!(attribute, 10);
    ///
    /// // Set the current value to 5.
    /// attribute.set_current(5);
    ///
    /// // The current value is now 5.
    /// assert_eq!(attribute.current_value(), 5);
    /// assert_eq!(attribute, 5);
    /// ```
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
        if self.max == 0 && self.min == 0 {
            self.max = min;
        }

        if min > self.max {
            return Err(AttributeError::MinGreaterThanMax(min, self.max));
        }

        self.min = min;
        self.current = self.current.clamp(self.min, self.max);

        Ok(self)
    }

    /// Set the maximum value of the attribute at instantiation.
    ///
    /// This will also set the current value to the maximum value if it is uninitialized.
    ///
    /// # Errors
    ///
    /// Returns an error if the maximum value is less than the minimum value.
    ///
    /// # Example
    ///
    /// ```
    /// use nwest_shared_component_library::IntegerAttribute;
    ///
    /// let attribute = IntegerAttribute::default();
    /// // Set the maximum value to 10.
    /// let attribute = attribute.set_max(10).unwrap();
    /// // The maximum value is now 10.
    /// assert_eq!(attribute.max, 10);
    /// // The minimum value is now 10.
    /// assert_eq!(attribute.min, 10);
    /// ```
    pub fn set_max(mut self, max: i32) -> Result<Self, AttributeError> {
        // If the current value is also uninitialized, set it to the max.
        if self.current == 0 {
            self.current = max;
        }

        if max < self.min {
            return Err(AttributeError::MaxLessThanMin(max, self.min));
        }

        self.max = max;
        self.current = self.current.clamp(self.min, self.max);

        Ok(self)
    }

    /// Set the current value of the attribute at instantiation. It will be clamped between `min` and `max`.
    ///
    /// Typically the current value will be set to the maximum value, but this allows for a different value to be set.
    ///
    /// # Example
    ///
    /// ```
    /// use nwest_shared_component_library::IntegerAttribute;
    ///
    /// let attribute = IntegerAttribute::default();
    /// // Set our max to 10.
    /// let attribute = attribute.set_max(10).unwrap();
    /// // Set our current value to 5.
    /// let attribute = attribute.set_current(5);
    ///
    /// // The current value is now 5.
    /// assert_eq!(attribute.current, 5);
    ///
    /// // Set our current value to 15.
    /// let attribute = attribute.set_current(15);
    ///
    /// // The current value is now 10, as it is clamped to the max.
    /// assert_eq!(attribute.current, 10);
    ///
    /// // Set our current value to -5.
    /// let attribute = attribute.set_current(-5);
    ///
    /// // The current value is now 0, as it is clamped to the min.
    /// assert_eq!(attribute.current, 0);
    /// ```
    #[must_use]
    pub fn set_current(mut self, current: i32) -> Self {
        self.current = current.clamp(self.min, self.max);

        self
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
        match self.min.cmp(&0) {
            std::cmp::Ordering::Less => {
                (self.current + self.min) as f32 / (self.max + self.min) as f32
            }
            std::cmp::Ordering::Equal => self.current as f32 / self.max as f32,
            std::cmp::Ordering::Greater => {
                (self.current - self.min) as f32 / (self.max - self.min) as f32
            }
        }
    }

    /// Set the current value of the attribute.
    ///
    /// This will be clamped between `min` and `max`.
    pub fn set_current_value(&mut self, value: i32) {
        self.current = value.clamp(self.min, self.max);
    }

    /// Set the max value of the attribute.
    ///
    /// # Errors
    ///
    /// Returns an error if the maximum value is less than the minimum value.
    pub fn set_max_value(&mut self, value: i32) -> Result<(), AttributeError> {
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
    pub fn set_min_value(&mut self, value: i32) -> Result<(), AttributeError> {
        if value > self.max {
            return Err(AttributeError::MinGreaterThanMax(value, self.max));
        }

        self.min = value;
        self.current = self.current.clamp(self.min, self.max);

        Ok(())
    }
}

impl Eq for IntegerAttribute {}

impl PartialEq for IntegerAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.current == other.current
    }
}

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
        write!(f, "{} ({:.2}%)", self.current, self.current_percentage(),)
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

/// Allow integer addition of `IntegerAttribute` and `u32`.
///
/// If the `u32` value would wrap when converted to `i32`, the current value is set to the maximum value of the attribute.
#[allow(clippy::cast_possible_wrap)]
impl std::ops::Add<u32> for IntegerAttribute {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        // Check for (cast) wrapping. If we would wrap, set the current value to the maximum value.
        if rhs > i32::MAX as u32 {
            return Self {
                min: self.min,
                max: self.max,
                current: self.max,
            };
        }

        Self {
            min: self.min,
            max: self.max,
            current: self
                .current
                .checked_add(rhs as i32)
                .unwrap_or(self.max)
                .clamp(self.min, self.max),
        }
    }
}

/// Allow integer subtraction of `IntegerAttribute` and `u32`.
///
/// If the `u32` value would wrap when converted to `i32`, we use `i32::MAX` as the value to subtract.
#[allow(clippy::cast_possible_wrap)]
impl std::ops::Sub<u32> for IntegerAttribute {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self::Output {
        // Check for (cast) wrapping. If we would wrap, perform subrtraction using `i32::MAX` instead of `rhs`.
        if rhs > i32::MAX as u32 {
            return Self {
                min: self.min,
                max: self.max,
                current: self
                    .current
                    .checked_sub(i32::MAX)
                    .unwrap_or(self.min)
                    .clamp(self.min, self.max),
            };
        }

        Self {
            min: self.min,
            max: self.max,
            current: (self.current - rhs as i32).clamp(self.min, self.max),
        }
    }
}

/// Allow multiplication of `IntegerAttribute` and `u32`.
///
/// If the `u32` value would wrap when converted to `i32`, we set the current value to the maximum or minimum value of the attribute instead. If the current
/// value is 0, we return the current value.
#[allow(clippy::cast_possible_wrap)]
impl std::ops::Mul<u32> for IntegerAttribute {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        // Check for a wrap when casting to `i32`. Approriately return the max or min value instead of the result.
        if rhs > i32::MAX as u32 {
            match self.current.cmp(&0) {
                std::cmp::Ordering::Less => {
                    return Self {
                        min: self.min,
                        max: self.max,
                        current: self.min,
                    };
                }
                std::cmp::Ordering::Equal => {
                    return Self {
                        min: self.min,
                        max: self.max,
                        current: self.current,
                    };
                }
                std::cmp::Ordering::Greater => {
                    return Self {
                        min: self.min,
                        max: self.max,
                        current: self.max,
                    };
                }
            }
        }

        Self {
            min: self.min,
            max: self.max,
            current: (self.current.checked_mul(rhs as i32).unwrap_or_else(|| {
                match self.current.cmp(&0) {
                    std::cmp::Ordering::Less => self.min,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => self.max,
                }
            }))
            .clamp(self.min, self.max),
        }
    }
}

/// Allow division of `IntegerAttribute` and `u32`.
///
/// If the `u32` value is 0, we return the current value. If the conversion to `i32` would wrap, we consider it dividing by `i32::MAX` and set the current value to 0.
#[allow(clippy::cast_possible_wrap)]
impl std::ops::Div<u32> for IntegerAttribute {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        // Check for division by 0 OR if the value would wrap when casting to `i32`.
        if rhs == 0 || rhs > i32::MAX as u32 {
            return Self {
                min: self.min,
                max: self.max,
                current: 0.clamp(self.min, self.max),
            };
        }

        Self {
            min: self.min,
            max: self.max,
            current: (self.current / rhs as i32).clamp(self.min, self.max),
        }
    }
}
