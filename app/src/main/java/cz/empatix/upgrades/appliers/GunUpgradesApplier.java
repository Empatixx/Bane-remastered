package cz.empatix.upgrades.appliers;

import cz.empatix.common.RngService;
import cz.empatix.guns.service.GunService;
import cz.empatix.player.ActiveUpgradeService;
import cz.empatix.upgrades.domain.PlayerUpgradeName;

public class GunUpgradesApplier implements UpgradesApplier{
    private final GunService gunService;
    private final ActiveUpgradeService activeUpgradeService;
    private final RngService rng;
    public GunUpgradesApplier(final GunService gunService,
                              final ActiveUpgradeService activeUpgradeService,
                              final RngService rng) {
        this.gunService = gunService;
        this.activeUpgradeService = activeUpgradeService;
        this.rng = rng;
    }
    @Override
    public void apply() {
        if (activeUpgradeService.isBought(PlayerUpgradeName.IMPROVED_DAMAGE)) {
            gunService.addHook((damage) -> (int)(damage * 1.2));
        }
        if (activeUpgradeService.isBought(PlayerUpgradeName.ENHANCED_DAMAGE)) {
            gunService.addHook((damage) -> (int)(damage * 1.25));
        }
        if (activeUpgradeService.isBought(PlayerUpgradeName.CRITICAL_HIT)){
            gunService.addHook((damage) -> {
                if (rng.nextFloat() < 0.1) {
                    return damage * 2;
                }
                return damage;
            });
        }
        if (activeUpgradeService.isBought(PlayerUpgradeName.ADDITIONAL_AMMO)){
            gunService.getGuns().forEach(gun -> gun.setMaxAmmo((int) (gun.getMaxAmmo() * 1.1)));
        }


        if (activeUpgradeService.isBought(PlayerUpgradeName.ADDITIONAL_GUN)){
            gunService.addSlot();
        }
    }
}
