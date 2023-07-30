package cz.empatix.upgrades.domain;

import cz.empatix.upgrades.service.UpgradeService;

public class PotionEffectivenessUpgrade {
    private final int level;
    public PotionEffectivenessUpgrade(){
        level = UpgradeService.getInstance().getLevel(UpgradeTraitType.POTION_EFFECTIVENESS);
    }
}
