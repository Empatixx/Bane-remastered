package cz.empatix.tiles.factory;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertSame;

class TileOrientationTest {

    @Test
    void valueOfRoundTripsForEveryConstant() {
        for (final TileOrientation o : TileOrientation.values()) {
            assertSame(o, TileOrientation.valueOf(o.name()));
        }
    }

    @Test
    void containsExpectedNumberOfOrientations() {
        assertEquals(15, TileOrientation.values().length);
    }

    @Test
    void keyOrientationsArePresent() {
        assertNotNull(TileOrientation.CENTER);
        assertNotNull(TileOrientation.TOP_LEFT);
        assertNotNull(TileOrientation.BOTTOM_RIGHT_OUTER_CORNER);
        assertNotNull(TileOrientation.TOP_LEFT_INNER_CORNER);
    }
}
