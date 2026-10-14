use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::guns::{GunKind, GunSpec, MAX_CLIP};
use crate::upgrades::{Purchase, Quality};
use crate::weapon_tree::{self, Node, Tree};

pub(crate) const MUTATION_COST: u64 = 800;
pub(crate) const REROLL_COST: u64 = 100;
pub(crate) const DEADEYE_EVERY: u32 = 5;
const RARE_FROM_ROW: i32 = 2;
const EPIC_FROM_ROW: i32 = 3;
const MIN_INTERVAL_SCALE: f32 = 0.2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stat {
    Damage,
    FireRate,
    Magazine,
    Reload,
    Accuracy,
    Crit,
}

impl Stat {
    pub(crate) const ALL: [Self; weapon_tree::STAT_KINDS] = [
        Self::Damage,
        Self::FireRate,
        Self::Magazine,
        Self::Reload,
        Self::Accuracy,
        Self::Crit,
    ];

    /// The bonus of a common node, in percent or in rounds for the magazine.
    pub(crate) fn amount(self) -> u32 {
        match self {
            Self::Damage => 5,
            Self::FireRate => 4,
            Self::Magazine => 1,
            Self::Reload => 6,
            Self::Accuracy => 8,
            Self::Crit => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mutation {
    Piercing,
    Explosive,
    Venom,
    Frost,
    Ricochet,
    Deadeye,
}

impl Mutation {
    pub(crate) const ALL: [Self; weapon_tree::MUTATION_KINDS] = [
        Self::Piercing,
        Self::Explosive,
        Self::Venom,
        Self::Frost,
        Self::Ricochet,
        Self::Deadeye,
    ];
}

pub(crate) fn stat_of(node: &Node) -> Stat {
    Stat::ALL[node.stat.min(Stat::ALL.len() - 1)]
}

pub(crate) fn mutation_of(node: &Node) -> Option<Mutation> {
    node.mutation
        .map(|id| Mutation::ALL[id.min(Mutation::ALL.len() - 1)])
}

pub(crate) fn quality_of(node: &Node) -> Quality {
    if node.mutation.is_some() || node.row >= EPIC_FROM_ROW {
        Quality::Epic
    } else if node.row >= RARE_FROM_ROW {
        Quality::Rare
    } else {
        Quality::Common
    }
}

pub(crate) fn multiplier(node: &Node) -> u32 {
    match quality_of(node) {
        Quality::Common => 1,
        Quality::Rare => 2,
        Quality::Epic => 3,
    }
}

pub(crate) fn cost(node: &Node) -> u64 {
    if node.mutation.is_some() {
        MUTATION_COST
    } else {
        100 + 100 * (node.row.max(1) as u64 - 1)
    }
}

/// What the player has done with the tree of one weapon: how many times it was regrown and
/// which of its nodes were bought.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct WeaponProgress {
    pub(crate) rerolls: u32,
    pub(crate) owned: BTreeSet<usize>,
}

impl WeaponProgress {
    pub(crate) fn tree(&self, kind: GunKind) -> Tree {
        weapon_tree::generate(kind, self.rerolls)
    }

    pub(crate) fn owns(&self, index: usize) -> bool {
        self.owned.contains(&index)
    }

    /// The node is not bought yet and hangs on the root or on a bought node.
    pub(crate) fn can_unlock(&self, tree: &Tree, index: usize) -> bool {
        tree.nodes.get(index).is_some_and(|node| {
            !self.owns(index) && node.parent.is_none_or(|parent| self.owns(parent))
        })
    }

    pub(crate) fn spent(&self, tree: &Tree) -> u64 {
        self.owned
            .iter()
            .filter_map(|index| tree.nodes.get(*index))
            .map(cost)
            .sum()
    }

    pub(crate) fn buy(&mut self, kind: GunKind, index: usize, gold: &mut u64) -> Purchase {
        let tree = self.tree(kind);
        let Some(node) = tree.nodes.get(index) else {
            return Purchase::Locked;
        };
        if self.owns(index) {
            return Purchase::AlreadyOwned;
        }
        if !self.can_unlock(&tree, index) {
            return Purchase::Locked;
        }
        let price = cost(node);
        if *gold < price {
            return Purchase::TooExpensive;
        }
        *gold -= price;
        self.owned.insert(index);
        Purchase::Bought
    }

    /// Gold the reroll leaves after it pays back the bought nodes and takes its fee.
    pub(crate) fn gold_after_reroll(&self, kind: GunKind, gold: u64) -> Option<u64> {
        (gold + self.spent(&self.tree(kind))).checked_sub(REROLL_COST)
    }

    /// Regrows the tree: every bought node is paid back and the reroll fee is taken.
    pub(crate) fn reroll(&mut self, kind: GunKind, gold: &mut u64) -> bool {
        let Some(left) = self.gold_after_reroll(kind, *gold) else {
            return false;
        };
        *gold = left;
        self.rerolls += 1;
        self.owned.clear();
        true
    }

    pub(crate) fn mods(&self, kind: GunKind) -> GunMods {
        let tree = self.tree(kind);
        let mut mods = GunMods::default();
        for node in self.owned.iter().filter_map(|index| tree.nodes.get(*index)) {
            if let Some(mutation) = mutation_of(node) {
                mods.mutation = Some(mutation);
                continue;
            }
            let bonus = (stat_of(node).amount() * multiplier(node)) as f32 / 100.0;
            match stat_of(node) {
                Stat::Damage => mods.damage += bonus,
                Stat::FireRate => mods.fire_rate += bonus,
                Stat::Magazine => {
                    mods.magazine += (stat_of(node).amount() * multiplier(node)) as i32;
                }
                Stat::Reload => mods.reload += bonus,
                Stat::Accuracy => mods.accuracy += bonus,
                Stat::Crit => mods.crit += bonus,
            }
        }
        mods
    }
}

/// The bonuses of the bought nodes of one weapon.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct GunMods {
    pub(crate) damage: f32,
    pub(crate) fire_rate: f32,
    pub(crate) magazine: i32,
    pub(crate) reload: f32,
    pub(crate) accuracy: f32,
    pub(crate) crit: f32,
    pub(crate) mutation: Option<Mutation>,
}

impl GunMods {
    pub(crate) fn apply(&self, spec: GunSpec) -> GunSpec {
        GunSpec {
            damage_scale: spec.damage_scale * (1.0 + self.damage),
            fire_interval: spec.fire_interval / (1.0 + self.fire_rate).max(MIN_INTERVAL_SCALE),
            clip_size: (spec.clip_size + self.magazine).min(MAX_CLIP as i32),
            reload_time: spec.reload_time / (1.0 + self.reload),
            inaccuracy: spec.inaccuracy * (1.0 - self.accuracy).max(0.0),
            crit_chance: spec.crit_chance + self.crit,
            mutation: self.mutation.or(spec.mutation),
            ..spec
        }
    }
}

/// The bonuses of every weapon, handed to the loadout of the player.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct WeaponMods([GunMods; GunKind::ALL.len()]);

impl WeaponMods {
    pub(crate) fn from_progress(progress: &BTreeMap<GunKind, WeaponProgress>) -> Self {
        Self(std::array::from_fn(|index| {
            let kind = GunKind::ALL[index];
            progress
                .get(&kind)
                .map_or_else(GunMods::default, |progress| progress.mods(kind))
        }))
    }

