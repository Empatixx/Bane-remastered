use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::tex;
use crate::assets::Textures;
use crate::components::{
    Animation, Clip, Color, Expiring, Floating, Layer, Light, LightFade, Lit, PoisonPool, Position,
    ShieldAura, Size, Sprite, Text, ThenPlay,
};
use crate::objects::{POOL_SECONDS, POOL_SIZE};
use crate::render::FontSize;

const EXPLOSION_LIGHT: f32 = 1.5;

const SHIELD_SIZE: Vec2 = Vec2::new(96.0, 144.0);
const FLOAT_SECONDS: f32 = 0.5;
const FLOAT_RISE: f32 = 30.0;

pub(crate) fn poison_pool(world: &mut World, center: Vec2, textures: &Textures) -> Entity {
    let spawn = Clip {
        row: 0,
        frames: 6,
        frame_time: 0.07,
        looping: false,
        flipped: false,
    };
    let idle = Clip {
        row: 1,
        frames: 6,
        frame_time: 0.14,
        looping: true,
        flipped: false,
    };
    world.spawn((
        PoisonPool {
            remaining: POOL_SECONDS,
            tick: 0.0,
        },
        Position(center),
        Size(POOL_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(tex::POISON_PUDDLE), 0, 0, 6, 2),
        Animation::new(6, 2, spawn),
        ThenPlay(idle),
        Layer(0),
        Lit,
    ))
}

pub(crate) fn frost_burst(world: &mut World, center: Vec2, textures: &Textures) {
    let clip = Clip {
        row: 0,
        frames: 9,
        frame_time: 0.07,
        looping: false,
        flipped: false,
    };
    world.spawn((
        Expiring,
        Position(center),
        Size(Vec2::new(224.0, 224.0)),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(tex::FROST_BURST), 0, 0, 9, 1),
        Animation::new(9, 1, clip),
        Layer(3),
        Light::rgb(Vec2::ZERO, 90.0, 170.0, 255.0, 200.0, EXPLOSION_LIGHT),
        LightFade {
            base: EXPLOSION_LIGHT,
        },
    ));
}

pub(crate) fn explosion(world: &mut World, center: Vec2, textures: &Textures) {
    let clip = Clip {
        row: 0,
        frames: 9,
        frame_time: 0.08,
        looping: false,
        flipped: false,
    };
    let size = Vec2::new(128.0, 128.0);
    world.spawn((
        Expiring,
        Position(center),
        Size(size),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(tex::BARREL_EXPLOSION), 0, 0, 9, 1),
        Animation::new(9, 1, clip),
        Layer(3),
        Lit,
        Light::rgb(Vec2::ZERO, 215.0, 40.0, 40.0, 150.0, EXPLOSION_LIGHT),
        LightFade {
            base: EXPLOSION_LIGHT,
        },
    ));
}

pub(crate) fn shield_aura(world: &mut World, center: Vec2, textures: &Textures) -> Entity {
    let clip = Clip {
        row: 0,
        frames: 4,
        frame_time: 0.1,
        looping: true,
        flipped: false,
    };
    world.spawn((
        ShieldAura,
        Position(center),
        Size(SHIELD_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(textures.get(tex::ARTEFACTS_MAGICSHIELD), 0, 0, 4, 1),
        Animation::new(4, 1, clip),
        Layer(4),
        Lit,
    ))
}

pub(crate) fn floating_number(world: &mut World, change: i32, position: Vec2) -> Entity {
    let (content, color) = if change > 0 {
        (format!("+{change}"), Vec4::new(0.0, 1.0, 0.0, 1.0))
    } else {
        (change.abs().to_string(), Vec4::new(1.0, 0.0, 0.0, 1.0))
    };
    world.spawn((
        Floating {
            remaining: FLOAT_SECONDS,
            velocity: Vec2::new(0.0, -FLOAT_RISE / FLOAT_SECONDS),
        },
        Position(position),
        Text::new(content, color, FontSize::Size22),
        Layer(4),
    ))
}
