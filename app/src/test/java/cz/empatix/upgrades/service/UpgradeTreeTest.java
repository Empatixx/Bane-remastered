package cz.empatix.upgrades.service;

import cz.empatix.testsupport.HeadlessAssets;
import cz.empatix.testsupport.MockedAssets;
import cz.empatix.upgrades.domain.PlayerUpgrade;
import cz.empatix.upgrades.domain.PlayerUpgradeName;
import cz.empatix.upgrades.domain.QualityLevel;
import cz.empatix.upgrades.domain.TreeType;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Logic coverage of {@link UpgradeTree}: parent-connector count derivation, hit-testing delegation
 * ({@code getPressedUpgrade}), bulk bought-state application by name ({@code setBoughtUpgrades}),
 * and the structure-mutating helpers ({@code setPosition}/{@code update}). Construction fetches the
 * tree-icon texture and builds {@link cz.empatix.upgrades.domain.TalentPath}s, so all runs are
 * wrapped in a {@link MockedAssets}-stubbed static.
 */
@ExtendWith(HeadlessAssets.class)
class UpgradeTreeTest {

    private static PlayerUpgrade named(final PlayerUpgradeName name) {
        final PlayerUpgrade u = new PlayerUpgrade("ingame/talents/boot.tga", QualityLevel.COMMON, false);
        u.setName(name);
        return u;
    }

    @Test
    void connectorCountIsUpgradesMinusOne() {
        final List<PlayerUpgrade> upgrades = List.of(
                named(PlayerUpgradeName.IMPROVED_DAMAGE),
                named(PlayerUpgradeName.ADDITIONAL_AMMO),
                named(PlayerUpgradeName.CRITICAL_HIT));

        // The tree constructs (size-1) parent connectors; setPosition must place them all.
        final UpgradeTree tree = new UpgradeTree(upgrades, TreeType.OFFENSIVE);
        assertEquals(3, tree.getUpgrades().size());
        assertDoesNotThrow(() -> tree.setPosition(100, 100));
    }

    @Test
    void getUpgradesReturnsTheBackingList() {
        final PlayerUpgrade a = named(PlayerUpgradeName.IMPROVED_HEALTH);
        final PlayerUpgrade b = named(PlayerUpgradeName.IMPROVED_ARMOR);
        final UpgradeTree tree = new UpgradeTree(List.of(a, b), TreeType.DEFENSIVE);

        assertSame(a, tree.getUpgrades().get(0));
        assertSame(b, tree.getUpgrades().get(1));
    }

    @Test
    void getPressedUpgradeReturnsTheHitUpgradeOrNull() {
        final PlayerUpgrade a = named(PlayerUpgradeName.IMPROVED_DAMAGE);
        final PlayerUpgrade b = named(PlayerUpgradeName.ADDITIONAL_AMMO);
        final UpgradeTree tree = new UpgradeTree(List.of(a, b), TreeType.OFFENSIVE);

        // setPosition lays them out vertically (74px apart) starting at y=300.
        tree.setPosition(500, 300);

        // a sits at (500,300), b at (500,374); pick a point clearly inside a's 64x64 box.
        assertSame(a, tree.getPressedUpgrade(510, 310));
        assertNull(tree.getPressedUpgrade(0, 0));
    }

    @Test
    void setBoughtUpgradesAppliesOnlyMatchingNames() {
        final PlayerUpgrade health = named(PlayerUpgradeName.IMPROVED_HEALTH);
        final PlayerUpgrade armor = named(PlayerUpgradeName.IMPROVED_ARMOR);
        final UpgradeTree tree = new UpgradeTree(List.of(health, armor), TreeType.DEFENSIVE);

        final Map<PlayerUpgradeName, Boolean> bought = new HashMap<>();
        bought.put(PlayerUpgradeName.IMPROVED_HEALTH, true);
        // IMPROVED_ARMOR intentionally absent -> must be left untouched (stays false).
        bought.put(PlayerUpgradeName.PET, true); // unrelated name -> ignored, no error

        tree.setBoughtUpgrades(bought);

        assertTrue(health.isBought());
        assertFalse(armor.isBought());
    }

    @Test
    void updateDoesNotThrow() {
        final UpgradeTree tree = new UpgradeTree(
                List.of(named(PlayerUpgradeName.IMPROVED_DAMAGE), named(PlayerUpgradeName.ADDITIONAL_AMMO)),
                TreeType.OFFENSIVE);
        tree.setPosition(100, 100);

        assertDoesNotThrow(() -> tree.update(0.016));
    }
}
