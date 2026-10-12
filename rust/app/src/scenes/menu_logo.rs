use std::f32::consts::TAU;

use glam::{Vec2, Vec4};
use hecs::{Entity, World};

use crate::components::{Color, Layer, Light, Position, ScreenSpace, Shine, Size, Sprite};
use crate::render::TextureId;

const ENTER_SECONDS: f32 = 1.0;
const GLIDE_HOLD: f32 = 0.2;
const GLIDE_SECONDS: f32 = 0.9;
const DROP: f32 = 70.0;
const BOB: f32 = 4.0;
const BOB_PERIOD: f32 = 4.0;
const SHINE_START: f32 = 2.2;
const SHINE_PERIOD: f32 = 6.5;
const SHINE_SECONDS: f32 = 0.9;
const GLOW_PERIOD: f32 = 3.0;
const GLOW_BASE: f32 = 1.0;
const GLOW_SWING: f32 = 0.35;
const GLOW_RADIUS: f32 = 260.0;

#[derive(Debug, Clone, Copy)]
enum Entrance {
    Drop,
    Glide { from: Vec2, from_size: Vec2 },
}

pub(crate) struct LogoAnim {
    logo: Entity,
    glow: Entity,
    rest: Vec2,
    size: Vec2,
    entrance: Entrance,
    clock: f32,
}

fn ease_in_out(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t.clamp(0.0, 1.0)).powi(3)
}

impl LogoAnim {
    pub(crate) fn build(world: &mut World, texture: TextureId, rest: Vec2, size: Vec2) -> Self {
        let logo = world.spawn((
            Position(rest - Vec2::new(0.0, DROP)),
            Size(size),
            Color(Vec4::new(1.0, 1.0, 1.0, 0.0)),
            Sprite::whole(texture),
            Layer(0),
            ScreenSpace,
            Shine(0.0),
        ));
        let glow = world.spawn((
            Position(rest),
            Light::rgb(Vec2::ZERO, 240.0, 150.0, 60.0, GLOW_RADIUS, 0.0),
        ));
        Self {
            logo,
            glow,
            rest,
            size,
            entrance: Entrance::Drop,
            clock: 0.0,
        }
    }

    pub(crate) fn gliding(
        world: &mut World,
        texture: TextureId,
        rest: Vec2,
        size: Vec2,
        from: Vec2,
        from_size: Vec2,
    ) -> Self {
        let mut anim = Self::build(world, texture, rest, size);
        anim.entrance = Entrance::Glide { from, from_size };
        if let Ok((position, scale, color)) =
            world.query_one_mut::<(&mut Position, &mut Size, &mut Color)>(anim.logo)
        {
            position.0 = from;
            scale.0 = from_size;
            color.0.w = 1.0;
        }
        anim
    }

    pub(crate) fn entity(&self) -> Entity {
        self.logo
    }