    pub(crate) fn of(&self, kind: GunKind) -> GunMods {
        GunKind::ALL
            .iter()
            .position(|other| *other == kind)
            .map_or_else(GunMods::default, |index| self.0[index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds_and_rerolls() -> impl Iterator<Item = (GunKind, u32)> {
        GunKind::ALL
            .into_iter()
            .flat_map(|kind| (0..20).map(move |rerolls| (kind, rerolls)))
    }

    fn first_root(tree: &Tree) -> usize {
        tree.nodes
            .iter()
            .position(|node| node.parent.is_none())
            .unwrap_or(0)
    }

    #[test]
    fn only_a_node_on_the_root_or_on_a_bought_node_can_be_bought() {
        for (kind, rerolls) in kinds_and_rerolls() {
            let progress = WeaponProgress {
                rerolls,
                ..WeaponProgress::default()
            };
            let tree = progress.tree(kind);
            for (index, node) in tree.nodes.iter().enumerate() {
                assert_eq!(progress.can_unlock(&tree, index), node.parent.is_none());
            }
        }
    }

    #[test]
    fn buying_pays_with_gold_and_remembers_the_node() {
        let kind = GunKind::Pistol;
        let mut progress = WeaponProgress::default();
        let tree = progress.tree(kind);
        let root = first_root(&tree);
        let mut gold = 150;

        assert_eq!(progress.buy(kind, root, &mut gold), Purchase::Bought);
        assert_eq!(gold, 50);
        assert!(progress.owns(root));
        assert_eq!(progress.buy(kind, root, &mut gold), Purchase::AlreadyOwned);
        let child = tree
            .nodes
            .iter()
            .position(|node| node.parent == Some(root))
            .expect("a child");
        assert_eq!(progress.buy(kind, child, &mut gold), Purchase::TooExpensive);
        assert_eq!(gold, 50);
        let deep = tree
            .nodes
            .iter()
            .position(|node| node.mutation.is_some())
            .expect("a mutation");
        let mut rich = 99_999;
        assert_eq!(progress.buy(kind, deep, &mut rich), Purchase::Locked);
    }

    #[test]
    fn a_reroll_pays_back_the_tree_takes_its_fee_and_regrows_it() {
        let kind = GunKind::Uzi;
        let mut progress = WeaponProgress::default();
        let root = first_root(&progress.tree(kind));
        let mut gold = 100;
        assert_eq!(progress.buy(kind, root, &mut gold), Purchase::Bought);
        assert_eq!(gold, 0);

        assert!(progress.reroll(kind, &mut gold));

        assert_eq!(gold, 0);
        assert_eq!(progress.rerolls, 1);
        assert!(progress.owned.is_empty());
        assert!(!progress.reroll(kind, &mut gold));
        assert_eq!(progress.rerolls, 1);
    }

    #[test]
    fn bought_stats_add_up_by_rarity() {
        let kind = GunKind::Revolver;
        let progress = WeaponProgress::default();
        let tree = progress.tree(kind);
        let mut everything = WeaponProgress {
            owned: (0..tree.nodes.len()).collect(),
            ..WeaponProgress::default()
        };
        let mods = everything.mods(kind);
        let expected = |wanted: Stat| -> u32 {
            tree.nodes
                .iter()
                .filter(|node| node.mutation.is_none() && stat_of(node) == wanted)
                .map(|node| wanted.amount() * multiplier(node))
                .sum()
        };

        assert!((mods.damage * 100.0 - expected(Stat::Damage) as f32).abs() < 1e-3);
        assert_eq!(mods.magazine, expected(Stat::Magazine) as i32);
        assert!((mods.crit * 100.0 - expected(Stat::Crit) as f32).abs() < 1e-3);
        assert_eq!(
            mods.mutation,
            tree.nodes.iter().find_map(mutation_of),
            "the mutation is switched on"
        );
        everything.owned.clear();
        assert_eq!(everything.mods(kind), GunMods::default());
    }

    #[test]
    fn mods_make_the_gun_better_and_never_break_the_clip_hud() {
        for kind in GunKind::ALL {
            let spec = kind.spec();
            let mods = GunMods {
                damage: 0.3,
                fire_rate: 0.2,
                magazine: 99,
                reload: 0.5,
                accuracy: 2.0,
                crit: 0.1,
                mutation: Some(Mutation::Deadeye),
            };
            let better = mods.apply(spec);

            assert!(better.damage_scale > spec.damage_scale);
            assert!(better.fire_interval < spec.fire_interval);
            assert!(better.reload_time < spec.reload_time);
            assert_eq!(better.inaccuracy, 0.0);
            assert_eq!(better.clip_size, MAX_CLIP as i32);
            assert_eq!(better.mutation, Some(Mutation::Deadeye));
            assert_eq!(GunMods::default().apply(spec).clip_size, spec.clip_size);
        }
    }

    #[test]
    fn every_weapon_gets_its_own_mods() {
        let mut progress = BTreeMap::new();
        let mut uzi = WeaponProgress::default();
        uzi.owned.insert(first_root(&uzi.tree(GunKind::Uzi)));
        progress.insert(GunKind::Uzi, uzi.clone());

        let mods = WeaponMods::from_progress(&progress);

        assert_eq!(mods.of(GunKind::Uzi), uzi.mods(GunKind::Uzi));
        assert_ne!(mods.of(GunKind::Uzi), GunMods::default());
        assert_eq!(mods.of(GunKind::Pistol), GunMods::default());
    }
}
