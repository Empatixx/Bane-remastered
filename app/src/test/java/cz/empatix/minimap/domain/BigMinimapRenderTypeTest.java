package cz.empatix.minimap.domain;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

/** Covers the alpha mapping of the {@link BigMinimapRenderType} enum (no assets required). */
class BigMinimapRenderTypeTest {

    @Test
    void currentIsFullyOpaque() {
        assertEquals(1f, BigMinimapRenderType.CURRENT.alpha());
    }

    @Test
    void unseenIsFullyTransparent() {
        assertEquals(0f, BigMinimapRenderType.UNSEEN.alpha());
    }

    @Test
    void alphaDecreasesWithLessVisibility() {
        // CURRENT > VISITED > UNVISITED > UNSEEN
        assertTrue(BigMinimapRenderType.CURRENT.alpha() > BigMinimapRenderType.VISITED.alpha());
        assertTrue(BigMinimapRenderType.VISITED.alpha() > BigMinimapRenderType.UNVISITED.alpha());
        assertTrue(BigMinimapRenderType.UNVISITED.alpha() > BigMinimapRenderType.UNSEEN.alpha());
    }

    @Test
    void intermediateAlphasMatchSpec() {
        assertEquals(0.75f, BigMinimapRenderType.VISITED.alpha());
        assertEquals(0.5f, BigMinimapRenderType.UNVISITED.alpha());
    }
}
