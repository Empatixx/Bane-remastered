package cz.empatix.upgrades.service;

import cz.empatix.upgrades.domain.UpgradeTraitType;

import java.io.File;
import java.util.Map;

public class UpgradeService {
    private final FileUpgradeService fileUpgradeService = new FileUpgradeService();
    private final Map<UpgradeTraitType, Integer> upgrades = fileUpgradeService.load();
    private final static UpgradeService INSTANCE = new UpgradeService();
    public static UpgradeService getInstance() {
        return INSTANCE;
    }

    public void upgrade(final UpgradeTraitType upgradeTraitType) {
        upgrades.put(upgradeTraitType, upgrades.get(upgradeTraitType) + 1);
    }
    public int getLevel(final UpgradeTraitType upgradeTraitType) {
        return upgrades.get(upgradeTraitType);
    }
}
