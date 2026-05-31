package cz.empatix.shadow.service;

import cz.empatix.lightning.domain.LightSource;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.shadow.domain.PlayerTextureShadow;
import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector3f;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Headless coverage for {@link ShadowService} driven by a REAL {@link Player}: the strongest-light
 * selection that excludes the player's own light, the intensity fade-in/fade-out gate
 * ({@code shadowIntensity < 0.01 -> no shadow}), and the early-out when no qualifying light exists.
 * The {@link LightningService} singleton is cleared per test; a large {@code dt} is used so the
 * interpolated intensity reaches its target in a single tick. Only shadow presence/count is asserted.
 */
@ExtendWith(HeadlessAssets.class)
class ShadowServiceTest {

    @BeforeEach
    void resetWorld() {
        LightningService.getInstance().clear();
    }

    private static LightSource externalLightAt(final float x, final float y, final float intensity) {
        final LightSource light = LightningService.getInstance()
                .createLightSource(new Vector3f(255, 255, 255), 100f, intensity);
        light.setPosition(x, y);
        return light;
    }

    @Test
    void noExternalLightYieldsNoShadow() {
        final Player player = new Player();
        player.setPosition(0, 0);
        // Only the player's own light exists (created in Player ctor); it is excluded as a caster.
        final ShadowService service = new ShadowService(player);

        final List<Renderable> shadows = service.toRender(1.0f);

        assertTrue(shadows.isEmpty(), "the player's own light cannot cast the player's shadow");
    }

    @Test
    void aNearbyExternalLightCastsAPlayerShadow() {
        final Player player = new Player();
        player.setPosition(300, 300);
        externalLightAt(320, 320, 1.0f); // close, strong, displayable
        final ShadowService service = new ShadowService(player);

        // large dt -> interpolated intensity jumps straight to its (positive) target this tick
        final List<Renderable> shadows = service.toRender(1.0f);

        assertEquals(1, shadows.size());
        assertInstanceOf(PlayerTextureShadow.class, shadows.get(0));
    }

    @Test
    void aLightBeyondMaxDistanceProducesNoShadow() {
        final Player player = new Player();
        player.setPosition(0, 0);
        externalLightAt(10_000, 10_000, 1.0f); // beyond MAX_SHADOW_DISTANCE
        final ShadowService service = new ShadowService(player);

        final List<Renderable> shadows = service.toRender(1.0f);

        assertTrue(shadows.isEmpty());
    }

    @Test
    void shadowFadesOutAfterTheLightIsRemoved() {
        final Player player = new Player();
        player.setPosition(300, 300);
        final LightSource light = externalLightAt(320, 320, 1.0f);
        final ShadowService service = new ShadowService(player);

        // build up a shadow first
        assertEquals(1, service.toRender(1.0f).size());

        // remove the light and let intensity interpolate back down to ~0 over many ticks
        light.destroy();
        LightningService.getInstance().update();
        List<Renderable> shadows = List.of();
        for (int i = 0; i < 50; i++) {
            shadows = service.toRender(1.0f);
        }

        assertTrue(shadows.isEmpty(), "with no light the shadow intensity decays below the render threshold");
    }

    @Test
    void updateIsANoOpAndDoesNotThrow() {
        final Player player = new Player();
        final ShadowService service = new ShadowService(player);

        service.update(); // documented no-op; just exercise the method

        assertTrue(service.toRender(0.016f).isEmpty());
    }
}
