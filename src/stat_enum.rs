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

/// Defines the possible damage categories that an entity can have.
///
/// There are 5 general categories of damage: `All`, `Physical`, `Magical`, `True`, and `Mental`. These categories are used to
/// specify the broad buffs and debuffs applied to an entity. More specific damage categories can be used instead, like the various
/// types of elemental damage (`Fire`, `Ice`, etc.), or the various types of physical damage (`Slashing`, `Piercing`, etc.), or
/// even other magic types (`Summoning`, `Necromancy`, etc.).
#[derive(
    Serialize, Deserialize, Clone, Copy, Component, Resource, Default, PartialEq, Eq, Hash,
)]
pub enum TypeCategory {
    /// All represents all types of damage. It's the default value, since a `None` value is a special case.
    #[default]
    All,
    /// Physical damage is damage that is dealt by physical means, such as a sword or a punch.
    Physical,
    /// Magical damage is damage that is dealt by magical means, such as a spell or a potion.
    Magical,
    /// True damage is damage that is dealt by an unblockable means, such as a curse or other unique effects.
    True,
    /// Mental damage is damage directed towards the mind of an entity, such as a psychic attack or a fear spell.
    Mental,
    /// `None` is a special case that represents nothing. It is used for stats that are not affected by
    /// a specific damage category, but still are a `Stat::Complex`. `None` specifically has no interaction with
    /// any category.
    None,
    /// `Fire` is used for elemental fire damage.
    Fire,
    /// `Lightning` is used for elemental lightning damage.
    Lightning,
    /// `Water` is used for elemental water damage.
    Water,
    /// `Earth` is used for elemental earth damage.
    Earth,
    /// `Air` is used for elemental air damage.
    Air,
    /// `Ice` is used for elemental ice damage.
    Ice,
    /// `Force` is used for force damage.
    Force,
    /// `Light` is used for light (radiant) damage.
    Light,
    /// `Dark` is used for darkness damage.
    Dark,
    /// `Arcane` is used for arcane damage.
    Arcane,
    /// `Death` is used for death magic.
    Death,
    /// `Life` is used for life magic.
    Life,
    /// `Poison` is used for poison damage.
    Poison,
    /// `Enhancement` is used for enhancement magic.
    Enhancement,
    /// `Reduction` is used for reduction magic.
    Reduction,
    /// `Summoning` is used for summoning magic.
    Summoning,
    /// `Necromancy` is used for necromancy magic.
    Necromancy,
    /// `Polymorph` is used for polymorph magic.
    Polymorph,
    /// `Time` is used for time magic.
    Time,
    /// `Space` is used for space magic.
    Space,
    /// `Gravity` is used for gravity magic.
    Gravity,
    /// `Illusion` is used for illusion magic.
    Illusion,
    /// `Enchantment` is used for enchantment magic.
    Enchantment,
    /// `Curse` is used for curse magic.
    Curse,
    /// `Blessing` is used for blessings.
    Blessing,
    /// `Healing` is used for healing magic.
    Healing,
}

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
