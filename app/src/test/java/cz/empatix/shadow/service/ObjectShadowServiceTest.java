package cz.empatix.shadow.service;

import cz.empatix.audio.init.ALInit;
import cz.empatix.lightning.domain.LightSource;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.shadow.domain.ObjectShadow;
import cz.empatix.shadow.domain.ShadowCaster;
import org.joml.Vector2f;
import org.joml.Vector3f;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/**
 * Headless coverage for {@link ObjectShadowService}: the O(lights x casters) strongest-light search,
 * the per-caster shadow geometry construction, and the early-out guards (no casters, no light, casters
 * that can't render, casters beyond {@code MAX_SHADOW_DISTANCE}). Casters are mocked with fixed
 * positions and 4-vertex render data; only shadow counts and geometry presence are asserted, never
 * exact float values.
 */
class ObjectShadowServiceTest {

    private static final float MAX_SHADOW_DISTANCE = 400f;

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void resetLights() {
        LightningService.getInstance().clear();
    }

    /** A nearby, displayable light placed at the given world coordinates. */
    private static LightSource lightAt(final float x, final float y, final float intensity) {
        final LightSource light = LightningService.getInstance()
                .createLightSource(new Vector3f(255, 255, 255), 100f, intensity);
        light.setPosition(x, y);
        return light;
    }

    /** A caster with a quad (4 vertices) of render data at the given feet position. */
    private static ShadowCaster casterAt(final float x, final float y, final boolean canRender) {
        final ShadowCaster caster = mock(ShadowCaster.class);
        when(caster.getShadowPosition()).thenReturn(new Vector2f(x, y));
        when(caster.getShadowWidth()).thenReturn(32);
        when(caster.getShadowHeight()).thenReturn(32);
        when(caster.getShadowFeetOffsetY()).thenReturn(0f);
        when(caster.canRender()).thenReturn(canRender);
        when(caster.renderData()).thenReturn(new RenderData(quad(), 7, null));
        return caster;
    }

    private static List<Vertex> quad() {
        return List.of(
                new Vertex(new Vector3f(0, 0, 0), new Vector2f(0, 0)),
                new Vertex(new Vector3f(10, 0, 0), new Vector2f(1, 0)),
                new Vertex(new Vector3f(10, 10, 0), new Vector2f(1, 1)),
                new Vertex(new Vector3f(0, 10, 0), new Vector2f(0, 1)));
    }

    @Test
    void emptyCasterListProducesNoShadows() {
        final ObjectShadowService service = new ObjectShadowService();

        final List<Renderable> shadows = service.computeShadows(List.of(), 0.016f);

        assertTrue(shadows.isEmpty());
    }

    @Test
    void noLightMeansNoShadows() {
        // no lights registered at all
        final ObjectShadowService service = new ObjectShadowService();

        final List<Renderable> shadows = service.computeShadows(List.of(casterAt(0, 0, true)), 0.016f);

        assertTrue(shadows.isEmpty());
    }

    @Test
    void oneVisibleCasterNearALightYieldsOneShadow() {
        lightAt(50, 50, 1.0f);
        final ObjectShadowService service = new ObjectShadowService();

        final List<Renderable> shadows = service.computeShadows(List.of(casterAt(200, 200, true)), 0.016f);

        assertEquals(1, shadows.size());
        assertInstanceOf(ObjectShadow.class, shadows.get(0));
    }

    @Test
    void casterThatCannotRenderIsSkipped() {
        lightAt(50, 50, 1.0f);
        final ObjectShadowService service = new ObjectShadowService();

        final List<Renderable> shadows = service.computeShadows(List.of(casterAt(200, 200, false)), 0.016f);

        assertTrue(shadows.isEmpty(), "an invisible caster casts no shadow and contributes no light strength");
    }

    @Test
    void casterFartherThanMaxDistanceFromEveryLightCastsNoShadow() {
        lightAt(0, 0, 1.0f);
        final ObjectShadowService service = new ObjectShadowService();

        // well beyond MAX_SHADOW_DISTANCE (400) from the only light
        final List<Renderable> shadows = service.computeShadows(List.of(casterAt(5_000, 5_000, true)), 0.016f);

        assertTrue(shadows.isEmpty());
    }

    @Test
    void everyVisibleCasterInRangeGetsItsOwnShadow() {
        lightAt(0, 0, 1.0f);
        final ObjectShadowService service = new ObjectShadowService();

        final List<ShadowCaster> casters = List.of(
                casterAt(100, 100, true),
                casterAt(150, 120, true),
                casterAt(180, 90, true));

        final List<Renderable> shadows = service.computeShadows(casters, 0.016f);

        assertEquals(3, shadows.size());
    }

    @Test
    void invisibleCastersAreExcludedFromTheShadowCount() {
        lightAt(0, 0, 1.0f);
        final ObjectShadowService service = new ObjectShadowService();

        final List<ShadowCaster> casters = List.of(
                casterAt(100, 100, true),
                casterAt(150, 120, false),
                casterAt(180, 90, true));

        final List<Renderable> shadows = service.computeShadows(casters, 0.016f);

        assertEquals(2, shadows.size(), "only the two renderable casters produce shadows");
    }

    @Test
    void destroyedLightsAreNotConsideredForStrength() {
        final LightSource light = lightAt(50, 50, 1.0f);
        light.destroy(); // canDisplay() -> false
        final ObjectShadowService service = new ObjectShadowService();

        final List<Renderable> shadows = service.computeShadows(List.of(casterAt(200, 200, true)), 0.016f);

        assertTrue(shadows.isEmpty());
    }
}
