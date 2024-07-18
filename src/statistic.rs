//! # Statistic
//!
//! This module contains the statistic struct and implementations.
//!
//! A statistic or stat is a single descriptive value that measures some attribute for an entity.
//! Some example stats might be attack, defense, speed, etc.
//!
//! Behind the scenes, a statistic holds a `DecimalAttribute` and a `Stat` enum.
use bevy_ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

use crate::{DecimalAttribute, Stat};

/// A statistic is a single descriptive value that measures some attribute for an entity.
#[derive(Serialize, Deserialize, Clone, Copy, Component, Resource, Default)]
pub struct Statistic {
    /// The value of the statistic.
    value: DecimalAttribute,
    /// The type of statistic.
    stat: Stat,
}

impl Statistic {
    /// Create a new statistic with the given value and stat.
    #[must_use]
    pub const fn new(value: DecimalAttribute, stat: Stat) -> Self {
        Self { value, stat }
    }

    /// Get the value of the statistic.
    #[must_use]
    pub const fn value(&self) -> DecimalAttribute {
        self.value
    }

    /// Get the type of statistic.
    #[must_use]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
}
