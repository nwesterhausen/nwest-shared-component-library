//! This module defines the `TypeCategory` enum, which is used to define the possible damage categories that an entity can have.

use bevy_ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

use crate::traits::DescriptiveComponent;

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

impl DescriptiveComponent for TypeCategory {
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
