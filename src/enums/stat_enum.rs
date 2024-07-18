//! Module for the `Stat` enum.
//!
//! A `Stat` is a single descriptive value that measures some attribute for an entity. The stats here
//! are broken down into two categories: `BaseStat` and `ComplexStat`. A `BaseStat` is a simple stat
//! that has no additional properties. A `ComplexStat` is a stat that has additional properties, such
//! as a damage category and a stat modifier.
//!
//! To be the most accommodating, the `Stat` enum is designed to be as flexible as possible. This means
//! that it can represent a wide variety of stats, from simple stats like health and mana to complex
//! stats like damage resistance and damage amplification and even mental armor and physical evasion.

use bevy_ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

use crate::{traits::DescriptiveComponent, BaseStat, StatModifier, TypeCategory};

/// Defines the possible stats that an entity can have.
#[derive(
    Serialize, Deserialize, Clone, Copy, Component, Resource, Default, PartialEq, Eq, Hash,
)]
pub enum Stat {
    /// No stat. This is a default empty value.
    #[default]
    None,
    /// A simple stat that has no additional properties.
    ///
    /// This is used for stats like health, mana, stamina, etc. which are not modified by any other properties.
    Simple(BaseStat),
    /// A complex stat that has additional properties.
    ///
    /// Stats like `DamageResistance`, `DamageAmplification`, etc. are considered complex stats. They can be modified both
    /// by a type of damage and the specific modifier.
    Complex(BaseStat, TypeCategory, StatModifier),
}

impl DescriptiveComponent for Stat {
    fn name(&self) -> String {
        match self {
            Self::None => "None".to_string(),
            Self::Simple(stat) => stat.name(),
            Self::Complex(stat, category, modifier) => {
                format!("{} {} {}", stat.name(), category.name(), modifier.name())
            }
        }
    }

    fn description(&self) -> String {
        todo!()
    }

    fn value(&self) -> String {
        todo!()
    }

    fn percentage(&self) -> String {
        todo!()
    }
}
