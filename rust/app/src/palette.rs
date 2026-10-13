use glam::Vec4;

const fn rgb(red: u8, green: u8, blue: u8) -> Vec4 {
    Vec4::new(
        red as f32 / 255.0,
        green as f32 / 255.0,
        blue as f32 / 255.0,
        1.0,
    )
}

/// Labels, prompts and menu text.
pub(crate) const ORANGE: Vec4 = rgb(222, 112, 38);
/// Values, key names and plate rims.
pub(crate) const SALMON: Vec4 = rgb(225, 152, 116);
/// Speech and button text on light plates.
pub(crate) const RUST: Vec4 = rgb(200, 92, 18);
