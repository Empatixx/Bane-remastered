use crate::assets::Textures;
use crate::audio::AudioCommand;
use crate::navigation::Navigation;
use crate::physics::Physics;

/// What every enemy-side fixed-step system needs besides the world. Built once per
/// `fixed_update`; `dt` is already scaled by the hourglass (`hostile_dt`).
pub(crate) struct Hostile<'a> {
    pub(crate) physics: &'a mut Physics,
    pub(crate) navigation: &'a Navigation,
    pub(crate) textures: &'a Textures,
    pub(crate) sounds: &'a mut Vec<AudioCommand>,
    pub(crate) dt: f32,
}

impl std::fmt::Debug for Hostile<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hostile").field("dt", &self.dt).finish()
    }
}

/// Runs one enemy-side tick in a test with default navigation and textures and a sound queue
/// nobody listens to.
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
    let mut hostile = Hostile {
        physics,
        navigation: &navigation,
        textures: &textures,
        sounds: &mut sounds,
        dt,
    };
    tick(world, &mut hostile);
}
