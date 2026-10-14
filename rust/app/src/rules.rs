use crate::guns::GunKind;
use crate::upgrades::Perks;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct RunRules {
    pub(crate) player_max_health: Option<i32>,
    pub(crate) player_damage: f32,
    pub(crate) enemy_health: f32,
    pub(crate) enemy_damage: f32,
    pub(crate) only_gun: Option<GunKind>,
    pub(crate) shops: bool,
    pub(crate) dodge: bool,
    pub(crate) armor_regen: bool,
}

impl Default for RunRules {
    fn default() -> Self {
        Self {
            player_max_health: None,
            player_damage: 1.0,
            enemy_health: 1.0,
            enemy_damage: 1.0,
            only_gun: None,
            shops: true,
            dodge: true,
            armor_regen: true,
        }
    }
}

impl RunRules {
    pub(crate) fn shape(&self, mut perks: Perks) -> Perks {
        perks.damage_scales[0] *= self.player_damage;
        perks.max_health_cap = self.player_max_health;
        perks.only_gun = self.only_gun;
        perks.dodge &= self.dodge;
        perks.armor_regeneration &= self.armor_regen;
        perks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_rules_leave_the_perks_untouched() {
        let perks = Perks::default();
        assert_eq!(RunRules::default().shape(perks), perks);
    }

    #[test]
    fn rules_scale_damage_cap_health_and_switch_things_off() {
        let rules = RunRules {
            player_max_health: Some(1),
            player_damage: 2.0,
            enemy_damage: 1.5,
            only_gun: Some(GunKind::Pistol),
            dodge: false,
            armor_regen: false,
            ..RunRules::default()
        };
        let owned = Perks {
            armor_regeneration: true,
            ..Perks::default()
        };
        let shaped = rules.shape(owned);

        assert_eq!(shaped.max_health(10), 1);
        assert_eq!(shaped.bullet_damage(10, false), 24);
        assert!(!shaped.dodge);
        assert!(!shaped.armor_regeneration);
        assert_eq!(shaped.only_gun, Some(GunKind::Pistol));
    }
}
