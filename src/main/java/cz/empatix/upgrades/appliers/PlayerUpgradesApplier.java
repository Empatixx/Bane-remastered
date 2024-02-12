package cz.empatix.upgrades.appliers;

import cz.empatix.player.ActiveUpgradeService;
import cz.empatix.player.Player;
import cz.empatix.upgrades.domain.PlayerUpgradeName;

public class PlayerUpgradesApplier implements UpgradesApplier {
    private final ActiveUpgradeService activeUpgradeService;
    private final Player player;
    public PlayerUpgradesApplier(final ActiveUpgradeService activeUpgradeService,
                                 final Player player) {
        this.activeUpgradeService = activeUpgradeService;
        this.player = player;
    }

    @Override
    public void apply() {
        if (activeUpgradeService.isBought(PlayerUpgradeName.IMPROVED_HEALTH)) {
            player.setMaxHealth((int)(player.getMaxHealth()* 1.2));
        }
        if (activeUpgradeService.isBought(PlayerUpgradeName.IMPROVED_ARMOR)) {
            player.setMaxArmor((int)(player.getMaxArmor()* 1.2));
        }
        if (activeUpgradeService.isBought(PlayerUpgradeName.ENHANCED_HEALTH)){
            player.setMaxHealth((int)(player.getMaxHealth()* 1.25));
        }
        if (activeUpgradeService.isBought(PlayerUpgradeName.REDUCED_DAMAGE_TAKEN)){
            player.addDamageHook((damage) -> (int)(damage * 0.9));
        }
        if (activeUpgradeService.isBought(PlayerUpgradeName.IMPROVED_MOVEMENT_SPEED)){
            player.addHook((force) -> (int)(force * 1.1));
        }
    }
}
