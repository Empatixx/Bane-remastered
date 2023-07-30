package cz.empatix.upgrades.domain;

import cz.empatix.upgrades.service.UpgradeService;

public class DamageBoostUpgrade {
    private final int level;
    public DamageBoostUpgrade(){
        level = UpgradeService.getInstance().getLevel(UpgradeTraitType.DAMAGE_BOOST);
    }
    public int getBoostedDamage(final int defaultDamage){
        return defaultDamage + level;
    }
}
