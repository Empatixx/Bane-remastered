package cz.empatix.upgrades.domain;

import cz.empatix.testsupport.HeadlessAssets;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotSame;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Domain-logic coverage of {@link PlayerUpgrade}: purchase gating ({@code canBuy}/parent chain),
 * bought-state toggling, getters/setters, hit-testing ({@code isPressed}) and {@code deepCopy}.
 * The constructor fetches the talent-frame texture through {@code AssetsService}, so each test
 * runs inside a {@code mockStatic} wired by {@link MockedAssets}. Render-output methods that touch
 * GL are intentionally not exercised.
 */
@ExtendWith(HeadlessAssets.class)
class PlayerUpgradeTest {

    private static PlayerUpgrade upgrade(final QualityLevel level, final boolean bought) {
        return new PlayerUpgrade("ingame/talents/boot.tga", level, bought);
    }

    @Test
    void freshUpgradeHasDefaultsAndUnknownName() {
        final PlayerUpgrade u = upgrade(QualityLevel.COMMON, false);

        assertEquals("", u.getTitle());
        assertEquals("", u.getDescription());
        assertEquals(0, u.getPrice());
        assertNull(u.getParent());
        assertFalse(u.isBought());
        assertSame(PlayerUpgradeName.UNKNOWN, u.getName());
        assertEquals(64, u.getWidth());  // 32 * SCALE(2)
        assertEquals(64, u.getHeight());
    }

    @Test
    void settersRoundTrip() {
        final PlayerUpgrade u = upgrade(QualityLevel.RARE, false);
        u.setTitle("Trap Master");
        u.setDescription("immobilize traps");
        u.setPrice(140);
        u.setName(PlayerUpgradeName.TRAP_MASTER);

        assertEquals("Trap Master", u.getTitle());
        assertEquals("immobilize traps", u.getDescription());
        assertEquals(140, u.getPrice());
        assertSame(PlayerUpgradeName.TRAP_MASTER, u.getName());
    }

    @Test
    void rootUpgradeWithoutParentCanAlwaysBeBought() {
        assertTrue(upgrade(QualityLevel.COMMON, false).canBuy());
    }

    @Test
    void childCanOnlyBeBoughtOnceParentIsBought() {
        final PlayerUpgrade parent = upgrade(QualityLevel.COMMON, false);
        final PlayerUpgrade child = upgrade(QualityLevel.RARE, false);
        child.setParent(parent);

        assertSame(parent, child.getParent());
        assertFalse(child.canBuy());     // parent not bought yet

        parent.setBought(true);
        assertTrue(child.canBuy());      // unlocked once parent bought
    }

    @Test
    void setBoughtTogglesBoughtFlag() {
        final PlayerUpgrade u = upgrade(QualityLevel.EPIC, false);
        assertFalse(u.isBought());

        u.setBought(true);
        assertTrue(u.isBought());

        u.setBought(false);
        assertFalse(u.isBought());
    }

    @Test
    void isPressedTestsTheUpgradeBoundingBox() {
        final PlayerUpgrade u = upgrade(QualityLevel.COMMON, false);
        u.setPosition(100, 200);

        assertTrue(u.isPressed(100, 200));        // top-left corner
        assertTrue(u.isPressed(132, 232));        // inside (64x64 box)
        assertTrue(u.isPressed(164, 264));        // bottom-right corner (inclusive)
        assertFalse(u.isPressed(99, 200));        // just left
        assertFalse(u.isPressed(100, 265));       // just below
    }

    @Test
    void deepCopyClonesStateButIsADistinctInstance() {
        final PlayerUpgrade parent = upgrade(QualityLevel.COMMON, true);
        final PlayerUpgrade u = upgrade(QualityLevel.RARE, true);
        u.setTitle("Armor");
        u.setDescription("more armor");
        u.setPrice(60);
        u.setParent(parent);

        final PlayerUpgrade copy = u.deepCopy();

        assertNotSame(u, copy);
        assertEquals("Armor", copy.getTitle());
        assertEquals("more armor", copy.getDescription());
        assertEquals(60, copy.getPrice());
        assertSame(parent, copy.getParent());
        assertTrue(copy.isBought());
    }
}
