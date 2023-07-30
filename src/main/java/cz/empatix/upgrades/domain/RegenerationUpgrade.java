package cz.empatix.upgrades.domain;

import cz.empatix.upgrades.service.UpgradeService;

public class RegenerationUpgrade {
    private final int level;
    public RegenerationUpgrade(){
        level = UpgradeService.getInstance().getLevel(UpgradeTraitType.REGENERATION);
    }
    public boolean canRegenerate() {
        return level > 0;
    }
}
