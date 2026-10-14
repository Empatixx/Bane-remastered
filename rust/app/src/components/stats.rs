use crate::enemies::EnemyKind;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct RunStats {
    pub(crate) fired: u32,
    pub(crate) hits: u32,
    pub(crate) kills: u32,
    pub(crate) items: u32,
    pub(crate) kills_of: [u32; EnemyKind::ALL.len()],
    pub(crate) gold: u32,
    pub(crate) hurt_this_floor: u32,
    pub(crate) untouched_floors: u32,
}

impl RunStats {
    pub(crate) fn kills_of(&self, kind: EnemyKind) -> u32 {
        self.kills_of[kind.index()]
    }
}
