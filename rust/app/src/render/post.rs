use bytemuck::{Pod, Zeroable};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct PostParams {
    pub(crate) blur: f32,
    pub(crate) darkness: f32,
    pub(crate) vignette: f32,
    pub(crate) time: f32,
    pub(crate) time_based: bool,
    pub(crate) time_slow: f32,
    pub(crate) clock: f32,
}

impl PostParams {
    pub(crate) fn blur_active(&self) -> bool {
        self.blur > 0.0
    }

    pub(crate) fn vignette_active(&self) -> bool {
        self.vignette > 0.0 || self.time_based
    }

    pub(crate) fn time_slow_active(&self) -> bool {
        self.time_slow > 0.0
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub(super) struct PostUniform {
    texel: [f32; 2],
    blur: f32,
    darkness: f32,
    vignette: f32,
    time: f32,
    time_based: f32,
    time_slow: f32,
    clock: f32,
    padding: [f32; 3],
}

impl PostUniform {
    pub(super) fn new(params: &PostParams, width: u32, height: u32) -> Self {
        Self {
            texel: [1.0 / width.max(1) as f32, 1.0 / height.max(1) as f32],
            blur: params.blur,
            darkness: 1.0 - params.darkness,
            vignette: params.vignette,
            time: params.time,
            time_based: if params.time_based { 1.0 } else { 0.0 },
            time_slow: params.time_slow,
            clock: params.clock,
            padding: [0.0; 3],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_uniform_has_the_size_the_shader_expects() {
        assert_eq!(size_of::<PostUniform>(), 48);
    }

    #[test]
    fn effects_are_active_only_when_they_have_something_to_show() {
        assert!(!PostParams::default().blur_active());
        assert!(!PostParams::default().vignette_active());
        assert!(PostParams {
            blur: 1.0,
            ..PostParams::default()
        }
        .blur_active());
        assert!(PostParams {
            time_based: true,
            ..PostParams::default()
        }
        .vignette_active());
        assert!(PostParams {
            time_slow: 0.5,
            ..PostParams::default()
        }
        .time_slow_active());
    }
}
