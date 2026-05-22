package cz.empatix.map.domain;

import org.junit.jupiter.api.Test;

import java.util.EnumSet;
import java.util.Set;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

class RoomDirectionTest {

    @Test
    void getOppositeReturnsTheReverse() {
        assertEquals(RoomDirection.BOTTOM, RoomDirection.TOP.getOpposite());
        assertEquals(RoomDirection.TOP, RoomDirection.BOTTOM.getOpposite());
        assertEquals(RoomDirection.RIGHT, RoomDirection.LEFT.getOpposite());
        assertEquals(RoomDirection.LEFT, RoomDirection.RIGHT.getOpposite());
    }

    @Test
    void oppositeIsInvolution() {
        for (final RoomDirection d : RoomDirection.values()) {
            assertEquals(d, d.getOpposite().getOpposite());
        }
    }

    @Test
    void horizontalAndVerticalClassification() {
        assertTrue(RoomDirection.LEFT.isHorizontal());
        assertTrue(RoomDirection.RIGHT.isHorizontal());
        assertFalse(RoomDirection.TOP.isHorizontal());
        assertFalse(RoomDirection.BOTTOM.isHorizontal());

        assertTrue(RoomDirection.TOP.isVertical());
        assertTrue(RoomDirection.BOTTOM.isVertical());
        assertFalse(RoomDirection.LEFT.isVertical());
        assertFalse(RoomDirection.RIGHT.isVertical());
    }

    @Test
    void everyDirectionIsExactlyHorizontalOrVertical() {
        for (final RoomDirection d : RoomDirection.values()) {
            assertTrue(d.isHorizontal() ^ d.isVertical(),
                    d + " must be exactly one of horizontal/vertical");
        }
    }

    @Test
    void oppositeKeepsAxis() {
        for (final RoomDirection d : RoomDirection.values()) {
            assertEquals(d.isHorizontal(), d.getOpposite().isHorizontal());
            assertEquals(d.isVertical(), d.getOpposite().isVertical());
        }
    }

    @Test
    void getRandomAlwaysReturnsAValidDirection() {
        for (int i = 0; i < 200; i++) {
            assertNotNull(RoomDirection.getRandom());
        }
    }

    @Test
    void getRandomWithoutNeverReturnsExcludedDirections() {
        final Set<RoomDirection> excluded = EnumSet.of(RoomDirection.TOP, RoomDirection.LEFT);
        for (int i = 0; i < 500; i++) {
            final RoomDirection r = RoomDirection.getRandomWithout(RoomDirection.TOP, RoomDirection.LEFT);
            assertFalse(excluded.contains(r));
        }
    }

    @Test
    void getRandomWithoutSingleExclusionReturnsAllowedRange() {
        final Set<RoomDirection> produced = EnumSet.noneOf(RoomDirection.class);
        for (int i = 0; i < 500; i++) {
            produced.add(RoomDirection.getRandomWithout(RoomDirection.BOTTOM));
        }
        assertFalse(produced.contains(RoomDirection.BOTTOM));
        // statistically the other three should all appear
        assertEquals(EnumSet.of(RoomDirection.TOP, RoomDirection.LEFT, RoomDirection.RIGHT), produced);
    }
}
