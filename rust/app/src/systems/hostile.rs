use hecs::Or;

use crate::assets::Textures;
use crate::audio::AudioCommand;
use crate::components::{Expiring, Frozen, Spawning};
use crate::navigation::Navigation;
use crate::physics::Physics;

/// A hostile that cannot act this tick: still rising (`Spawning`), dying (`Expiring`) or
/// frozen. Used as `.without::<Inactive>()` to query only the hostiles that are free to act.
pub(crate) type Inactive = Or<&'static Spawning, Or<&'static Expiring, &'static Frozen>>;

/// What every enemy-side fixed-step system needs besides the world. Built once per
/// `fixed_update`; `dt` is already scaled by the hourglass (`hostile_dt`).
pub(crate) struct Hostile<'a> {
    pub(crate) physics: &'a mut Physics,
    pub(crate) navigation: &'a Navigation,
    pub(crate) textures: &'a Textures,
    pub(crate) sounds: &'a mut Vec<AudioCommand>,
    /// The camera trauma of the hard hits on this tick.
    pub(crate) fx: &'a mut super::Fx,
    pub(crate) dt: f32,
}

impl std::fmt::Debug for Hostile<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hostile").field("dt", &self.dt).finish()
    }
}

/// Runs one enemy-side tick in a test with default navigation and textures, and a sound queue
/// and camera trauma nobody takes.
#[cfg(test)]
pub(crate) fn tick_for_test(
    world: &mut hecs::World,
    physics: &mut Physics,
    dt: f32,
    tick: fn(&mut hecs::World, &mut Hostile<'_>),
) {
    let navigation = Navigation::default();
    let textures = Textures::default();
    let mut sounds = Vec::new();
    let mut fx = super::Fx::default();
    let mut hostile = Hostile {
        physics,
        navigation: &navigation,
        textures: &textures,
        sounds: &mut sounds,
        fx: &mut fx,
        dt,
    };
    tick(world, &mut hostile);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_hostiles_that_are_not_rising_dying_or_frozen_are_active() {
        let mut world = hecs::World::new();
        let free = world.spawn((1_u8,));
        world.spawn((
            2_u8,
            Spawning {
                remaining: 1.0,
                total: 1.0,
            },
        ));
        world.spawn((3_u8, Expiring));
        world.spawn((4_u8, Frozen::new(1.0)));
        world.spawn((5_u8, Expiring, Frozen::new(1.0)));
        let active: Vec<hecs::Entity> = world
            .query::<hecs::Entity>()
            .with::<&u8>()
            .without::<Inactive>()
            .iter()
            .collect();
        assert_eq!(active, vec![free]);
    }
}
