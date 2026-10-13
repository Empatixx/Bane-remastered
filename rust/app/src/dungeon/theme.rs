use crate::rng::Rng;

use super::surface::Surface;

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
    pub(crate) geysers: usize,
    pub(crate) ice_patches: usize,
    pub(crate) web_patches: usize,
    pub(crate) arrow_traps: usize,
    pub(crate) icicles: usize,
    pub(crate) crystals: usize,
    pub(crate) frozen_skulls: usize,
    pub(crate) cocoons: usize,
}

const FIRST_FLOOR: Theme = Theme::Spider;

const LATER: [Theme; 6] = [
    Theme::Stone,
    Theme::Sewer,
    Theme::Moss,
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
            return FIRST_FLOOR;
        }
        let mut rng = Rng::new(run_seed ^ THEME_SALT);
        let mut order = LATER;
        for index in (1..order.len()).rev() {
            order.swap(index, rng.below(index + 1));
        }
        order[(floor as usize - 2) % order.len()]
    }

    pub(crate) fn surface(self) -> Option<Surface> {
        match self {
            Self::Ice => Some(Surface::Ice),
            Self::Lava => Some(Surface::Lava),
            Self::Sewer => Some(Surface::Water),
            _ => None,
        }
    }

    pub(crate) fn backdrop(self) -> [u8; 3] {
        match self {
            Self::Stone => [37, 19, 26],
            Self::Sewer => [16, 26, 22],
            Self::Moss => [22, 24, 16],
            Self::Spider => [14, 10, 16],
            Self::Ice => [14, 24, 40],
            Self::Lava => [20, 8, 8],
            Self::Temple => [36, 24, 18],
        }
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
            geysers: 0,
            ice_patches: 0,
            web_patches: if self == Self::Spider { 3 } else { 0 },
            arrow_traps: if self == Self::Temple { 2 } else { 0 },
            icicles: if self == Self::Ice { 5 } else { 0 },
            crystals: if self == Self::Ice { 3 } else { 0 },
            frozen_skulls: if self == Self::Ice { 2 } else { 0 },
            cocoons: if self == Self::Spider { 3 } else { 0 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_floor_is_the_spider_nest_and_no_floor_repeats_a_theme() {
        for seed in 0..50 {
            let themes: Vec<Theme> = (1..=5).map(|floor| Theme::for_floor(seed, floor)).collect();
            assert_eq!(themes[0], Theme::Spider);
            for (index, theme) in themes.iter().enumerate() {
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
    fn every_theme_has_its_own_backdrop() {
        let all = [
            Theme::Stone,
            Theme::Sewer,
            Theme::Moss,
            Theme::Spider,
            Theme::Ice,
            Theme::Lava,
            Theme::Temple,
        ];
        for (index, theme) in all.iter().enumerate() {
            for other in &all[index + 1..] {
                assert_ne!(theme.backdrop(), other.backdrop());
            }
        }
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
        assert_eq!(Theme::Lava.mix().geysers, 0);
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
                mix.geysers,
                mix.ice_patches,
                mix.web_patches,
                mix.arrow_traps
            ),
            (0, 0, 0, 0)
        );
    }
}
