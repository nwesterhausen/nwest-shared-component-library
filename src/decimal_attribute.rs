//! # Decimal Attribute
//!
//! This module contains the implementation of the `DecimalAttribute` struct. Similar to the `IntegerAttribute`,
//! the `DecimalAttribute` is a struct that represents an attribute with a decimal value. This can be used to
//! represent attributes like speed, weight, etc.
//!
//! Sometimes an integer value makes sense for an attribute, but other times a decimal value is more appropriate.

use bevy_ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

use crate::AttributeError;

/// A struct representing an attribute with a decimal value.
///
/// This struct is used to represent attributes like speed, weight, etc.
///
/// # Example
///
/// ```rust
/// use nwest_shared_components_library::DecimalAttribute;
///
/// let mut speed = DecimalAttribute::new(10.0);
/// ```
#[derive(Serialize, Deserialize, Clone, Copy, Component, Resource, Default)]
pub struct DecimalAttribute {
    /// The current value of the attribute.
    pub current: f64,
    /// The minimum value of the attribute.
    ///
    /// This value is used to ensure that the attribute does not go below a certain threshold.
    ///
    /// By default, this value is set to `0.0`.
    pub min: f64,
    /// The maximum value of the attribute.
    ///
    /// This value is used to ensure that the attribute does not go above a certain threshold.
    pub max: f64,
}

impl DecimalAttribute {
    /// Creates a new `DecimalAttribute` with the given value as its current value and maximum value.
    #[must_use]
    pub const fn new(value: f64) -> Self {
        Self {
            current: value,
            min: 0.0,
            max: value,
        }
    }

    /// Creates a new `DecimalAttribute` with the given values for its current, minimum, and maximum values.
    ///
    /// # Errors
    ///
    /// Returns an error if the minimum value is greater than the maximum value, or if the maximum value is less than the minimum value.
    pub fn as_defined(value: f64, min: f64, max: f64) -> Result<Self, AttributeError> {
        if min > max {
            return Err(AttributeError::DecimalMinGreaterThanMax(min, max));
        }

        if max < min {
            return Err(AttributeError::DecimalMaxLessThanMin(max, min));
        }

        Ok(Self {
            current: value,
            min,
            max,
        })
    }

    /// Creates a new `DecimalAttribute` with the given values for its current, minimum, and maximum values.
    ///
    /// This is a wrapper around `as_defined`.
    ///
    /// # Errors
    ///
    /// Returns an error if the minimum value is greater than the maximum value, or if the maximum value is less than the minimum value.
    pub fn with_min_max_and_current(
        value: f64,
        min: f64,
        max: f64,
    ) -> Result<Self, AttributeError> {
        Self::as_defined(value, min, max)
    }

    /// Creates a new `DecimalAttribute` with the given values for its minimum and maximum values. It sets the current value to the maximum value.
    ///
    /// This is a wrapper around `as_defined`.
    ///
    /// # Errors
    ///
    /// Returns an error if the minimum value is greater than the maximum value, or if the maximum value is less than the minimum value.
    pub fn with_min_and_max(min: f64, max: f64) -> Result<Self, AttributeError> {
        Self::as_defined(max, min, max)
    }

    /// Sets the minimum value of the attribute.
    ///
    /// This will also set the current value to the minimum value if the current value is less than the new minimum value.
    ///
    /// # Errors
    ///
    /// Returns an error if the minimum value is greater than the maximum value.
    pub fn set_min(&mut self, min: f64) -> Result<(), AttributeError> {
        if min > self.max {
            return Err(AttributeError::DecimalMinGreaterThanMax(min, self.max));
        }

        self.min = min;
        self.current = self.current.max(min);

        Ok(())
    }

    /// Sets the maximum value of the attribute.
    ///
    /// This will also set the current value to the maximum value if the current value is greater than the new maximum value.
    ///
    /// # Errors
    ///
    /// Returns an error if the maximum value is less than the minimum value.
    pub fn set_max(&mut self, max: f64) -> Result<(), AttributeError> {
        if max < self.min {
            return Err(AttributeError::DecimalMaxLessThanMin(max, self.min));
        }

        self.max = max;
        self.current = self.current.min(max);

        Ok(())
    }

    /// Sets the minimum and maximum values of the attribute.
    ///
    /// This will also set the current value to the maximum value if the current value is greater than the new maximum value, and to the minimum value if the current value is
    /// less than the new minimum value.
    ///
    /// # Errors
    ///
    /// Returns an error if the minimum value is greater than the maximum value.
    pub fn set_min_max(&mut self, min: f64, max: f64) -> Result<(), AttributeError> {
        if min > max {
            return Err(AttributeError::DecimalMinGreaterThanMax(min, max));
        }

        if max < min {
            return Err(AttributeError::DecimalMaxLessThanMin(max, min));
        }

        self.min = min;
        self.max = max;
        self.current = self.current.clamp(min, max);

        Ok(())
    }

