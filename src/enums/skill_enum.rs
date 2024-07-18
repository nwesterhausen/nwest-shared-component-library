//! Skills which could be used to level up a character.
//!
//! These can be mapped to `Stat`s to increase the character's abilities.

use bevy_ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

/// Skills which could be used to level up a character. These are intended to then be used
/// to increase the character's abilities, or to unlock new abilities/spells/attacks.
///
/// These are derived from a latin or greek root word, followed by the suffix "-mancy" for
/// the school of magic that the skill belongs to. Other skills are not necessarily magic-based.
#[derive(Serialize, Deserialize, Clone, Copy, Component, Resource, PartialEq, Eq, Hash)]
pub enum Skill {
    /// Pyromancy is the school of fire magic.
    Pyromancy,
    /// Fulgomancy is the school of lightning magic.
    Fulgomancy,
    /// Hydromancy is the school of water magic.
    Hydromancy,
    /// Geomancy is the school of earth magic.
    Geomancy,
    /// Aeromancy is the school of air magic.
    Aeromancy,
    /// Cryomancy is the school of ice magic.
    Cryomancy,
    /// Trudomancy is the school of force magic.
    Trudomancy,
    /// Photomancy is the school of light magic.
    Photomancy,
    /// Umbramancy is the school of dark magic.
    Umbramancy,
    /// Arcanomancy is the school of arcane magic.
    Arcanomancy,
    /// Vitomancy is the school of life magic.
    Vitomancy,
    /// Mortomancy is the school of death magic.
    Mortomancy,
    /// Ampiliomancy is the school of enhancement magic.
    Ampiliomancy,
    /// Diminiomancy is the school of reduction magic.
    Diminiomancy,
    /// Citomancy is the school of summoning magic.
    Citomancy,
    /// Necromancy is the school of necromancy.
    Necromancy,
    /// Mutatiomancy is the school of polymorph magic.
    Mutatiomancy,
    /// Chronomancy is the school of time magic.
    Chronomancy,
}
