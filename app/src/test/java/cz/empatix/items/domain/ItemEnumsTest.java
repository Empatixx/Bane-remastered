package cz.empatix.items.domain;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;

/** Coverage for the small plain item enums (no behavior beyond value membership). */
class ItemEnumsTest {

    @Test
    void ammoBoxTypeHasExpectedConstants() {
        assertEquals(4, AmmoBoxType.values().length);
        for (final AmmoBoxType t : AmmoBoxType.values()) {
            assertSame(t, AmmoBoxType.valueOf(t.name()));
        }
        assertSame(AmmoBoxType.UNIVERSAL, AmmoBoxType.valueOf("UNIVERSAL"));
    }

    @Test
    void recoverTypeHasHealthAndArmor() {
        assertEquals(2, RecoverType.values().length);
        assertSame(RecoverType.HEALTH, RecoverType.valueOf("HEALTH"));
        assertSame(RecoverType.ARMOR, RecoverType.valueOf("ARMOR"));
    }

    @Test
    void recoverSizeHasSmallAndLarge() {
        assertEquals(2, RecoverSize.values().length);
        assertSame(RecoverSize.SMALL, RecoverSize.valueOf("SMALL"));
        assertSame(RecoverSize.LARGE, RecoverSize.valueOf("LARGE"));
    }

    @Test
    void dropSourceHasPlayerAndNormal() {
        assertEquals(2, DropSource.values().length);
        assertSame(DropSource.PLAYER, DropSource.valueOf("PLAYER"));
        assertSame(DropSource.NORMAL, DropSource.valueOf("NORMAL"));
    }
}
