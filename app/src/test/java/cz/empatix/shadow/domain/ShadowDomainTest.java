package cz.empatix.shadow.domain;

import cz.empatix.player.Player;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Vertex;
import org.joml.Vector2f;
import org.joml.Vector3f;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/**
 * Pure (GL-free, asset-free) coverage of the {@code shadow.domain} renderables. These classes only do
 * vertex math and visibility bookkeeping in {@code renderData()}/{@code canRender()}, so they are
 * constructed directly with mocked collaborators. Only the translated geometry, scene/z-index, and the
 * empty-vertices / caster guards are asserted, never any GL output.
 */
class ShadowDomainTest {

    private static Vertex vertex(final float x, final float y, final float z) {
        return new Vertex(new Vector3f(x, y, z), new Vector2f(0, 0));
    }

    private static List<Vertex> quad() {
        return List.of(
                vertex(0, 0, 1),
                vertex(10, 0, 1),
                vertex(10, 10, 1),
                vertex(0, 10, 1));
    }

    // ---- PlayerShadow ---------------------------------------------------------------------------

    @Test
    void playerShadowExposesSceneAndZIndex() {
        final PlayerShadow shadow = new PlayerShadow(quad());

        assertSame(RenderScene.SCENE, shadow.renderScene());
        assertEquals(3, shadow.getZIndex());
    }

    @Test
    void playerShadowRendersWhenItHasVertices() {
        assertTrue(new PlayerShadow(quad()).canRender());
        assertFalse(new PlayerShadow(List.of()).canRender());
    }

    @Test
    void playerShadowRenderDataKeepsVerticesAndUsesZeroTexture() {
        final List<Vertex> vertices = quad();
        final PlayerShadow shadow = new PlayerShadow(vertices);

        final RenderData data = shadow.renderData();

        assertEquals(0, data.textureId());
        assertSame(vertices, data.vertices());
    }

    // ---- ObjectShadow ---------------------------------------------------------------------------

    private static ShadowCaster caster(final float x, final float y, final int w, final int h,
                                       final float feetOffset, final boolean canRender) {
        final ShadowCaster caster = mock(ShadowCaster.class);
        when(caster.getShadowPosition()).thenReturn(new Vector2f(x, y));
        when(caster.getShadowWidth()).thenReturn(w);
        when(caster.getShadowHeight()).thenReturn(h);
        when(caster.getShadowFeetOffsetY()).thenReturn(feetOffset);
        when(caster.canRender()).thenReturn(canRender);
        return caster;
    }

    @Test
    void objectShadowSceneAndZIndex() {
        final ObjectShadow shadow = new ObjectShadow(caster(0, 0, 32, 32, 0f, true), quad(), 7, 0.5f);

        assertSame(RenderScene.SCENE, shadow.renderScene());
        assertEquals(2, shadow.getZIndex());
    }

    @Test
    void objectShadowTranslatesRelativeVerticesToCasterFeet() {
        // feetX = x + width/2 = 100 + 16 = 116 ; feetY = y + height + offset = 200 + 32 + 4 = 236
        final ShadowCaster caster = caster(100, 200, 32, 32, 4f, true);
        final ObjectShadow shadow = new ObjectShadow(caster, List.of(vertex(5, 6, 9)), 7, 1f);

        final RenderData data = shadow.renderData();

        assertEquals(7, data.textureId());
        assertEquals(1, data.vertices().size());
        final Vertex v = data.vertices().get(0);
        assertEquals(116f + 5f, v.position().x());
        assertEquals(236f + 6f, v.position().y());
        // z is carried through unchanged
        assertEquals(9f, v.position().z());
    }

    @Test
    void objectShadowRenderWhenVerticesPresentAndCasterRenders() {
        assertTrue(new ObjectShadow(caster(0, 0, 32, 32, 0f, true), quad(), 1, 1f).canRender());
        // empty vertices -> not rendered even though the caster can render
        assertFalse(new ObjectShadow(caster(0, 0, 32, 32, 0f, true), List.of(), 1, 1f).canRender());
        // caster cannot render -> shadow suppressed
        assertFalse(new ObjectShadow(caster(0, 0, 32, 32, 0f, false), quad(), 1, 1f).canRender());
    }

    // ---- PlayerTextureShadow --------------------------------------------------------------------

    @Test
    void playerTextureShadowSceneAndZIndex() {
        final Player player = mock(Player.class);
        final PlayerTextureShadow shadow = new PlayerTextureShadow(player, quad(), 4, 0.7f);

        assertSame(RenderScene.SCENE, shadow.renderScene());
        assertEquals(3, shadow.getZIndex());
    }

    @Test
    void playerTextureShadowTranslatesRelativeVerticesToPlayerFeet() {
        final Player player = mock(Player.class);
        when(player.getRenderPosition()).thenReturn(new Vector2f(50, 80));
        when(player.getWidth()).thenReturn(40);
        when(player.getHeight()).thenReturn(60);
        // feetX = 50 + 20 = 70 ; feetY = 80 + 60 = 140
        final PlayerTextureShadow shadow = new PlayerTextureShadow(player, List.of(vertex(3, 4, 2)), 4, 1f);

        final RenderData data = shadow.renderData();

        assertEquals(4, data.textureId());
        final Vertex v = data.vertices().get(0);
        assertEquals(70f + 3f, v.position().x());
        assertEquals(140f + 4f, v.position().y());
        assertEquals(2f, v.position().z());
    }

    @Test
    void playerTextureShadowRendersOnlyWithVertices() {
        final Player player = mock(Player.class);
        assertTrue(new PlayerTextureShadow(player, quad(), 4, 1f).canRender());
        assertFalse(new PlayerTextureShadow(player, List.of(), 4, 1f).canRender());
    }

    @Test
    void shadowCasterFeetOffsetDefaultsToZero() {
        // default method on the interface; an anonymous minimal implementation exercises it.
        final ShadowCaster caster = new ShadowCaster() {
            @Override
            public Vector2f getShadowPosition() {
                return new Vector2f();
            }

            @Override
            public int getShadowWidth() {
                return 0;
            }

            @Override
            public int getShadowHeight() {
                return 0;
            }

            @Override
            public RenderData renderData() {
                return null;
            }

            @Override
            public boolean canRender() {
                return false;
            }
        };

        assertEquals(0f, caster.getShadowFeetOffsetY());
    }
}
