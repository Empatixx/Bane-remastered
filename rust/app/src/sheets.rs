//! Sprite-sheet layouts shared by the prefabs that spawn an entity and the systems that animate it.

pub(crate) mod player {
    use crate::clip::Clip;

    const FRAME_TIME: f32 = 0.1;
    const FRAMES: usize = 6;

    pub(crate) const COLUMNS: usize = 6;
    pub(crate) const ROWS: usize = 4;

    pub(crate) const IDLE: Clip = clip(0, false);
    pub(crate) const RIGHT: Clip = clip(1, false);
    pub(crate) const LEFT: Clip = clip(1, true);
    pub(crate) const BOTTOM: Clip = clip(2, false);
    pub(crate) const TOP: Clip = clip(3, false);

    const fn clip(row: usize, flipped: bool) -> Clip {
        Clip {
            row,
            frames: FRAMES,
            frame_time: FRAME_TIME,
            looping: true,
            flipped,
        }
    }
}

pub(crate) mod pet {
    use crate::clip::Clip;

    const MOVE_FRAME_TIME: f32 = 0.1;
    const IDLE_FRAME_TIME: f32 = 0.25;

    pub(crate) const COLUMNS: usize = 4;
    pub(crate) const ROWS: usize = 4;

    pub(crate) const IDLE: Clip = clip(3, 2, IDLE_FRAME_TIME, false);
    pub(crate) const BOTTOM: Clip = clip(0, 4, MOVE_FRAME_TIME, false);
    pub(crate) const TOP: Clip = clip(1, 4, MOVE_FRAME_TIME, false);
    pub(crate) const RIGHT: Clip = clip(2, 4, MOVE_FRAME_TIME, false);
    pub(crate) const LEFT: Clip = clip(2, 4, MOVE_FRAME_TIME, true);

    const fn clip(row: usize, frames: usize, frame_time: f32, flipped: bool) -> Clip {
        Clip {
            row,
            frames,
            frame_time,
            looping: true,
            flipped,
        }
    }
}

pub(crate) mod infinity {
    use crate::clip::Clip;

    pub(crate) const COLUMNS: usize = 4;
    pub(crate) const ROWS: usize = 1;

    pub(crate) const GLINT: Clip = Clip {
        row: 0,
        frames: 4,
        frame_time: 0.18,
        looping: true,
        flipped: false,
    };
}

pub(crate) mod slot {
    use crate::clip::Clip;

    pub(crate) const COLUMNS: usize = 6;
    pub(crate) const ROWS: usize = 3;

    pub(crate) const SPIN: Clip = Clip {
        row: 0,
        frames: 4,
        frame_time: 0.1,
        looping: true,
        flipped: false,
    };
    pub(crate) const WIN: Clip = Clip {
        row: 1,
        frames: 6,
        frame_time: 0.1,
        looping: true,
        flipped: false,
    };
    pub(crate) const LOSE: Clip = Clip {
        row: 2,
        frames: 1,
        frame_time: 1.0,
        looping: true,
        flipped: false,
    };
    pub(crate) const IDLE: Clip = LOSE;
}

pub(crate) mod ghost {
    use crate::clip::Clip;

    pub(crate) const COLUMNS: usize = 4;
    pub(crate) const ROWS: usize = 2;

    pub(crate) const FLOAT: Clip = Clip {
        row: 0,
        frames: 4,
        frame_time: 0.15,
        looping: true,
        flipped: false,
    };
    pub(crate) const HIT: Clip = Clip {
        row: 1,
        frames: 3,
        frame_time: 0.07,
        looping: false,
        flipped: false,
    };
}

pub(crate) mod turret {
    use crate::clip::Clip;

    pub(crate) const COLUMNS: usize = 5;
    pub(crate) const ROWS: usize = 7;
    pub(crate) const RISE_SECONDS: f32 = 0.5;
    pub(crate) const CRUMBLE_SECONDS: f32 = 0.6;
    pub(crate) const RECOIL_SECONDS: f32 = 0.15;
    pub(crate) const AIM_ROWS: usize = 5;

    pub(crate) const RISE: Clip = Clip {
        row: 0,
        frames: 5,
        frame_time: RISE_SECONDS / 5.0,
        looping: false,
        flipped: false,
    };
    pub(crate) const CRUMBLE: Clip = Clip {
        row: 1,
        frames: 5,
        frame_time: CRUMBLE_SECONDS / 5.0,
        looping: false,
        flipped: false,
    };

    pub(crate) const fn idle(aim: usize, flipped: bool) -> Clip {
        Clip {
            row: 2 + aim,
            frames: 1,
            frame_time: 1.0,
            looping: true,
            flipped,
        }
    }

    pub(crate) const fn recoil(aim: usize, flipped: bool) -> Clip {
        Clip {
            row: 2 + aim,
            frames: 3,
            frame_time: RECOIL_SECONDS / 3.0,
            looping: false,
            flipped,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clip::Clip;

    fn looping(row: usize, frames: usize, frame_time: f32, flipped: bool) -> Clip {
        Clip {
            row,
            frames,
            frame_time,
            looping: true,
            flipped,
        }
    }

    #[test]
    fn the_player_sheet_keeps_its_rows() {
        assert_eq!((player::COLUMNS, player::ROWS), (6, 4));
        assert_eq!(player::IDLE, looping(0, 6, 0.1, false));
        assert_eq!(player::RIGHT, looping(1, 6, 0.1, false));
        assert_eq!(player::LEFT, looping(1, 6, 0.1, true));
        assert_eq!(player::BOTTOM, looping(2, 6, 0.1, false));
        assert_eq!(player::TOP, looping(3, 6, 0.1, false));
    }

    #[test]
    fn the_pet_sheet_keeps_its_rows() {
        assert_eq!((pet::COLUMNS, pet::ROWS), (4, 4));
        assert_eq!(pet::IDLE, looping(3, 2, 0.25, false));
        assert_eq!(pet::BOTTOM, looping(0, 4, 0.1, false));
        assert_eq!(pet::TOP, looping(1, 4, 0.1, false));
        assert_eq!(pet::RIGHT, looping(2, 4, 0.1, false));
        assert_eq!(pet::LEFT, looping(2, 4, 0.1, true));
    }

    #[test]
    fn the_slot_machine_sheet_keeps_its_rows_and_rests_on_a_loss() {
        assert_eq!((slot::COLUMNS, slot::ROWS), (6, 3));
        assert_eq!(slot::SPIN, looping(0, 4, 0.1, false));
        assert_eq!(slot::WIN, looping(1, 6, 0.1, false));
        assert_eq!(slot::LOSE, looping(2, 1, 1.0, false));
        assert_eq!(slot::IDLE, slot::LOSE);
    }
}
