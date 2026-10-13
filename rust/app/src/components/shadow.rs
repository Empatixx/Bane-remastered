/// A silhouette on the floor that turns away from the strongest light nearby.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DropShadow {
    pub(crate) angle: f32,
    pub(crate) intensity: f32,
    pub(crate) length: f32,
}

impl Default for DropShadow {
    fn default() -> Self {
        Self {
            angle: 0.0,
            intensity: 0.0,
            length: 20.0,
        }
    }
}