    /// Sets the current value of the attribute.
    ///
    /// If the new current value is less than the minimum value, it will be set to the minimum value. If it is greater than the maximum value, it will be set to the maximum value.
    pub fn set_current(&mut self, current: f64) {
        self.current = current.clamp(self.min, self.max);
    }

    /// Wrapper for `set_current`. Sets the current value of the attribute.
    ///
    /// If the new current value is less than the minimum value, it will be set to the minimum value. If it is greater than the maximum value, it will be set to the maximum value.
    pub fn set_value(&mut self, value: f64) {
        self.set_current(value);
    }

    /// Returns the current value of the attribute.
    #[must_use]
    pub fn current_value(&self) -> f64 {
        self.current.clamp(self.min, self.max)
    }

    /// Returns the current value of the attribute as a percentage of the maximum value.
    #[must_use]
    pub fn current_percentage(&self) -> f64 {
        // If min is != 0, adjust current and max until min *would be* 0
        let current = self.current - self.min;
        let max = self.max - self.min;

        if max == 0.0 {
            return 0.0;
        }

        current / max
    }
}

impl PartialEq<f64> for DecimalAttribute {
    fn eq(&self, other: &f64) -> bool {
        (self.current - *other).abs() < f64::EPSILON
    }
}

impl PartialEq<DecimalAttribute> for f64 {
    fn eq(&self, other: &DecimalAttribute) -> bool {
        (*self - other.current).abs() < Self::EPSILON
    }
}

/// Compare `DecimalAttribute` with `f32` for equality.
///
/// # Note
///
/// This is slightly less accurate than comparing a `DecimalAttribute` with a `f64`.
impl PartialEq<f32> for DecimalAttribute {
    #[allow(clippy::cast_lossless)]
    fn eq(&self, other: &f32) -> bool {
        (self.current - *other as f64).abs() < f64::EPSILON
    }
}

/// Compare `f32` with `DecimalAttribute` for equality.
///
/// # Note
///
/// This is slightly less accurate than comparing a `DecimalAttribute` with a `f64`.
impl PartialEq<DecimalAttribute> for f32 {
    #[allow(clippy::cast_lossless)]
    fn eq(&self, other: &DecimalAttribute) -> bool {
        (*self as f64 - other.current).abs() < f64::EPSILON
    }
}

/// Compare two `DecimalAttribute`s for equality. This only compares the `current` values.
impl PartialEq for DecimalAttribute {
    fn eq(&self, other: &Self) -> bool {
        (self.current - other.current).abs() < f64::EPSILON
    }
}

impl Eq for DecimalAttribute {}

impl std::hash::Hash for DecimalAttribute {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.current.to_bits().hash(state);
        self.min.to_bits().hash(state);
        self.max.to_bits().hash(state);
    }
}

impl std::fmt::Debug for DecimalAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DecimalAttribute")
            .field("current", &self.current)
            .field("min", &self.min)
            .field("max", &self.max)
            .finish()
    }
}

impl std::fmt::Display for DecimalAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:.2} ({:.2}%)",
            self.current,
            self.current_percentage() * 100.0
        )
    }
}

/// Allow conversion of `DecimalAttribute` to `f64`.
impl From<DecimalAttribute> for f64 {
    fn from(attribute: DecimalAttribute) -> Self {
        attribute.current
    }
}

/// Allow conversion of `DecimalAttribute` to `f32`.
///
/// # Note
///
/// This is slightly less accurate than converting a `DecimalAttribute` to a `f64`.
impl From<DecimalAttribute> for f32 {
    #[allow(clippy::cast_possible_truncation)]
    fn from(attribute: DecimalAttribute) -> Self {
        attribute.current as Self
    }
}

/// Allow f64 addition with `DecimalAttribute`.
impl std::ops::Add<f64> for DecimalAttribute {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            current: (self.current + rhs).clamp(self.min, self.max),
            min: self.min,
            max: self.max,
        }
    }
}

/// Allow f32 addition with `DecimalAttribute`.
///
/// # Note
///
/// This is slightly less accurate than adding a `f64` to a `DecimalAttribute`.
impl std::ops::Add<f32> for DecimalAttribute {
    type Output = Self;

