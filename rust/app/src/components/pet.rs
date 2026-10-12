use hecs::Entity;

#[derive(Debug, Clone, Default)]
pub(crate) struct Pet {
    pub(crate) bite_cooldown: f32,
    pub(crate) loot: Option<Entity>,
    pub(crate) enemy: Option<Entity>,
    pub(crate) heading: Option<Entity>,
    pub(crate) stranded: bool,
    pub(crate) unreachable: Vec<Entity>,
}
