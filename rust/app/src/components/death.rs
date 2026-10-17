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
    /// The Scientist's record of the run, under everything: the hero is subject 41 to him.
    Record,
}
