package cz.empatix.upgrades.appliers;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.guns.service.GunService;
import cz.empatix.map.service.MapService;
import cz.empatix.player.ActiveUpgradeService;
import cz.empatix.player.Player;
import cz.empatix.testsupport.MockedAssets;
import cz.empatix.upgrades.domain.PlayerUpgradeName;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.atLeastOnce;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/**
 * Composition coverage of {@link UpgradesApplicationService}: it wires the player/gun/map appliers
 * and {@code apply()} fans the call out to all three. With all collaborators mocked we verify the
 * delegation happens (each applier queries the upgrade service) and that a mixed bought-set is
 * applied across the player and gun appliers in a single {@code apply()} pass.
 *
 * <p>{@link GunService} has an asset-fetching static initializer; mocking it triggers that block, so
 * we force class-init once under a {@link MockedAssets}-stubbed static before any mock is created.
 */
class UpgradesApplicationServiceTest {

    @BeforeAll
    static void headlessAudio() {
        cz.empatix.audio.init.ALInit.canUseAL = false;
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            try {
                Class.forName("cz.empatix.guns.service.GunService", true,
                        UpgradesApplicationServiceTest.class.getClassLoader());
            } catch (final ClassNotFoundException e) {
                throw new IllegalStateException(e);
            }
        }
    }

    @Test
    void applyWithNothingBoughtRunsAllAppliersWithoutError() {
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);
        final Player player = mock(Player.class);
        final GunService gunService = mock(GunService.class);
        final MapService mapService = mock(MapService.class);

        final UpgradesApplicationService service =
                new UpgradesApplicationService(upgrades, player, gunService, mapService);

        assertDoesNotThrow(service::apply);
        // Every applier consults the upgrade service at least once.
        verify(upgrades, atLeastOnce()).isBought(any());
    }

    @Test
    void applyFansOutAcrossPlayerAndGunAppliers() {
        final ActiveUpgradeService upgrades = mock(ActiveUpgradeService.class);
        final Player player = mock(Player.class);
        final GunService gunService = mock(GunService.class);
        final MapService mapService = mock(MapService.class);

        when(player.getMaxHealth()).thenReturn(100);
        when(upgrades.isBought(PlayerUpgradeName.IMPROVED_HEALTH)).thenReturn(true);  // player applier
        when(upgrades.isBought(PlayerUpgradeName.ADDITIONAL_GUN)).thenReturn(true);   // gun applier

        new UpgradesApplicationService(upgrades, player, gunService, mapService).apply();

        verify(player).setMaxHealth(120); // (int)(100 * 1.2) from the player applier
        verify(gunService).addSlot();     // from the gun applier
    }
}
