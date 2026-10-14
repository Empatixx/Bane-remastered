use crate::upgrades::Perks;

#[derive(Debug, Clone, Copy)]
pub(crate) struct PlayerPerks(pub(crate) Perks);

/// The die for the next hit the player takes, cast anew every tick from the combat stream of
/// the run; it decides whether the fraction of a hit Kevlar leaves still hurts.
#[derive(Debug, Clone, Copy)]
pub(crate) struct HitRoll(pub(crate) f32);
