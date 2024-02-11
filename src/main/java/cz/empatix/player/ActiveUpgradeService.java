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
    private boolean isBought(final PlayerUpgradeName upgradeName) {
        return boughtUpgrades.get(upgradeName);
    }
    public void apply(final Player player){
        if (isBought(PlayerUpgradeName.IMPROVED_HEALTH)) {
            player.setMaxHealth((int)(player.getMaxHealth()* 1.2));
        }
        if (isBought(PlayerUpgradeName.IMPROVED_ARMOR)) {
            player.setMaxArmor((int)(player.getMaxArmor()* 1.2));
        }
        if (isBought(PlayerUpgradeName.ENHANCED_HEALTH)){
            player.setMaxHealth((int)(player.getMaxHealth()* 1.25));
        }
        if (isBought(PlayerUpgradeName.REDUCED_DAMAGE_TAKEN)){
            player.addDamageHook((damage) -> (int)(damage * 0.9));
        }
        if (isBought(PlayerUpgradeName.IMPROVED_MOVEMENT_SPEED)){
            player.addHook((force) -> (int)(force * 1.1));
        }
    }
    public void apply(final GunService gunService){
        if (isBought(PlayerUpgradeName.ADDITIONAL_GUN)){
            gunService.addSlot();
        }
        if (isBought(PlayerUpgradeName.IMPROVED_DAMAGE)){
            gunService.addDamageHook((damage) -> (int)(damage * 1.1));
        }
    }

}
