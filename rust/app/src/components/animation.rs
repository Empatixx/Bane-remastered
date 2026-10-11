#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Clip {
    pub(crate) row: usize,
    pub(crate) frames: usize,
    pub(crate) frame_time: f32,
    pub(crate) looping: bool,
    pub(crate) flipped: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Animation {
    pub(crate) columns: usize,
    pub(crate) rows: usize,
    clip: Clip,
    time: f32,
}

impl Animation {
    pub(crate) fn new(columns: usize, rows: usize, clip: Clip) -> Self {
        Self {
            columns,
            rows,
            clip,
            time: 0.0,
        }
    }

    pub(crate) fn play(&mut self, clip: Clip) {
        self.clip = clip;
        self.time = 0.0;
    }

    pub(crate) fn switch_to(&mut self, clip: Clip) {
        if self.clip != clip {
            self.play(clip);
        }
    }

    pub(crate) fn flipped(&self) -> bool {
        self.clip.flipped
    }

    pub(crate) fn advance(&mut self, dt: f32) {
        self.time += dt;
    }

    pub(crate) fn clip_frames(&self) -> usize {
        self.clip.frames
    }

    pub(crate) fn row(&self) -> usize {
        self.clip.row
    }

    pub(crate) fn frame(&self) -> usize {
        let elapsed = (self.time / self.clip.frame_time) as usize;
        if self.clip.looping {
            elapsed % self.clip.frames
        } else {
            elapsed.min(self.clip.frames - 1)
        }
    }

    pub(crate) fn finished(&self) -> bool {
        !self.clip.looping && self.time >= self.clip.frames as f32 * self.clip.frame_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLIP: Clip = Clip {
        row: 0,
        frames: 4,
        frame_time: 0.1,
        looping: false,
        flipped: false,
    };

    #[test]
    fn plays_frames_in_order_and_holds_the_last_one() {
        let mut animation = Animation::new(4, 2, CLIP);
        assert_eq!(animation.frame(), 0);
        animation.advance(0.25);
        assert_eq!(animation.frame(), 2);
        assert!(!animation.finished());
        animation.advance(1.0);
        assert_eq!(animation.frame(), 3);
        assert!(animation.finished());
    }

    #[test]
    fn looping_clip_wraps_and_never_finishes() {
        let mut animation = Animation::new(
            4,
            2,
            Clip {
                looping: true,
                ..CLIP
            },
        );
        animation.advance(0.45);
        assert_eq!(animation.frame(), 0);
        assert!(!animation.finished());
    }
}
