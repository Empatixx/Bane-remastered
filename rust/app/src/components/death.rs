#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DeathPart {
    Dim,
    Skull,
    Title,
    Icon(usize),
    Label(usize),
    Reward(usize),
    Prompt,
    Note,
}
