use std::time::Duration;

const REPORT_EVERY: Duration = Duration::from_secs(1);

#[derive(Debug, Default)]
pub(crate) struct PerfCounter {
    frames: u32,
    updates: u32,
    elapsed: Duration,
}

impl PerfCounter {
    pub(crate) fn frame(&mut self) {
        self.frames += 1;
    }

    pub(crate) fn update(&mut self) {
        self.updates += 1;
    }

    pub(crate) fn advance(&mut self, dt: Duration) -> Option<(u32, u32)> {
        self.elapsed += dt;
        if self.elapsed < REPORT_EVERY {
            return None;
        }
        let seconds = self.elapsed.as_secs_f32();
        let report = (
            (self.frames as f32 / seconds).round() as u32,
            (self.updates as f32 / seconds).round() as u32,
        );
        *self = Self::default();
        Some(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_once_per_second_and_then_starts_over() {
        let mut counter = PerfCounter::default();
        for _ in 0..120 {
            counter.frame();
        }
        for _ in 0..60 {
            counter.update();
        }

        assert_eq!(counter.advance(Duration::from_millis(500)), None);
        assert_eq!(counter.advance(Duration::from_millis(500)), Some((120, 60)));
        assert_eq!(counter.advance(Duration::from_millis(500)), None);
    }
}
