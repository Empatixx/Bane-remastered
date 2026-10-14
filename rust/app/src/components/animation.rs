pub(crate) use crate::clip::Clip;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Animation {
    pub(crate) columns: usize,
    pub(crate) rows: usize,
    clip: Clip,
    time: f32,
    held: Option<usize>,
    ping_pong: bool,
}

impl Animation {
    pub(crate) fn new(columns: usize, rows: usize, clip: Clip) -> Self {
        Self {
            columns,
            rows,
            clip,
            time: 0.0,
            held: None,
            ping_pong: false,
        }
    }

    pub(crate) fn ping_pong(mut self) -> Self {
        self.ping_pong = true;
        self
    }

    pub(crate) fn play(&mut self, clip: Clip) {
        self.clip = clip;
        self.time = 0.0;
        self.held = None;
    }

    pub(crate) fn hold(&mut self, frame: usize) {
        self.held = Some(frame);
    }

    pub(crate) fn switch_to(&mut self, clip: Clip) {
        if self.clip != clip {
            self.play(clip);
        }
    }

    pub(crate) fn face(&mut self, flipped: bool) {
        self.clip.flipped = flipped;
    }

    pub(crate) fn clip(&self) -> Clip {
        self.clip
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
        if let Some(frame) = self.held {
            return frame;
        }
        let elapsed = (self.time / self.clip.frame_time) as usize;
        if self.clip.looping && self.ping_pong {
            let step = elapsed % (self.clip.frames * 2);
            if step < self.clip.frames {
                step
            } else {
                self.clip.frames * 2 - 1 - step
            }
        } else if self.clip.looping {
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
    fn a_ping_pong_clip_plays_forwards_and_then_backwards() {
        let mut animation = Animation::new(
            4,
            2,
            Clip {
                looping: true,
                ..CLIP
            },
        )
        .ping_pong();
        let mut frames = Vec::new();
        for _ in 0..10 {
            frames.push(animation.frame());
            animation.advance(0.1);
        }
        assert_eq!(frames, vec![0, 1, 2, 3, 3, 2, 1, 0, 0, 1]);
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
