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

fn percent(scale: f32) -> String {
    let change = ((scale - 1.0) * 100.0).round() as i32;
    format!("{change:+}%")
}

impl RunRules {
    /// What the rules change against a normal run, one short phrase per change.
    pub(crate) fn describe(&self) -> Vec<String> {
        let mut parts = Vec::new();
        if let Some(health) = self.player_max_health {
            parts.push(format!(
                "{health} heart{}",
                if health == 1 { "" } else { "s" }
            ));
        }
        if self.player_damage != 1.0 {
            parts.push(format!("Your damage {}", percent(self.player_damage)));
        }
        if self.enemy_health != 1.0 {
            parts.push(format!("Enemy health {}", percent(self.enemy_health)));
        }
        if self.enemy_damage != 1.0 {
            parts.push(format!("Enemy damage {}", percent(self.enemy_damage)));
        }
        if let Some(gun) = self.only_gun {
            parts.push(format!("{} only", gun.name()));
        }
        if !self.shops {
            parts.push("No shops".to_owned());
        }
        if !self.dodge {
            parts.push("No dodge".to_owned());
        }
        if !self.armor_regen {
            parts.push("No armor regen".to_owned());
        }
        parts
    }

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
        assert_eq!(shaped.bullet_damage(10, false, 0.0), 24);
        assert!(!shaped.dodge);
        assert!(!shaped.armor_regeneration);
        assert_eq!(shaped.only_gun, Some(GunKind::Pistol));
    }

    #[test]
    fn the_description_names_every_change_and_nothing_else() {
        assert!(RunRules::default().describe().is_empty());
        let rules = RunRules {
            player_max_health: Some(1),
            player_damage: 2.0,
            enemy_health: 1.6,
            only_gun: Some(GunKind::Pistol),
            shops: false,
            ..RunRules::default()
        };
        assert_eq!(
            rules.describe(),
            vec![
                "1 heart",
                "Your damage +100%",
                "Enemy health +60%",
                "Pistol only",
                "No shops"
            ]
        );
    }
}
