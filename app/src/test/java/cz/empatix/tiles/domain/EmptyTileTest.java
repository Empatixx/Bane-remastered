package cz.empatix.tiles.domain;

import cz.empatix.render.domain.RenderScene;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;

/** Pure-logic coverage for the GL-free {@link EmptyTile} placeholder record. */
class EmptyTileTest {

    @Test
    void emptyTileIsNeverRenderedAndSitsOnTheBaseLayer() {
        final EmptyTile tile = new EmptyTile();

        assertFalse(tile.canRender());
        assertEquals(0, tile.getZIndex());
        assertEquals(RenderScene.SCENE, tile.renderScene());
    }

    @Test
    void renderDataIsUnsupportedForAnEmptyTile() {
        final EmptyTile tile = new EmptyTile();

        assertThrows(IllegalStateException.class, tile::renderData);
    }

    @Test
    void destroyIsANoOp() {
        final EmptyTile tile = new EmptyTile();

        tile.destroy(); // must not throw
    }
}
