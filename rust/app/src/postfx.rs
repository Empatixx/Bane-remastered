use std::f32::consts::TAU;

use crate::render::PostParams;

const MAX_BLUR: f32 = 5.0;
const BLUR_STEP: f32 = 5.0;
const MAX_DARKNESS: f32 = 0.3;
const DARKNESS_STEP: f32 = 0.5;
const VIGNETTE_TWEEN: f32 = 2.0;
const PULSE_PERIOD: f32 = 3.0;
const SLOW_IN_SECONDS: f32 = 0.4;
const SLOW_OUT_SECONDS: f32 = 0.6;
const CLOCK_PERIOD: f32 = 600.0;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct SceneFx {
    pub(crate) paused: bool,
    pub(crate) low_health: bool,
    pub(crate) time_slow: f32,
}

#[derive(Debug, Default)]
pub(crate) struct PostFx {
    blur: f32,
    darkness: f32,
    vignette: f32,
    time_based: bool,
    time_slow: f32,
}

impl PostFx {
    pub(crate) fn update(&mut self, dt: f32, fx: SceneFx, hit: bool) {
        if fx.paused {
            self.blur = (self.blur + BLUR_STEP * dt).min(MAX_BLUR);
            self.darkness = (self.darkness + DARKNESS_STEP * dt).min(MAX_DARKNESS);
        } else {
            self.blur = 0.0;
            self.darkness = 0.0;
        }
        self.time_based = fx.low_health;
        self.time_slow = if fx.time_slow > self.time_slow {
            (self.time_slow + dt / SLOW_IN_SECONDS).min(fx.time_slow)
        } else {
            (self.time_slow - dt / SLOW_OUT_SECONDS).max(fx.time_slow)
        };
        if hit {
            self.vignette = 1.0;
        } else if !self.time_based {
            self.vignette -= self.vignette * dt * VIGNETTE_TWEEN;
            if self.vignette < 0.001 {
                self.vignette = 0.0;
            }
        }
    }

    pub(crate) fn params(&self, time: f32) -> PostParams {
        PostParams {
            blur: self.blur,
            darkness: self.darkness,
            vignette: self.vignette,
            time: (time % PULSE_PERIOD) * TAU / PULSE_PERIOD,
            time_based: self.time_based,
            time_slow: self.time_slow,
            clock: time % CLOCK_PERIOD,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAUSED: SceneFx = SceneFx {
        paused: true,
        low_health: false,
        time_slow: 0.0,
    };

    #[test]
    fn pausing_ramps_the_blur_up_to_the_maximum_and_unpausing_resets_it() {
        let mut fx = PostFx::default();
        fx.update(0.5, PAUSED, false);
        assert!((fx.params(0.0).blur - 2.5).abs() < 0.001);
        fx.update(5.0, PAUSED, false);
        assert_eq!(fx.params(0.0).blur, MAX_BLUR);
        assert!((fx.params(0.0).darkness - MAX_DARKNESS).abs() < 0.001);

        fx.update(0.016, SceneFx::default(), false);
        assert!(!fx.params(0.0).blur_active());
    }

    #[test]
    fn a_hit_flashes_the_vignette_which_then_fades_out() {
        let mut fx = PostFx::default();
        fx.update(0.016, SceneFx::default(), true);
        assert_eq!(fx.params(0.0).vignette, 1.0);
        for _ in 0..600 {
            fx.update(0.016, SceneFx::default(), false);
        }
        assert!(!fx.params(0.0).vignette_active());
    }

    #[test]
    fn low_health_keeps_the_pulsing_vignette_on() {
        let mut fx = PostFx::default();
        let low = SceneFx {
            paused: false,
            low_health: true,
            time_slow: 0.0,
        };
        for _ in 0..300 {
            fx.update(0.016, low, false);
        }
        assert!(fx.params(0.0).vignette_active());
    }

    #[test]
    fn the_time_slow_effect_ramps_in_quickly_and_out_more_slowly() {
        let mut fx = PostFx::default();
        let slowed = SceneFx {
            time_slow: 1.0,
            ..SceneFx::default()
        };
        fx.update(SLOW_IN_SECONDS / 2.0, slowed, false);
        assert!((fx.params(0.0).time_slow - 0.5).abs() < 0.001);
        fx.update(1.0, slowed, false);
        assert_eq!(fx.params(0.0).time_slow, 1.0);
        assert!(fx.params(0.0).time_slow_active());

        fx.update(SLOW_OUT_SECONDS / 2.0, SceneFx::default(), false);
        assert!((fx.params(0.0).time_slow - 0.5).abs() < 0.001);
        fx.update(1.0, SceneFx::default(), false);
        assert_eq!(fx.params(0.0).time_slow, 0.0);
        assert!(!fx.params(0.0).time_slow_active());
    }
}
