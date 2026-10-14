//! One animation of a sprite sheet: which row, how many frames and how fast.

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Clip {
    pub(crate) row: usize,
    pub(crate) frames: usize,
    pub(crate) frame_time: f32,
    pub(crate) looping: bool,
    pub(crate) flipped: bool,
}
