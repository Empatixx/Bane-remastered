use glam::{Vec3, Vec4};
use hecs::World;

use crate::components::{Color, Interactable, Interaction, Light};

pub(crate) const PORTAL_LIGHT: Vec3 = Vec3::new(128.0, 0.0, 128.0);

pub(crate) fn tint_portal(world: &mut World, accent: Option<[f32; 3]>) {
    let light = accent.map_or(PORTAL_LIGHT, Vec3::from) / 255.0;
    let tint = accent.map_or(Vec4::ONE, |[red, green, blue]| {
        Vec4::new(
            0.5 + red / 510.0,
            0.5 + green / 510.0,
            0.5 + blue / 510.0,
            1.0,
        )
    });
    for (interactable, color, glow) in world.query_mut::<(&Interactable, &mut Color, &mut Light)>()
    {
        if interactable.action == Interaction::EnterPortal {
            color.0 = tint;
            glow.color = light;
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::assets::Textures;
    use crate::components::{Color, Light};
    use crate::prefabs;

    use super::*;

    #[test]
    fn the_portal_takes_the_accent_and_returns_to_purple_without_one() {
        let mut world = World::new();
        let portal = prefabs::portal(&mut world, Vec2::ZERO, &Textures::default());
        tint_portal(&mut world, Some([255.0, 0.0, 0.0]));
        let red = world.get::<&Light>(portal).expect("light").color;
        assert!(red.x > red.z);
        tint_portal(&mut world, None);
        assert_eq!(
            world.get::<&Light>(portal).expect("light").color,
            PORTAL_LIGHT / 255.0
        );
        assert_eq!(
            world.get::<&Color>(portal).expect("color").0,
            glam::Vec4::ONE
        );
    }
}
