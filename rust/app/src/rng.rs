use std::cell::RefCell;

const SEED_VAR: &str = "BANE_SEED";

/// Independent sequences of one run, so that e.g. firing more shots never changes the loot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stream {
    Map,
    Loot,
    Combat,
}

#[derive(Debug, Clone)]
pub(crate) struct Rng {
    state: u64,
}

impl Default for Rng {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Rng {
    pub(crate) fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub(crate) fn from_clock() -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |elapsed| elapsed.as_nanos() as u64);
        Self::new(nanos)
    }

    /// The seed of a new run: `BANE_SEED` replays a run, otherwise the clock picks one.
    pub(crate) fn run_seed() -> u64 {
        std::env::var(SEED_VAR)
            .ok()
            .and_then(|seed| seed.trim().parse().ok())
            .unwrap_or_else(|| Self::from_clock().next_u64())
    }

    pub(crate) fn stream(run_seed: u64, floor: u32, stream: Stream) -> Self {
        let mut mixer = Self::new(run_seed ^ (u64::from(floor) << 8) ^ stream as u64);
        Self::new(mixer.next_u64())
    }

    pub(crate) fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    pub(crate) fn below(&mut self, bound: usize) -> usize {
        if bound == 0 {
            return 0;
        }
        (self.next_u64() % bound as u64) as usize
    }

    pub(crate) fn float(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }

    pub(crate) fn signed(&mut self) -> f32 {
        self.float() * 2.0 - 1.0
    }

    pub(crate) fn pick<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            None
        } else {
            items.get(self.below(items.len()))
        }
    }
}

/// Rounds a value with a fraction up as often as the fraction is large, so that e.g. +20%
/// damage on a 1-damage gun still adds a point every fifth shot. `roll` lies in `0..1`.
pub(crate) fn round_by_chance(value: f32, roll: f32) -> i32 {
    let nearest = value.round();
    if (value - nearest).abs() < SNAP {
        return nearest as i32;
    }
    let whole = value.floor();
    whole as i32 + i32::from(value - whole > roll)
}

/// Float noise of multiplying whole numbers by e.g. 1.2 must not count as a fraction.
const SNAP: f32 = 1e-4;

/// Randomness for looks and sounds only, it never changes how a run plays out.
pub(crate) fn cosmetic() -> f32 {
    thread_local! {
        static COSMETIC: RefCell<Rng> = RefCell::new(Rng::from_clock());
    }
    COSMETIC.with(|rng| rng.borrow_mut().float())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_same_seed_gives_the_same_sequence() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        for _ in 0..20 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn values_stay_inside_their_ranges() {
        let mut rng = Rng::new(7);
        for _ in 0..1000 {
            assert!(rng.below(5) < 5);
            let value = rng.float();
            assert!((0.0..1.0).contains(&value));
        }
        assert_eq!(rng.below(0), 0);
        assert!(rng.pick::<u8>(&[]).is_none());
        for _ in 0..1000 {
            assert!((-1.0..1.0).contains(&rng.signed()));
        }
    }

    #[test]
    fn a_fraction_rounds_up_as_often_as_it_is_large() {
        assert_eq!(round_by_chance(1.25, 0.1), 2);
        assert_eq!(round_by_chance(1.25, 0.9), 1);
        assert_eq!(round_by_chance(3.0, 0.0), 3);
        assert_eq!(round_by_chance(5.0 * 1.2, 0.0), 6);
        assert_eq!(round_by_chance(0.9, 0.95), 0);

        let mut rng = Rng::new(11);
        let total: i32 = (0..10_000).map(|_| round_by_chance(1.2, rng.float())).sum();
        assert!((11_800..=12_200).contains(&total), "{total}");
    }

    #[test]
    fn streams_repeat_for_the_same_run_and_differ_between_floors_and_kinds() {
        let mut first = Rng::stream(42, 1, Stream::Loot);
        let mut again = Rng::stream(42, 1, Stream::Loot);
        assert_eq!(first.next_u64(), again.next_u64());

        let loot = Rng::stream(42, 1, Stream::Loot).next_u64();
        assert_ne!(loot, Rng::stream(42, 2, Stream::Loot).next_u64());
        assert_ne!(loot, Rng::stream(42, 1, Stream::Combat).next_u64());
        assert_ne!(loot, Rng::stream(43, 1, Stream::Loot).next_u64());
    }
}
