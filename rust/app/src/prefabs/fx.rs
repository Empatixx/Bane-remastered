use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::assets::Textures;
use crate::assets::tex;
use crate::components::{
    Animation, Casing, Clip, Color, Expiring, Floating, Layer, Light, LightFade, Lit, PoisonPool,
    PoisonTrail, Position, ScreenSpace, ShieldAura, Size, Sprite, Text, ThenPlay,
};
use crate::objects::{POOL_SECONDS, POOL_SIZE, TRAIL_DOT, TRAIL_SECONDS, TRAIL_VARIANTS};
use crate::render::{FontSize, TextureId};

const EXPLOSION_LIGHT: f32 = 1.5;
const CASING_SIZE: Vec2 = Vec2::new(12.0, 6.0);
const CASING_START: f32 = 14.0;
const CASING_MUZZLE: f32 = 10.0;

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

pub(crate) fn casing(
    world: &mut World,
    origin: Vec2,
    aim: Vec2,
    shell: bool,
    roll: [f32; 4],
    texture: TextureId,
) -> Entity {
    let forward = aim.normalize_or(Vec2::X);
    let side = Vec2::new(-forward.y, forward.x);
    let velocity =
        side * (70.0 + 50.0 * roll[0]) - forward * (30.0 * roll[1]) + Vec2::new(0.0, -10.0);
    world.spawn((
        Casing {
            velocity,
            height: CASING_START,
            rise: 120.0 + 90.0 * roll[2],
            angle: forward.to_angle(),
            spin: (roll[3] * 2.0 - 1.0).signum() * (10.0 + 12.0 * roll[3]),
            age: 0.0,
            settled: false,
        },
        Position(origin + forward * CASING_MUZZLE + side * 4.0),
        Size(CASING_SIZE),
        Color(Vec4::ONE),
        Sprite::cell(texture, 0, usize::from(shell), 1, 2),
        Layer(-1),
    ))
}

pub(crate) fn poison_dot(
    world: &mut World,
    center: Vec2,
    variant: usize,
    flipped: bool,
    textures: &Textures,
) -> Entity {
    let mut sprite = Sprite::cell(
        textures.get(tex::SNAKE_TRAIL),
        variant % TRAIL_VARIANTS,
        0,
        TRAIL_VARIANTS,
        1,
    );
    if flipped {
        sprite.uv = Sprite::flipped(sprite.uv);
    }
    world.spawn((
        PoisonPool {
            remaining: TRAIL_SECONDS,
            tick: 0.0,
        },
        PoisonTrail,
        Position(center),
        Size(TRAIL_DOT),
        Color(Vec4::ONE),
        sprite,
        Layer(0),
        Lit,
    ))
}

const WAVE_BANNER_SECONDS: f32 = 1.8;
const WAVE_BANNER_AT: Vec2 = Vec2::new(480.0, 110.0);

pub(crate) fn wave_banner(world: &mut World, number: usize, total: usize) -> Entity {
    world.spawn((
        Floating {
            remaining: WAVE_BANNER_SECONDS,
            velocity: Vec2::new(0.0, -6.0),
        },
        Position(WAVE_BANNER_AT),
        Text::centered(
            format!("Wave {number}/{total}"),
            Vec4::new(1.0, 0.85, 0.2, 1.0),
            FontSize::Size32,
        ),
        Layer(6),
        ScreenSpace,
    ))
}
