use glam::{Vec2, Vec3, Vec4};
use hecs::{Entity, World};

use crate::components::{
    Color, Interactable, Interaction, Layer, Light, Position, Size, Sprite, Text,
};
use crate::prefabs::PORTAL_LIGHT;
use crate::progress::ChallengeId;
use crate::render::{FontSize, TextureId};

/// A part of the sign under the portal naming the challenge the next run takes on.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PortalSign;

const SIGN_LINE: f32 = 22.0;
const SIGN_DROP: f32 = 26.0;
const SIGN_PADDING: Vec2 = Vec2::new(16.0, 8.0);
const SIGN_CHAR_WIDTH: f32 = 8.0;
const SIGN_BACKDROP: Vec4 = Vec4::new(0.0, 0.0, 0.0, 0.6);
const SIGN_LAYER: i32 = 4;

/// Writes the chosen challenge and its rules under the portal, or clears the sign.
/// The portal sits against the top wall, so the camera never shows what is above it.
pub(crate) fn sign_portal(world: &mut World, challenge: Option<ChallengeId>) {
    let old: Vec<Entity> = world
        .query::<Entity>()
        .with::<&PortalSign>()
        .iter()
        .collect();
    for entity in old {
        let _ = world.despawn(entity);
    }
    let Some(id) = challenge else {
        return;
    };
    let Some(bottom) = world
        .query::<(&Interactable, &Position, &Size)>()
        .iter()
        .find(|(interactable, _, _)| interactable.action == Interaction::EnterPortal)
        .map(|(_, position, size)| position.0 + Vec2::new(0.0, size.0.y / 2.0))
    else {
        return;
    };
    let spec = id.spec();
    // Halfway to white, so even the dark red accents read on the dark backdrop.
    let accent = (Vec3::from(spec.accent) / 255.0 + Vec3::ONE) / 2.0;
    let lines = [
        (
            format!("Challenge: {}", spec.title),
            accent.extend(1.0),
            FontSize::Size22,
        ),
        (
            spec.rules.describe().join(", "),
            Vec4::ONE,
            FontSize::Size16,
        ),
        (id.goal(), Vec4::ONE, FontSize::Size16),
    ];
    let first = bottom.y + SIGN_DROP;
    let widest = lines
        .iter()
        .map(|(text, _, _)| text.len())
        .max()
        .unwrap_or(0) as f32;
    let size = Vec2::new(
        widest * SIGN_CHAR_WIDTH,
        SIGN_LINE * (lines.len() - 1) as f32,
    ) + SIGN_PADDING * 2.0
        + Vec2::new(0.0, SIGN_LINE);
    world.spawn((
        PortalSign,
        Position(Vec2::new(
            bottom.x,
            first + SIGN_LINE * (lines.len() - 1) as f32 / 2.0,
        )),
        Size(size),
        Color(SIGN_BACKDROP),
        Sprite::whole(TextureId::WHITE),
        Layer(SIGN_LAYER - 1),
    ));
    for (index, (content, color, size)) in lines.into_iter().enumerate() {
        world.spawn((
            PortalSign,
            Position(Vec2::new(bottom.x, first + SIGN_LINE * index as f32)),
            Text::centered(content, color, size),
            Layer(SIGN_LAYER),
        ));
    }
}

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

    fn sign_lines(world: &World) -> Vec<String> {
        world
            .query::<&Text>()
            .with::<&PortalSign>()
            .iter()
            .map(|text| text.content.clone())
            .collect()
    }

    #[test]
    fn the_sign_over_the_portal_names_the_challenge_and_clears_without_one() {
        let mut world = World::new();
        prefabs::portal(&mut world, Vec2::new(0.0, 300.0), &Textures::default());
        sign_portal(&mut world, Some(ChallengeId::Horde));
        let lines = sign_lines(&world);
        assert!(lines.contains(&"Challenge: Horde".to_owned()));
        assert!(lines.contains(&"Enemy health +60%, Enemy damage +30%".to_owned()));

        sign_portal(&mut world, Some(ChallengeId::Bane));
        assert!(!sign_lines(&world).contains(&"Challenge: Horde".to_owned()));

        sign_portal(&mut world, None);
        assert!(sign_lines(&world).is_empty());
    }
}
