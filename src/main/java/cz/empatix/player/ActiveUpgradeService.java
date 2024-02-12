package cz.empatix.player;

import cz.empatix.guns.service.GunService;
import cz.empatix.upgrades.domain.PlayerUpgradeName;
import cz.empatix.upgrades.service.FileUpgradeService;

import java.util.Map;

public class ActiveUpgradeService {
    private final Map<PlayerUpgradeName, Boolean> boughtUpgrades;
    public ActiveUpgradeService() {
        final FileUpgradeService fileUpgradeService = new FileUpgradeService();
        boughtUpgrades = fileUpgradeService.load();
    }
    public boolean isBought(final PlayerUpgradeName upgradeName) {
        return boughtUpgrades.get(upgradeName);
    }
}
