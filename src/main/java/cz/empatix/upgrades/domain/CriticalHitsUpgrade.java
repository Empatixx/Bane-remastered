package cz.empatix.upgrades.domain;

import cz.empatix.upgrades.service.UpgradeService;

public class CriticalHitsUpgrade {
    private final int level;
    public CriticalHitsUpgrade(){
        level = UpgradeService.getInstance().getLevel(UpgradeTraitType.CRITICAL_HITS);
    }
    public boolean canCriticalHit() {
        return level > 0;
    }
}
