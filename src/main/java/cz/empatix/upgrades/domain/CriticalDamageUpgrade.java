package cz.empatix.upgrades.domain;

import cz.empatix.upgrades.service.UpgradeService;

public class CriticalDamageUpgrade {
    private final int level;
    public CriticalDamageUpgrade(){
        level = UpgradeService.getInstance().getLevel(UpgradeTraitType.CRITICAL_DAMAGE);
    }
    public float getCriticalMultiplier(){
        return 2f + level * 0.25f;
    }
}
