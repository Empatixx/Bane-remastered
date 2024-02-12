package cz.empatix.upgrades.appliers;

import cz.empatix.guns.service.GunService;
import cz.empatix.player.ActiveUpgradeService;
import cz.empatix.player.Player;

import java.util.List;

public class UpgradesApplicationService {
    private final List<UpgradesApplier> upgradesAppliers;
    public UpgradesApplicationService(final ActiveUpgradeService activeUpgradeService,
                                      final Player player,
                                      final GunService gunService) {
        this.upgradesAppliers = List.of(
                new PlayerUpgradesApplier(activeUpgradeService, player),
                new GunUpgradesApplier(gunService, activeUpgradeService)
        );
    }
    public void apply() {
        upgradesAppliers.forEach(UpgradesApplier::apply);
    }
}
