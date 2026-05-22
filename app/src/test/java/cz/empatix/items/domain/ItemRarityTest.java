package cz.empatix.items.domain;

import org.junit.jupiter.api.Test;

import java.util.EnumSet;
import java.util.Set;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

class ItemRarityTest {

    @Test
    void weightsMatchDeclaredValues() {
        assertEquals(60, ItemRarity.COMMON.getWeight());
        assertEquals(25, ItemRarity.UNCOMMON.getWeight());
        assertEquals(15, ItemRarity.RARE.getWeight());
        assertEquals(0, ItemRarity.UNDEFINED.getWeight());
    }

    @Test
    void positivelyWeightedRaritiesArePositive() {
        assertTrue(ItemRarity.COMMON.getWeight() > 0);
        assertTrue(ItemRarity.UNCOMMON.getWeight() > 0);
        assertTrue(ItemRarity.RARE.getWeight() > 0);
    }

    @Test
    void randomRarityWithSingleOptionAlwaysReturnsThatOption() {
        for (int i = 0; i < 50; i++) {
            assertSame(ItemRarity.COMMON, ItemRarity.randomRarity(ItemRarity.COMMON));
        }
    }

    @Test
    void randomRarityOnlyReturnsOneOfTheProvidedOptions() {
        final Set<ItemRarity> allowed =
                EnumSet.of(ItemRarity.COMMON, ItemRarity.UNCOMMON, ItemRarity.RARE);

        for (int i = 0; i < 1000; i++) {
            final ItemRarity result = ItemRarity.randomRarity(
                    ItemRarity.COMMON, ItemRarity.UNCOMMON, ItemRarity.RARE);
            assertTrue(allowed.contains(result),
                    "randomRarity returned an unexpected value: " + result);
        }
    }

    @Test
    void allWeightedRaritiesAreReachable() {
        final Set<ItemRarity> seen = EnumSet.noneOf(ItemRarity.class);
        // With these weights every option has substantial probability;
        // many draws should hit all three at least once.
        for (int i = 0; i < 5000 && seen.size() < 3; i++) {
            seen.add(ItemRarity.randomRarity(
                    ItemRarity.COMMON, ItemRarity.UNCOMMON, ItemRarity.RARE));
        }
        assertTrue(seen.contains(ItemRarity.COMMON));
        assertTrue(seen.contains(ItemRarity.UNCOMMON));
        assertTrue(seen.contains(ItemRarity.RARE));
    }

    @Test
    void valueOfRoundTrips() {
        for (final ItemRarity r : ItemRarity.values()) {
            assertSame(r, ItemRarity.valueOf(r.name()));
        }
    }
}