    #[allow(clippy::cast_lossless)]
    fn add(self, rhs: f32) -> Self::Output {
        Self {
            current: (self.current + rhs as f64).clamp(self.min, self.max),
            min: self.min,
            max: self.max,
        }
    }
}

/// Allow f64 subtraction with `DecimalAttribute`.
impl std::ops::Sub<f64> for DecimalAttribute {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Self {
            current: (self.current - rhs).clamp(self.min, self.max),
            min: self.min,
            max: self.max,
        }
    }
}

/// Allow f32 subtraction with `DecimalAttribute`.
///
/// # Note
///
/// This is slightly less accurate than subtracting a `f64` from a `DecimalAttribute`.
impl std::ops::Sub<f32> for DecimalAttribute {
    type Output = Self;

    #[allow(clippy::cast_lossless)]
    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            current: (self.current - rhs as f64).clamp(self.min, self.max),
            min: self.min,
            max: self.max,
        }
    }
}

/// Allow f64 addition assignment with `DecimalAttribute`.
impl std::ops::AddAssign<f64> for DecimalAttribute {
    fn add_assign(&mut self, rhs: f64) {
        self.current = (self.current + rhs).clamp(self.min, self.max);
    }
}

/// Allow f32 addition assignment with `DecimalAttribute`.
///
/// # Note
///
/// This is slightly less accurate than adding a `f64` to a `DecimalAttribute`.
impl std::ops::AddAssign<f32> for DecimalAttribute {
    #[allow(clippy::cast_lossless)]
    fn add_assign(&mut self, rhs: f32) {
        self.current = (self.current + rhs as f64).clamp(self.min, self.max);
    }
}

/// Allow f64 subtraction assignment with `DecimalAttribute`.
impl std::ops::SubAssign<f64> for DecimalAttribute {
    fn sub_assign(&mut self, rhs: f64) {
        self.current = (self.current - rhs).clamp(self.min, self.max);
    }
}

/// Allow f32 subtraction assignment with `DecimalAttribute`.
///
/// # Note
///
/// This is slightly less accurate than subtracting a `f64` from a `DecimalAttribute`.
impl std::ops::SubAssign<f32> for DecimalAttribute {
    #[allow(clippy::cast_lossless)]
    fn sub_assign(&mut self, rhs: f32) {
        self.current = (self.current - rhs as f64).clamp(self.min, self.max);
    }
}

/// Allow multiplication of `DecimalAttribute` by `f64`.
impl std::ops::Mul<f64> for DecimalAttribute {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            current: (self.current * rhs).clamp(self.min, self.max),
            min: self.min,
            max: self.max,
        }
    }
}

/// Allow multiplication of `DecimalAttribute` by `f32`.
///
/// # Note
///
/// This is slightly less accurate than multiplying a `DecimalAttribute` by a `f64`.
impl std::ops::Mul<f32> for DecimalAttribute {
    type Output = Self;

    #[allow(clippy::cast_lossless)]
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            current: (self.current * rhs as f64).clamp(self.min, self.max),
            min: self.min,
            max: self.max,
        }
    }
}

/// Allow multiplication assignment of `DecimalAttribute` by `f64`.
impl std::ops::MulAssign<f64> for DecimalAttribute {
    fn mul_assign(&mut self, rhs: f64) {
        self.current = (self.current * rhs).clamp(self.min, self.max);
    }
}

/// Allow multiplication assignment of `DecimalAttribute` by `f32`.
///
/// # Note
///
/// This is slightly less accurate than multiplying a `DecimalAttribute` by a `f64`.
impl std::ops::MulAssign<f32> for DecimalAttribute {
    #[allow(clippy::cast_lossless)]
    fn mul_assign(&mut self, rhs: f32) {
        self.current = (self.current * rhs as f64).clamp(self.min, self.max);
    }
}

/// Allow division of `DecimalAttribute` by `f64`.
///
/// # Note
///
/// This will not allow division by zero.
impl std::ops::Div<f64> for DecimalAttribute {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            return self;
        }

        Self {
            current: (self.current / rhs).clamp(self.min, self.max),
            min: self.min,
            max: self.max,
        }
    }
}

/// Allow division of `DecimalAttribute` by `f32`.
///
/// # Note
///
/// This is slightly less accurate than dividing a `DecimalAttribute` by a `f64`.
///
/// This will not allow division by zero.
impl std::ops::Div<f32> for DecimalAttribute {
    type Output = Self;

    #[allow(clippy::cast_lossless)]
    fn div(self, rhs: f32) -> Self::Output {
        if rhs == 0.0 {
            return self;
        }

        Self {
            current: (self.current / rhs as f64).clamp(self.min, self.max),
            min: self.min,
            max: self.max,
        }
    }
}

