package cz.empatix.particles.service;

import cz.empatix.particles.domain.GazewindParticle;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.lang.reflect.Field;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Headless coverage for {@link GazewindService}: a cooldown-gated spawner of gazewind particles.
 * Particles fetch a GL-free texture (see {@link HeadlessAssets}). The 200ms spawn cooldown is forced
 * open by back-dating {@code lastGazeWindTime} via reflection (no sleeping); particle bookkeeping is
 * asserted through the private list and the renderable collection, never via GL.
 */
@ExtendWith(HeadlessAssets.class)
class GazewindServiceTest {

    @SuppressWarnings("unchecked")
    private static List<GazewindParticle> particles(final GazewindService service) throws Exception {
        final Field field = GazewindService.class.getDeclaredField("gazewindParticles");
        field.setAccessible(true);
        return (List<GazewindParticle>) field.get(service);
    }

    /** Pushes the last-spawn timestamp far into the past so the next spawn is not on cooldown. */
    private static void clearCooldown(final GazewindService service) throws Exception {
        final Field field = GazewindService.class.getDeclaredField("lastGazeWindTime");
        field.setAccessible(true);
        field.setLong(service, 0L);
    }

    @Test
    void spawnAddsAParticleWhenOffCooldown() throws Exception {
        final GazewindService service = new GazewindService();

        service.spawn(100f, 200f);

        assertEquals(1, particles(service).size());
    }

    @Test
    void backToBackSpawnIsSuppressedByTheCooldown() throws Exception {
        final GazewindService service = new GazewindService();

        service.spawn(100f, 200f);
        service.spawn(110f, 210f); // within 200ms cooldown -> ignored

        assertEquals(1, particles(service).size());
    }

    @Test
    void spawnAgainAfterTheCooldownElapses() throws Exception {
        final GazewindService service = new GazewindService();
        service.spawn(100f, 200f);

        clearCooldown(service);
        service.spawn(120f, 220f);

        assertEquals(2, particles(service).size());
    }

    @Test
    void spawnCentersTheParticleOnTheRequestedPoint() throws Exception {
        final GazewindService service = new GazewindService();

        service.spawn(100f, 200f);

        final GazewindParticle particle = particles(service).get(0);
        // particle is offset by half its footprint so its centre sits on (100, 200)
        assertEquals(100f - particle.getWidth() / 2f, particle.transform().x, 0.0001f);
        assertEquals(200f - particle.getHeight() / 2f, particle.transform().y, 0.0001f);
    }

    @Test
    void clearRemovesAllParticles() throws Exception {
        final GazewindService service = new GazewindService();
        service.spawn(100f, 200f);

        service.clear();

        assertTrue(particles(service).isEmpty());
    }

    @Test
    void updateKeepsUnfinishedParticles() throws Exception {
        final GazewindService service = new GazewindService();
        service.spawn(100f, 200f);

        service.update(0.016f); // fresh particle has not finished playing -> kept

        assertEquals(1, particles(service).size());
    }

    @Test
    void collectSubRenderablesExposesEveryLiveParticle() throws Exception {
        final GazewindService service = new GazewindService();
        service.spawn(100f, 200f);
        clearCooldown(service);
        service.spawn(150f, 250f);

        final RenderableState state = new RenderableState(8);
        service.collectSubRenderables(state);

        assertEquals(2, state.getList().size());
    }
}
