#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct RunStats {
    pub(crate) fired: u32,
    pub(crate) hits: u32,
    pub(crate) kills: u32,
    pub(crate) items: u32,
}
