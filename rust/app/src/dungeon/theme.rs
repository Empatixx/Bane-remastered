use crate::assets::{Tex, tex};
use crate::rng::Rng;

use super::surface::Surface;

const THEME_SALT: u64 = 0x7E3A_11C5_9D42_B6F1;
const THEME_VAR: &str = "BANE_THEME";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    pub(crate) ice_patches: usize,
    pub(crate) web_patches: usize,
    pub(crate) arrow_traps: usize,
    pub(crate) icicles: usize,
    pub(crate) crystals: usize,
    pub(crate) frozen_skulls: usize,
    pub(crate) cocoons: usize,
    pub(crate) mushrooms: usize,
    /// Clutches of spider eggs against the walls.
    pub(crate) eggs: usize,
    /// Victims wrapped in webs lying on the floor.
    pub(crate) wrapped: usize,
    /// How often a room gets a bowl of fire by a wall, in percent.
    pub(crate) fire_bowl_percent: usize,
    /// How often a room gets a piece of a toppled statue, in percent.
    pub(crate) statue_percent: usize,
    /// How often a room gets shackles in a wall, in percent.
    pub(crate) shackles_percent: usize,
    /// Moss grows over the walls and banners are overgrown.
    pub(crate) mossy: bool,
    pub(crate) moss_drapes: usize,
}

const ALL: [Theme; 7] = [
    Theme::Stone,
    Theme::Sewer,
    Theme::Moss,
    Theme::Spider,
    Theme::Ice,
    Theme::Lava,
    Theme::Temple,
];

impl Theme {
    pub(crate) fn tileset_texture(self) -> Tex {
        match self {
            Self::Stone => tex::TILESET,
            Self::Sewer => tex::TILESET_SEWER,
            Self::Moss => tex::TILESET_MOSS,
            Self::Spider => tex::TILESET_SPIDER,
            Self::Ice => tex::TILESET_ICE,
            Self::Lava => tex::TILESET_LAVA,
            Self::Temple => tex::TILESET_TEMPLE,
        }
    }

    pub(crate) fn ladder_texture(self) -> Tex {
        match self {
            Self::Stone => tex::LADDER,
            Self::Sewer => tex::LADDER_SEWER,
            Self::Moss => tex::LADDER_MOSS,
            Self::Spider => tex::LADDER_SPIDER,
            Self::Ice => tex::LADDER_ICE,
            Self::Lava => tex::LADDER_LAVA,
            Self::Temple => tex::LADDER_TEMPLE,
        }
    }

    /// Every theme but stone is drawn with the wide tileset layout.
    pub(crate) fn wide_tiles(self) -> bool {
        self != Self::Stone
    }

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
        let mut rng = Rng::new(run_seed ^ THEME_SALT);
        let mut order = ALL;
        for index in (1..order.len()).rev() {
            order.swap(index, rng.below(index + 1));
        }
        order[floor.saturating_sub(1) as usize % order.len()]
    }

    pub(crate) fn surface(self) -> Option<Surface> {
        match self {
            Self::Ice => Some(Surface::Ice),
            Self::Lava => Some(Surface::Lava),
            Self::Sewer => Some(Surface::Water),
            Self::Moss => Some(Surface::Bog),
            Self::Temple => Some(Surface::Sand),
            Self::Stone => Some(Surface::Blood),
            _ => None,
        }
    }

    /// Which of the floor's own looks its rugs, tables and mine carts take: their row (or block
    /// of rows) in the sheets that hold one per floor, in this order. The stone floor's are the
    /// first, which the shop and the treasure rooms use on every floor.
    pub(crate) fn style(self) -> usize {
        match self {
            Self::Stone => 0,
            Self::Sewer => 1,
            Self::Moss => 2,
            Self::Spider => 3,
            Self::Ice => 4,
            Self::Lava => 5,
            Self::Temple => 6,
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
            ice_patches: 0,
            web_patches: if self == Self::Spider { 3 } else { 0 },
            arrow_traps: if self == Self::Temple { 2 } else { 0 },
            icicles: if self == Self::Ice { 5 } else { 0 },
            crystals: if self == Self::Ice { 3 } else { 0 },
            frozen_skulls: if self == Self::Ice { 2 } else { 0 },
            cocoons: if self == Self::Spider { 3 } else { 0 },
            mushrooms: if self == Self::Moss { 4 } else { 0 },
            eggs: if self == Self::Spider { 2 } else { 0 },
            wrapped: if self == Self::Spider { 2 } else { 0 },
            fire_bowl_percent: if self == Self::Temple { 45 } else { 0 },
            statue_percent: if self == Self::Temple { 30 } else { 0 },
            shackles_percent: if self == Self::Stone { 45 } else { 0 },
            mossy: self == Self::Moss,
            moss_drapes: if self == Self::Moss { 7 } else { 0 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_theme_has_its_own_tileset_and_only_stone_uses_narrow_tiles() {
        use crate::assets::tex;
        let all = [
            Theme::Stone,
            Theme::Sewer,
            Theme::Moss,
            Theme::Spider,
            Theme::Ice,
            Theme::Lava,
            Theme::Temple,
        ];
        let textures: std::collections::HashSet<_> =
            all.iter().map(|theme| theme.tileset_texture()).collect();

        assert_eq!(textures.len(), all.len());
        let ladders: std::collections::HashSet<_> =
            all.iter().map(|theme| theme.ladder_texture()).collect();
        assert_eq!(ladders.len(), all.len());
        assert_eq!(Theme::Stone.ladder_texture(), tex::LADDER);
        assert_eq!(Theme::Stone.tileset_texture(), tex::TILESET);
        assert_eq!(Theme::Lava.tileset_texture(), tex::TILESET_LAVA);
        assert!(!Theme::Stone.wide_tiles());
        assert!(all[1..].iter().all(|theme| theme.wide_tiles()));
    }

    #[test]
    fn the_first_seven_floors_of_a_run_never_repeat_a_theme() {
        for seed in 0..50 {
            let themes: Vec<Theme> = (1..=7).map(|floor| Theme::for_floor(seed, floor)).collect();
            for (index, theme) in themes.iter().enumerate() {
                assert!(!themes[..index].contains(theme), "seed {seed}: {themes:?}");
            }
        }
    }

    #[test]
    fn any_theme_can_open_a_run() {
        let first: std::collections::HashSet<Theme> =
            (0..200).map(|seed| Theme::for_floor(seed, 1)).collect();
        assert_eq!(first.len(), ALL.len());
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
            (mix.ice_patches, mix.web_patches, mix.arrow_traps),
            (0, 0, 0)
        );
    }
}
