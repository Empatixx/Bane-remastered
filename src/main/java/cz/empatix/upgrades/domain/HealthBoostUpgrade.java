package cz.empatix.upgrades.domain;

import cz.empatix.upgrades.service.UpgradeService;

public class HealthBoostUpgrade {
    private final int level;
    public HealthBoostUpgrade(){
        level = UpgradeService.getInstance().getLevel(UpgradeTraitType.HEALTH_BOOST);
    }
    public int getBoostedHealth(final int defaultHealth){
        return defaultHealth + level * 2;
    }
}