    pub(crate) fn tick(&mut self, world: &mut World, camera: Vec2, dt: f32) {
        self.clock += dt;
        let (arrival, at, scale, alpha) = match self.entrance {
            Entrance::Drop => {
                let arrival = ease_out(self.clock / ENTER_SECONDS);
                let bob = (self.clock * TAU / BOB_PERIOD).sin() * BOB * arrival;
                (
                    arrival,
                    self.rest + Vec2::new(0.0, bob - (1.0 - arrival) * DROP),
                    self.size,
                    arrival,
                )
            }
            Entrance::Glide { from, from_size } => {
                let arrival = ease_in_out((self.clock - GLIDE_HOLD) / GLIDE_SECONDS);
                let bob = (self.clock * TAU / BOB_PERIOD).sin() * BOB * arrival;
                (
                    arrival,
                    from.lerp(self.rest, arrival) + Vec2::new(0.0, bob),
                    from_size.lerp(self.size, arrival),
                    1.0,
                )
            }
        };
        let sweep = self.clock - SHINE_START;
        let shine = if sweep >= 0.0 && sweep % SHINE_PERIOD < SHINE_SECONDS {
            ((sweep % SHINE_PERIOD) / SHINE_SECONDS).max(0.001)
        } else {
            0.0
        };
        if let Ok((position, size, color, glow)) =
            world.query_one_mut::<(&mut Position, &mut Size, &mut Color, &mut Shine)>(self.logo)
        {
            position.0 = at;
            size.0 = scale;
            color.0.w = alpha;
            glow.0 = shine;
        }
        if let Ok((position, light)) = world.query_one_mut::<(&mut Position, &mut Light)>(self.glow)
        {
            position.0 = camera + at;
            light.intensity =
                arrival * (GLOW_BASE + GLOW_SWING * (self.clock * TAU / GLOW_PERIOD).sin());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn logo() -> (World, LogoAnim) {
        let mut world = World::new();
        let anim = LogoAnim::build(
            &mut world,
            TextureId::WHITE,
            Vec2::new(480.0, 100.0),
            Vec2::new(228.0, 108.0),
        );
        (world, anim)
    }

    fn state(world: &World, anim: &LogoAnim) -> (Vec2, f32, f32) {
        let position = world
            .get::<&Position>(anim.logo)
            .map(|p| p.0)
            .unwrap_or_default();
        let alpha = world
            .get::<&Color>(anim.logo)
            .map(|c| c.0.w)
            .unwrap_or(-1.0);
        let shine = world.get::<&Shine>(anim.logo).map(|s| s.0).unwrap_or(-1.0);
        (position, alpha, shine)
    }

    #[test]
    fn the_logo_starts_invisible_above_its_place_and_drops_in() {
        let (mut world, mut anim) = logo();
        anim.tick(&mut world, Vec2::ZERO, 0.0);
        let (at, alpha, _) = state(&world, &anim);
        assert_eq!(alpha, 0.0);
        assert!(at.y < 100.0 - DROP + 1.0);

        anim.tick(&mut world, Vec2::ZERO, 0.5);
        let (halfway, alpha, _) = state(&world, &anim);
        assert!(alpha > 0.5 && alpha < 1.0);
        assert!(halfway.y > at.y && halfway.y < 100.0);

        anim.tick(&mut world, Vec2::ZERO, 1.0);
        let (settled, alpha, _) = state(&world, &anim);
        assert_eq!(alpha, 1.0);
        assert!((settled.y - 100.0).abs() <= BOB + 0.01);
    }

    #[test]
    fn once_settled_the_logo_only_floats_a_few_pixels() {
        let (mut world, mut anim) = logo();
        anim.tick(&mut world, Vec2::ZERO, 2.0);
        let mut low = f32::MAX;
        let mut high = f32::MIN;
        for _ in 0..600 {
            anim.tick(&mut world, Vec2::ZERO, 1.0 / 60.0);
            let (at, _, _) = state(&world, &anim);
            low = low.min(at.y);
            high = high.max(at.y);
        }
        assert!(
            high - low > BOB && high - low <= BOB * 2.0 + 0.01,
            "{}",
            high - low
        );
    }

    #[test]
    fn a_shine_crosses_the_logo_now_and_then_and_is_off_in_between() {
        let (mut world, mut anim) = logo();
        let mut active = 0;
        let mut idle = 0;
        for _ in 0..1800 {
            anim.tick(&mut world, Vec2::ZERO, 1.0 / 60.0);
            let (_, _, shine) = state(&world, &anim);
            assert!((0.0..=1.0).contains(&shine));
            if shine > 0.0 {
                active += 1;
            } else {
                idle += 1;
            }
        }
        assert!(active > 60 && idle > active);
    }

    #[test]
    fn the_warm_glow_follows_the_camera_and_pulses() {
        let (mut world, mut anim) = logo();
        anim.tick(&mut world, Vec2::new(500.0, 40.0), 2.0);
        let at = world
            .get::<&Position>(anim.glow)
            .map(|p| p.0)
            .unwrap_or_default();
        assert!(at.x > 900.0 && at.y > 100.0);
        let first = world
            .get::<&Light>(anim.glow)
            .map(|l| l.intensity)
            .unwrap_or(0.0);
        anim.tick(&mut world, Vec2::new(500.0, 40.0), 0.75);
        let second = world
            .get::<&Light>(anim.glow)
            .map(|l| l.intensity)
            .unwrap_or(0.0);
        assert!(first > 0.0 && (first - second).abs() > 0.05);
    }

    #[test]
    fn a_gliding_logo_starts_where_the_loading_title_was_and_settles_on_its_place() {
        let mut world = World::new();
        let mut anim = LogoAnim::gliding(
            &mut world,
            TextureId::WHITE,
            Vec2::new(480.0, 104.0),
            Vec2::new(228.0, 108.0),
            Vec2::new(480.0, 200.0),
            Vec2::new(285.0, 135.0),
        );
        let state = |world: &World, anim: &LogoAnim| {
            let at = world
                .get::<&Position>(anim.logo)
                .map(|p| p.0)
                .unwrap_or_default();
            let size = world
                .get::<&Size>(anim.logo)
                .map(|s| s.0)
                .unwrap_or_default();
            let alpha = world
                .get::<&Color>(anim.logo)
                .map(|c| c.0.w)
                .unwrap_or(-1.0);
            (at, size, alpha)
        };
        let (at, size, alpha) = state(&world, &anim);
        assert_eq!(
            (at, size, alpha),
            (Vec2::new(480.0, 200.0), Vec2::new(285.0, 135.0), 1.0)
        );

        anim.tick(&mut world, Vec2::ZERO, GLIDE_HOLD * 0.5);
        let (held, _, _) = state(&world, &anim);
        assert_eq!(held, Vec2::new(480.0, 200.0));

        anim.tick(
            &mut world,
            Vec2::ZERO,
            GLIDE_HOLD * 0.5 + GLIDE_SECONDS * 0.5,
        );
        let (middle, middle_size, alpha) = state(&world, &anim);
        assert!(middle.y < 200.0 && middle.y > 104.0, "{middle:?}");
        assert!(middle_size.x < 285.0 && middle_size.x > 228.0);
        assert_eq!(alpha, 1.0);

        anim.tick(&mut world, Vec2::ZERO, 2.0);
        let (settled, final_size, _) = state(&world, &anim);
        assert!((settled.y - 104.0).abs() <= BOB + 0.01, "{settled:?}");
        assert_eq!(final_size, Vec2::new(228.0, 108.0));
    }
}
