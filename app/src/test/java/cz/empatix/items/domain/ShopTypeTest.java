package cz.empatix.items.domain;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;

class ShopTypeTest {

    @Test
    void multipliersMatchSpec() {
        assertEquals(2f, ShopType.OFFENSIVE.getMultiplier());
        assertEquals(1f, ShopType.AMMUNITION.getMultiplier());
        assertEquals(1.5f, ShopType.DEFENSIVE.getMultiplier());
    }

    @Test
    void everyTypeHasAPositiveMultiplier() {
        for (final ShopType type : ShopType.values()) {
            assertEquals(true, type.getMultiplier() > 0f);
        }
    }

    @Test
    void valueOfRoundTrips() {
        for (final ShopType t : ShopType.values()) {
            assertSame(t, ShopType.valueOf(t.name()));
        }
    }
}
