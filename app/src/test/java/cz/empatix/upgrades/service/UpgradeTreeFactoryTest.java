package cz.empatix.upgrades.service;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.testsupport.MockedAssets;
import cz.empatix.upgrades.domain.PlayerUpgrade;
import cz.empatix.upgrades.domain.PlayerUpgradeName;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mockStatic;

/**
 * Structure coverage of {@link UpgradeTreeFactory}: each tree has five upgrades with the expected
 * names, prices, parent chain and starting bought-state. Every factory call instantiates real
 * {@link PlayerUpgrade}s (texture fetch in ctor), so all assertions run inside a
 * {@link MockedAssets}-stubbed static.
 */
class UpgradeTreeFactoryTest {

    @BeforeAll
    static void headlessAudio() {
        cz.empatix.audio.init.ALInit.canUseAL = false;
    }

    // Expected per-level pricing, shared across all three trees.
    private static final int[] PRICES = {20, 60, 140, 300, 500};

    private void assertChainPricesAndParents(final List<PlayerUpgrade> upgrades) {
        assertEquals(5, upgrades.size());
        for (int i = 0; i < upgrades.size(); i++) {
            assertEquals(PRICES[i], upgrades.get(i).getPrice(),
                    "price mismatch at index " + i);
            if (i == 0) {
                // root has no parent and starts unlocked (bought).
                assertTrue(upgrades.get(0).isBought());
            } else {
                // each child's parent is the previous tier.
                assertSame(upgrades.get(i - 1), upgrades.get(i).getParent(),
                        "parent mismatch at index " + i);
            }
        }
    }

    @Test
    void utilityTreeHasExpectedNamesPricesAndChain() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final List<PlayerUpgrade> upgrades = UpgradeTreeFactory.createUtilityTree().getUpgrades();

            assertEquals(
                    List.of(PlayerUpgradeName.IMPROVED_MOVEMENT_SPEED, PlayerUpgradeName.DISCOUNT,
                            PlayerUpgradeName.TRAP_MASTER, PlayerUpgradeName.ADDITIONAL_GUN,
                            PlayerUpgradeName.GAMBLE_MACHINE),
                    upgrades.stream().map(PlayerUpgrade::getName).toList());
            assertChainPricesAndParents(upgrades);
            // First two tiers ship unlocked, the rare/epic tiers do not.
            assertTrue(upgrades.get(1).isBought());
            assertFalse(upgrades.get(2).isBought());
        }
    }

    @Test
    void defenseTreeHasExpectedNamesPricesAndChain() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final List<PlayerUpgrade> upgrades = UpgradeTreeFactory.createDefenseTree().getUpgrades();

            assertEquals(
                    List.of(PlayerUpgradeName.IMPROVED_HEALTH, PlayerUpgradeName.IMPROVED_ARMOR,
                            PlayerUpgradeName.ARMOR_REGENERATION, PlayerUpgradeName.ENHANCED_HEALTH,
                            PlayerUpgradeName.REDUCED_DAMAGE_TAKEN),
                    upgrades.stream().map(PlayerUpgrade::getName).toList());
            assertChainPricesAndParents(upgrades);
        }
    }

    @Test
    void offenseTreeHasExpectedNamesPricesAndChain() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final List<PlayerUpgrade> upgrades = UpgradeTreeFactory.createOffenseTree().getUpgrades();

            assertEquals(
                    List.of(PlayerUpgradeName.IMPROVED_DAMAGE, PlayerUpgradeName.ADDITIONAL_AMMO,
                            PlayerUpgradeName.CRITICAL_HIT, PlayerUpgradeName.ENHANCED_DAMAGE,
                            PlayerUpgradeName.PET),
                    upgrades.stream().map(PlayerUpgrade::getName).toList());
            assertChainPricesAndParents(upgrades);
        }
    }

    @Test
    void everyUpgradeHasANonEmptyTitleAndDescription() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final List<PlayerUpgrade> all = List.of(
                    UpgradeTreeFactory.createUtilityTree(),
                    UpgradeTreeFactory.createOffenseTree(),
                    UpgradeTreeFactory.createDefenseTree())
                    .stream().flatMap(t -> t.getUpgrades().stream()).toList();

            assertEquals(15, all.size());
            for (final PlayerUpgrade u : all) {
                assertFalse(u.getTitle().isBlank(), "blank title for " + u.getName());
                assertFalse(u.getDescription().isBlank(), "blank description for " + u.getName());
            }
        }
    }
}
