//! This module defines the possible stat modifiers that an entity can have.

use bevy_ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

use crate::traits::DescriptiveComponent;

/// Defines the possible stat modifiers that an entity can have.
#[derive(
    Serialize, Deserialize, Clone, Copy, Component, Resource, Default, PartialEq, Eq, Hash,
)]
pub enum StatModifier {
    /// No modifier. This is a default empty value. It allows the base stat to be used.
    #[default]
    None,
    /// `Reduction` represents a reduction in incoming values; for example, damage reduction. This can also be used for non-
    /// positive stats, like a speed reduction, or armor reduction.
    Reduction,
    /// `Regeneration` represents a passive increase in a value over time; for example, health regeneration.
    Regeneration,
    /// `Amplification` represents an increase in outgoing values; for example, attack amplification. This can also be used for
    /// non-positive stats, like damage amplification to increase incoming damage.
    Amplification,
    /// `Resistance` is a percentage-based reduction in incoming values; for example, damage resistance. This is applied before
    /// armor and defense.
    Resistance,
    /// `Speed` affects how quickly an entity can perform actions; for example, attack speed.
    Speed,
    /// `Size` affects the area affected by an entity; for example: projectile size or attack size.
    Size,
    /// `Lifetime` affects how long an entity exists; for example, the lifetime of a projectile.
    Lifetime,
    /// `Range` affects how far an entity can reach; for example, attack range or projectile range.
    Range,
    /// `Chance` affects the probability of an event occurring; for example, critical strike chance.
    Chance,
    /// `Duration` affects how long an effect lasts; for example, stun duration.
    Duration,
    /// `Cooldown` affects how long an entity must wait before performing an action again; for example, attack cooldown.
    Cooldown,
    /// `Cost` affects how much of a resource an entity must spend to perform an action; for example, mana cost. This is used
    /// with the `DamageCategory` to determine the type of cost (physical being stamina, magical being mana, etc.).
    Cost,
    /// `Penetration` affects how much of a resistance an entity can ignore; for example, armor penetration.
    Penetration,
    /// `Vampirism` affects how much of a value an entity can steal; for example, life steal.
    Vampirism,
}

impl DescriptiveComponent for StatModifier {
    fn name(&self) -> String {
        todo!()
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