/// Allow division assignment of `DecimalAttribute` by `f64`.
///
/// # Note
///
/// This will not allow division by zero.
impl std::ops::DivAssign<f64> for DecimalAttribute {
    fn div_assign(&mut self, rhs: f64) {
        if rhs == 0.0 {
            return;
        }

        self.current = (self.current / rhs).clamp(self.min, self.max);
    }
}

/// Allow division assignment of `DecimalAttribute` by `f32`.
///
/// # Note
///
/// This is slightly less accurate than dividing a `DecimalAttribute` by a `f64`.
///
/// This will not allow division by zero.
impl std::ops::DivAssign<f32> for DecimalAttribute {
    #[allow(clippy::cast_lossless)]
    fn div_assign(&mut self, rhs: f32) {
        if rhs == 0.0 {
            return;
        }

        self.current = (self.current / rhs as f64).clamp(self.min, self.max);
    }
}

/// Allow comparison of `DecimalAttribute` with `f64`.
impl std::cmp::PartialOrd<f64> for DecimalAttribute {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.current.partial_cmp(other)
    }
}

/// Allow comparison of `DecimalAttribute` with `f32`.
///
/// # Note
///
/// This is slightly less accurate than comparing a `DecimalAttribute` with a `f64`.
impl std::cmp::PartialOrd<f32> for DecimalAttribute {
    #[allow(clippy::cast_lossless)]
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        self.current.partial_cmp(&(*other as f64))
    }
}

/// Allow comparison of `DecimalAttribute` with `DecimalAttribute`.
///
/// # Note
///
/// This only compares the `current` values.
impl std::cmp::PartialOrd for DecimalAttribute {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.current.partial_cmp(&other.current)
    }
}

/// Allow negation of `DecimalAttribute`.
///
/// This will negate the `current` value, still clamping it to the `min` and `max` values.
impl std::ops::Neg for DecimalAttribute {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            current: (-self.current).clamp(self.min, self.max),
            min: self.min,
            max: self.max,
        }
    }
}

/// Allow calculating remainder of `DecimalAttribute` by `f64`. This assigns the remainder to the `current` value.
///
/// # Note
///
/// This will not allow division by zero.
impl std::ops::Rem<f64> for DecimalAttribute {
    type Output = Self;

    fn rem(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            return self;
        }

        Self {
            current: (self.current % rhs).clamp(self.min, self.max),
            min: self.min,
            max: self.max,
        }
    }
}

/// Allow calculating remainder of `DecimalAttribute` by `f32`. This assigns the remainder to the `current` value.
///
/// # Note
///
/// This is slightly less accurate than calculating the remainder of a `DecimalAttribute` by a `f64`.
///
/// This will not allow division by zero.
impl std::ops::Rem<f32> for DecimalAttribute {
    type Output = Self;

    #[allow(clippy::cast_lossless)]
    fn rem(self, rhs: f32) -> Self::Output {
        if rhs == 0.0 {
            return self;
        }

        Self {
            current: (self.current % rhs as f64).clamp(self.min, self.max),
            min: self.min,
            max: self.max,
        }
    }
}

/// Allow calculating remainder assignment of `DecimalAttribute` by `f64`. This assigns the remainder to the `current` value.
///
/// # Note
///
/// This will not allow division by zero.
impl std::ops::RemAssign<f64> for DecimalAttribute {
    fn rem_assign(&mut self, rhs: f64) {
        if rhs == 0.0 {
            return;
        }

        self.current = (self.current % rhs).clamp(self.min, self.max);
    }
}

/// Allow calculating remainder assignment of `DecimalAttribute` by `f32`. This assigns the remainder to the `current` value.
///
/// # Note
///
/// This is slightly less accurate than calculating the remainder of a `DecimalAttribute` by a `f64`.
///
/// This will not allow division by zero.
impl std::ops::RemAssign<f32> for DecimalAttribute {
    #[allow(clippy::cast_lossless)]
    fn rem_assign(&mut self, rhs: f32) {
        if rhs == 0.0 {
            return;
        }

        self.current = (self.current % rhs as f64).clamp(self.min, self.max);
    }
}

/// Range of `DecimalAttribute` values.
impl std::ops::RangeBounds<f64> for DecimalAttribute {
    fn start_bound(&self) -> std::ops::Bound<&f64> {
        std::ops::Bound::Included(&self.min)
    }

    fn end_bound(&self) -> std::ops::Bound<&f64> {
        std::ops::Bound::Included(&self.max)
    }
}
