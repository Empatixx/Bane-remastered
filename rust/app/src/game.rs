#[derive(Debug, Default)]
pub(crate) struct Game {
    ticks: u64,
}

impl Game {
    pub(crate) fn fixed_update(&mut self) {
        self.ticks += 1;
        if self.ticks.is_multiple_of(60) {
            tracing::info!(ticks = self.ticks, "one second passed");
        }
    }
}
