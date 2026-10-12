use crate::assets::{tex, Tex};
use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

pub(crate) const TREE_TOP: f32 = 125.0;
pub(crate) const TREE_STEP: f32 = 74.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Upgrade {
    MovementSpeed,
    Discount,
    TrapMaster,
    AdditionalGun,
    GambleMachine,
    ImprovedHealth,
    ImprovedArmor,
    ArmorRegeneration,
    EnhancedHealth,
    ReducedDamageTaken,
    ImprovedDamage,
    AdditionalAmmo,
    CriticalHit,
    EnhancedDamage,
    Pet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tree {
    Utility,
    Offensive,
    Defensive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Quality {
    Common,
    Rare,
    Epic,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct UpgradeDef {
    pub(crate) title: &'static str,
    pub(crate) description: &'static [&'static str],
    pub(crate) texture: Tex,
    pub(crate) quality: Quality,
    pub(crate) price: u64,
}

impl Tree {
    pub(crate) const ALL: [Self; 3] = [Self::Utility, Self::Offensive, Self::Defensive];

    pub(crate) fn column(self) -> f32 {
        match self {
            Self::Utility => 364.0,
            Self::Offensive => 464.0,
            Self::Defensive => 564.0,
        }
    }

    pub(crate) fn icon(self) -> Tex {
        match self {
            Self::Utility => tex::UTILITY_ICON,
            Self::Offensive => tex::OFFENSE_ICON,
            Self::Defensive => tex::DEFENSE_ICON,
        }
    }

    pub(crate) fn upgrades(self) -> [Upgrade; 5] {
        use Upgrade::*;
        match self {
            Self::Utility => [
                MovementSpeed,
                Discount,
                TrapMaster,
                AdditionalGun,
                GambleMachine,
            ],
            Self::Offensive => [
                ImprovedDamage,
                AdditionalAmmo,
                CriticalHit,
                EnhancedDamage,
                Pet,
            ],
            Self::Defensive => [
                ImprovedHealth,
                ImprovedArmor,
                ArmorRegeneration,
                EnhancedHealth,
                ReducedDamageTaken,
            ],
        }
    }
}

impl Upgrade {
    pub(crate) const ALL: [Self; 15] = [
        Self::MovementSpeed,
        Self::Discount,
        Self::TrapMaster,
        Self::AdditionalGun,
        Self::GambleMachine,
        Self::ImprovedDamage,
        Self::AdditionalAmmo,
        Self::CriticalHit,
        Self::EnhancedDamage,
        Self::Pet,
        Self::ImprovedHealth,
        Self::ImprovedArmor,
        Self::ArmorRegeneration,
        Self::EnhancedHealth,
        Self::ReducedDamageTaken,
    ];

    pub(crate) fn tree(self) -> Tree {
        Tree::ALL
            .into_iter()
            .find(|tree| tree.upgrades().contains(&self))
            .unwrap_or(Tree::Utility)
    }

    pub(crate) fn index(self) -> usize {
        self.tree()
            .upgrades()
            .iter()
            .position(|upgrade| *upgrade == self)
            .unwrap_or(0)
    }

    pub(crate) fn parent(self) -> Option<Self> {
        self.index()
            .checked_sub(1)
            .map(|index| self.tree().upgrades()[index])
    }

    pub(crate) fn is_free(self) -> bool {
        self.index() == 0
    }

    pub(crate) fn top_left(self) -> (f32, f32) {
        (
            self.tree().column(),
            TREE_TOP + TREE_STEP * self.index() as f32,
        )
    }

    pub(crate) fn def(self) -> UpgradeDef {
        use Quality::{Common, Epic, Rare};
        let make = |title, description, texture, quality, price| UpgradeDef {
            title,
            description,
            texture,
            quality,
            price,
        };
        match self {
            Self::MovementSpeed => make(
                "Swiftwind Steps",
                &["Increases movement speed", " by 10%"],
                tex::TALENTS_BOOT,
                Common,
                20,
            ),
            Self::Discount => make(
                "Discount",
                &["The shop prices will", " have a chance to", " be reduced"],
                tex::TALENTS_DISCOUNT,
                Common,
                60,
            ),
            Self::TrapMaster => make(
                "Trap Master",
                &["All traps have a 30% chance", " to be immobilized"],
                tex::TALENTS_TRAP,
                Rare,
                140,
            ),
            Self::AdditionalGun => make(
                "Additional Gun",
                &["Adds an additional slot", " for a weapon"],
                tex::TALENTS_EXTRAGUN,
                Epic,
                300,
            ),
            Self::GambleMachine => make(
                "Gamble machine",
                &["Unlocks the gamble machine", " in the shop"],
                tex::TALENTS_TELEPORTATION,
                Common,
                500,
            ),
            Self::ImprovedHealth => make(
                "Hero's Heart",
                &["Increases initial health", " by 20%"],
                tex::TALENTS_HEALTH_CROSS,
                Common,
                20,
            ),
            Self::ImprovedArmor => make(
                "Improved Armor",
                &["Increases initial armor", " by 33%"],
                tex::TALENTS_ARMOR,
                Common,
                60,
            ),
            Self::ArmorRegeneration => make(
                "Armor Regeneration",
                &["Restores armor after", " successfully", " clearing a room"],
                tex::TALENTS_ARMORREGEN,
                Rare,
                140,
            ),
            Self::EnhancedHealth => make(
                "Titan's Heart",
                &["Increases initial health", " by 25%"],
                tex::TALENTS_HEALTH_CROSS2,
                Epic,
                300,
            ),
            Self::ReducedDamageTaken => make(
                "Kevlar",
                &["Reduces damage taken", " by 10%"],
                tex::TALENTS_KEVLAR,
                Common,
                500,
            ),
            Self::ImprovedDamage => make(
                "Sharpened Bullets",
                &["Increases all damage", " by 20%"],
                tex::TALENTS_BLOODTEAR,
                Common,
                20,
            ),
            Self::AdditionalAmmo => make(
                "Additional Ammo",
                &["Increases ammo capacity", " by 10%"],
                tex::TALENTS_AMMO,
                Common,
                60,
            ),
            Self::CriticalHit => make(
                "Critical Hit",
                &[
                    "Your attacks have a 10%",
                    " chance to deal double",
                    " damage",
                ],
                tex::TALENTS_CRITICALHIT,
                Rare,
                140,
            ),
            Self::EnhancedDamage => make(
                "Dangerous Bullets",
                &["Increases all damage", " by 25%"],
                tex::TALENTS_BLOODTEAR2,
                Epic,
                300,
            ),
            Self::Pet => make(
                "Pet",
                &["Unlocks a pet that will", " help you in battle"],
                tex::TALENTS_PET,
                Common,
                500,
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Purchase {
    Bought,
    AlreadyOwned,
    Locked,
    TooExpensive,
}

pub(crate) fn owns(unlocked: &BTreeSet<Upgrade>, upgrade: Upgrade) -> bool {
    upgrade.is_free() || unlocked.contains(&upgrade)
}

pub(crate) fn can_buy(unlocked: &BTreeSet<Upgrade>, upgrade: Upgrade) -> bool {
    upgrade.parent().is_none_or(|parent| owns(unlocked, parent))
}

pub(crate) fn purchase(
    unlocked: &mut BTreeSet<Upgrade>,
    balance: &mut u64,
    upgrade: Upgrade,
) -> Purchase {
    if owns(unlocked, upgrade) {
        return Purchase::AlreadyOwned;
    }
    if !can_buy(unlocked, upgrade) {
        return Purchase::Locked;
    }
    let price = upgrade.def().price;
    if *balance < price {
        return Purchase::TooExpensive;
    }
    *balance -= price;
    unlocked.insert(upgrade);
    Purchase::Bought
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Perks {
    pub(crate) speed: f32,
    pub(crate) health_scales: [f32; 2],
    pub(crate) armor_scale: f32,
    pub(crate) damage_scales: [f32; 2],
    pub(crate) critical: bool,
    pub(crate) ammo_scale: f32,
    pub(crate) extra_slots: usize,
    pub(crate) armor_regeneration: bool,
    pub(crate) discount: bool,
    pub(crate) trap_master: bool,
    pub(crate) gamble_machine: bool,
    pub(crate) pet: bool,
    pub(crate) damage_taken_scale: f32,
}

impl Default for Perks {
    fn default() -> Self {
        Self::from_unlocked(&BTreeSet::new())
    }
}

impl Perks {
    pub(crate) fn from_unlocked(unlocked: &BTreeSet<Upgrade>) -> Self {
        let has = |upgrade| owns(unlocked, upgrade);
        let scale = |upgrade, value| if has(upgrade) { value } else { 1.0 };
        Self {
            speed: scale(Upgrade::MovementSpeed, 1.1),
            health_scales: [
                scale(Upgrade::ImprovedHealth, 1.2),
                scale(Upgrade::EnhancedHealth, 1.25),
            ],
            armor_scale: scale(Upgrade::ImprovedArmor, 1.2),
            damage_scales: [
                scale(Upgrade::ImprovedDamage, 1.2),
                scale(Upgrade::EnhancedDamage, 1.25),
            ],
            critical: has(Upgrade::CriticalHit),
            ammo_scale: scale(Upgrade::AdditionalAmmo, 1.1),
            extra_slots: usize::from(has(Upgrade::AdditionalGun)),
            armor_regeneration: has(Upgrade::ArmorRegeneration),
            discount: has(Upgrade::Discount),
            trap_master: has(Upgrade::TrapMaster),
            gamble_machine: has(Upgrade::GambleMachine),
            pet: has(Upgrade::Pet),
            damage_taken_scale: scale(Upgrade::ReducedDamageTaken, 0.9),
        }
    }

    pub(crate) fn max_health(&self, base: i32) -> i32 {
        self.health_scales
            .iter()
            .fold(base, |value, scale| (value as f32 * scale) as i32)
    }

    pub(crate) fn max_armor(&self, base: i32) -> i32 {
        (base as f32 * self.armor_scale) as i32
    }

    pub(crate) fn bullet_damage(&self, base: i32, critical_roll: f32) -> i32 {
        let mut damage = self
            .damage_scales
            .iter()
            .fold(base, |value, scale| (value as f32 * scale) as i32);
        if self.critical && critical_roll < 0.1 {
            damage *= 2;
        }
        damage
    }

    pub(crate) fn damage_taken(&self, damage: i32) -> i32 {
        (damage as f32 * self.damage_taken_scale) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(upgrades: &[Upgrade]) -> BTreeSet<Upgrade> {
        upgrades.iter().copied().collect()
    }

    #[test]
    fn every_upgrade_belongs_to_exactly_one_tree_in_the_order_of_bane() {
        for upgrade in Upgrade::ALL {
            let trees = Tree::ALL
                .into_iter()
                .filter(|tree| tree.upgrades().contains(&upgrade))
                .count();
            assert_eq!(trees, 1, "{upgrade:?}");
        }
        assert_eq!(Upgrade::Pet.parent(), Some(Upgrade::EnhancedDamage));
        assert_eq!(Upgrade::MovementSpeed.parent(), None);
    }

    #[test]
    fn the_first_upgrade_of_every_tree_is_free_and_prices_grow() {
        let free: Vec<Upgrade> = Upgrade::ALL.into_iter().filter(|u| u.is_free()).collect();
        assert_eq!(
            free,
            vec![
                Upgrade::MovementSpeed,
                Upgrade::ImprovedDamage,
                Upgrade::ImprovedHealth
            ]
        );
        for tree in Tree::ALL {
            let prices: Vec<u64> = tree.upgrades().iter().map(|u| u.def().price).collect();
            assert_eq!(prices, vec![20, 60, 140, 300, 500]);
        }
    }

    #[test]
    fn the_talents_sit_in_three_columns_one_row_every_74_pixels() {
        assert_eq!(Upgrade::Discount.top_left(), (364.0, 199.0));
        assert_eq!(Upgrade::AdditionalAmmo.top_left(), (464.0, 199.0));
        assert_eq!(Upgrade::ReducedDamageTaken.top_left(), (564.0, 421.0));
    }

    #[test]
    fn buying_needs_the_parent_and_the_money_and_takes_the_price() {
        let mut unlocked = BTreeSet::new();
        let mut balance = 70;

        assert_eq!(
            purchase(&mut unlocked, &mut balance, Upgrade::TrapMaster),
            Purchase::Locked
        );
        assert_eq!(
            purchase(&mut unlocked, &mut balance, Upgrade::Discount),
            Purchase::Bought
        );
        assert_eq!(balance, 10);
        assert_eq!(
            purchase(&mut unlocked, &mut balance, Upgrade::Discount),
            Purchase::AlreadyOwned
        );
        assert_eq!(
            purchase(&mut unlocked, &mut balance, Upgrade::TrapMaster),
            Purchase::TooExpensive
        );
        assert_eq!(balance, 10);
        assert_eq!(
            purchase(&mut unlocked, &mut balance, Upgrade::MovementSpeed),
            Purchase::AlreadyOwned
        );
    }

    #[test]
    fn perks_follow_the_bought_upgrades() {
        let none = Perks::default();
        assert_eq!(none.speed, 1.1);
        assert_eq!(none.max_health(10), 12);
        assert_eq!(none.bullet_damage(3, 0.0), 3);

        let all = Perks::from_unlocked(&set(&Upgrade::ALL));
        assert_eq!(all.max_health(10), 15);
        assert_eq!(all.max_armor(5), 6);
        assert_eq!(all.ammo_scale, 1.1);
        assert_eq!(all.extra_slots, 1);
        assert!(all.gamble_machine && all.pet && all.discount && all.trap_master);
    }

    #[test]
    fn damage_is_truncated_after_every_step_like_in_java() {
        let boosted = Perks::from_unlocked(&set(&[Upgrade::EnhancedDamage]));
        assert_eq!(boosted.bullet_damage(1, 0.9), 1);
        assert_eq!(boosted.bullet_damage(4, 0.9), 5);
        let critical = Perks::from_unlocked(&set(&[Upgrade::CriticalHit]));
        assert_eq!(critical.bullet_damage(3, 0.05), 6);
        assert_eq!(critical.bullet_damage(3, 0.5), 3);
    }

    #[test]
    fn kevlar_rounds_down_so_a_single_point_of_damage_disappears() {
        let kevlar = Perks::from_unlocked(&set(&[Upgrade::ReducedDamageTaken]));
        assert_eq!(kevlar.damage_taken(1), 0);
        assert_eq!(kevlar.damage_taken(3), 2);
        assert_eq!(Perks::default().damage_taken(3), 3);
    }
}
