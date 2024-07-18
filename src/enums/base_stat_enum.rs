//! Defines the base stats that an entity can have.

use bevy_ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

use crate::traits::DescriptiveComponent;

/// Defines the base stats that an entity can have.
///
/// This should be the very root of a stat, like in the case of `DamageResistance` or `DamageAmplification`, where the base
/// stat is `Damage`. In the `Stat` enum, use the `DescriptiveComponent` trait to get the full name of the stat, and a description.
///
/// If any additional "base" stats are needed, they should be added here.
#[derive(
    Serialize, Deserialize, Clone, Copy, Component, Resource, Default, PartialEq, Eq, Hash,
)]
pub enum BaseStat {
    /// No stat. This is a default empty value.
    #[default]
    None,
    /// `Health` represents the life of an entity, typically measured in hit points (HP).
    Health,
    /// `Mana` represents the magical energy of an entity, typically used to cast spells.
    Mana,
    /// `Stamina` represents the physical energy of an entity, typically used to perform physical actions.
    Stamina,
    /// `Attack` represents the damage that an entity can deal.
    Attack,
    /// `Damage` represents the damage that an entity receives.
    Damage,
    /// `Defense` represents the ability of an entity to resist damage.
    Defense,
    /// `Speed` represents the movement speed of an entity.
    Speed,
    /// `CriticalStrike` represents the chance of an entity to deal critical damage.
    CriticalStrike,
    /// `Armor` represents the physical resistance of an entity.
    Armor,
    /// `Evasion` represents the ability of an entity to avoid attacks. It could be considered a chance to dodge.
    Evasion,
    /// `Accuracy` represents the ability of an entity to hit a target. It could be considered a chance to hit.
    Accuracy,
    /// `Stun` represents the ability of an entity to stun a target.
    Stun,
    /// `Silence` represents the ability of an entity to silence a target.
    Silence,
    /// `Slow` represents the ability of an entity to slow a target.
    Slow,
    /// `Root` represents the ability of an entity to root a target.
    Root,
    /// `Fear` represents the ability of an entity to fear a target.
    Fear,
    /// `Charm` represents the ability of an entity to charm a target.
    Charm,
    /// `Taunt` represents the ability of an entity to taunt a target.
    Taunt,
    /// `Knockback` represents the ability of an entity to knockback a target.
    Knockback,
    /// `Projectile` describes details about the projectiles of an entity.
    Projectile,
}

impl DescriptiveComponent for BaseStat {
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
