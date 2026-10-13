use crate::rng::Rng;

const THEME_SALT: u64 = 0x7E3A_11C5_9D42_B6F1;
const THEME_VAR: &str = "BANE_THEME";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Theme {
    Stone,
    Sewer,
    Moss,
    Spider,
    Ice,
    Lava,
    Temple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TrapMix {
    pub(crate) spikes: usize,
    pub(crate) flamethrowers: usize,
    pub(crate) explosive_barrels: usize,
    pub(crate) barrels: usize,
    pub(crate) pots: usize,
    pub(crate) poison_barrels: usize,
    pub(crate) frost_barrels: usize,
    pub(crate) cobwebs: usize,
    pub(crate) lava_pools: usize,
    pub(crate) ice_patches: usize,
    pub(crate) web_patches: usize,
    pub(crate) arrow_traps: usize,
}

const VARIED: [Theme; 6] = [
    Theme::Sewer,
    Theme::Moss,
    Theme::Spider,
    Theme::Ice,
    Theme::Lava,
    Theme::Temple,
];

impl Theme {
    pub(crate) fn parse(name: &str) -> Option<Self> {
        match name.trim().to_ascii_lowercase().as_str() {
            "stone" => Some(Self::Stone),
            "sewer" => Some(Self::Sewer),
            "moss" => Some(Self::Moss),
            "spider" => Some(Self::Spider),
            "ice" => Some(Self::Ice),
            "lava" => Some(Self::Lava),
            "temple" => Some(Self::Temple),
            _ => None,
        }
    }

    pub(crate) fn for_floor(run_seed: u64, floor: u32) -> Self {
        if let Some(theme) = std::env::var(THEME_VAR)
            .ok()
            .and_then(|name| Self::parse(&name))
        {
            return theme;
        }
        if floor <= 1 {
            return Self::Stone;
        }
        let mut rng = Rng::new(run_seed ^ THEME_SALT);
        let mut order = VARIED;
        for index in (1..order.len()).rev() {
            order.swap(index, rng.below(index + 1));
        }
        order[(floor as usize - 2) % order.len()]
    }

    pub(crate) fn mix(self) -> TrapMix {
        let (
            spikes,
            flamethrowers,
            explosive_barrels,
            barrels,
            pots,
            poison_barrels,
            frost_barrels,
        ) = match self {
            Self::Stone => (2, 1, 3, 2, 2, 2, 2),
            Self::Sewer => (1, 0, 1, 2, 2, 5, 1),
            Self::Moss => (3, 0, 1, 2, 3, 3, 1),
            Self::Spider => (1, 1, 1, 1, 1, 3, 1),
            Self::Ice => (2, 0, 1, 1, 1, 1, 5),
            Self::Lava => (2, 3, 5, 1, 1, 0, 0),
            Self::Temple => (4, 2, 2, 1, 3, 1, 1),
        };
        TrapMix {
            spikes,
            flamethrowers,
            explosive_barrels,
            barrels,
            pots,
            poison_barrels,
            frost_barrels,
            cobwebs: if self == Self::Spider { 5 } else { 2 },
            lava_pools: if self == Self::Lava { 3 } else { 0 },
            ice_patches: if self == Self::Ice { 3 } else { 0 },
            web_patches: if self == Self::Spider { 3 } else { 0 },
            arrow_traps: if self == Self::Temple { 2 } else { 0 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_floor_is_plain_stone_and_the_next_ones_never_repeat_a_theme() {
        for seed in 0..50 {
            assert_eq!(Theme::for_floor(seed, 1), Theme::Stone);
            let themes: Vec<Theme> = (2..=5).map(|floor| Theme::for_floor(seed, floor)).collect();
            for (index, theme) in themes.iter().enumerate() {
                assert_ne!(*theme, Theme::Stone);
                assert!(!themes[..index].contains(theme), "seed {seed}: {themes:?}");
            }
        }
    }

    #[test]
    fn theme_names_can_be_parsed_for_previews() {
        assert_eq!(Theme::parse(" Lava "), Some(Theme::Lava));
        assert_eq!(Theme::parse("ice"), Some(Theme::Ice));
        assert_eq!(Theme::parse("nonsense"), None);
    }

    #[test]
    fn a_run_always_gets_the_same_theme_for_a_floor() {
        assert_eq!(Theme::for_floor(7, 3), Theme::for_floor(7, 3));
    }

    #[test]
    fn the_themes_lean_on_their_own_hazards() {
        assert!(Theme::Lava.mix().flamethrowers > Theme::Stone.mix().flamethrowers);
        assert_eq!(Theme::Lava.mix().frost_barrels, 0);
        assert!(Theme::Ice.mix().frost_barrels > Theme::Stone.mix().frost_barrels);
        assert!(Theme::Sewer.mix().poison_barrels > Theme::Stone.mix().poison_barrels);
        assert!(Theme::Spider.mix().cobwebs > Theme::Stone.mix().cobwebs);
        assert!(Theme::Temple.mix().spikes > Theme::Stone.mix().spikes);
        assert!(Theme::Lava.mix().lava_pools > 0);
        assert!(Theme::Ice.mix().ice_patches > 0);
        assert!(Theme::Spider.mix().web_patches > 0);
        assert!(Theme::Temple.mix().arrow_traps > 0);
    }

    #[test]
    fn stone_keeps_the_original_hazard_counts() {
        let mix = Theme::Stone.mix();
        assert_eq!(
            (mix.spikes, mix.flamethrowers, mix.explosive_barrels),
            (2, 1, 3)
        );
        assert_eq!(
            (mix.barrels, mix.pots, mix.poison_barrels, mix.frost_barrels),
            (2, 2, 2, 2)
        );
        assert_eq!(mix.cobwebs, 2);
        assert_eq!(
            (
                mix.lava_pools,
                mix.ice_patches,
                mix.web_patches,
                mix.arrow_traps
            ),
            (0, 0, 0, 0)
        );
    }
}
