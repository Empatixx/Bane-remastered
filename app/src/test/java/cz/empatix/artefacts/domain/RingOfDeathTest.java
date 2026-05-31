package cz.empatix.artefacts.domain;

import cz.empatix.player.Player;
import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/**
 * Covers the burst behaviour of {@link RingOfDeath}: activation spawns a fixed ring of bullets that
 * surface through {@code getRenderables}. Bullet construction needs both assets and a physics world.
 */
@ExtendWith(HeadlessAssets.class)
class RingOfDeathTest {

    private Player playerAt(final float x, final float y) {
        final Player player = mock(Player.class);
        when(player.getPosition()).thenReturn(new Vector2f(x, y));
        return player;
    }

    @Test
    void noBulletsBeforeActivation() {
        final RingOfDeath artefact = new RingOfDeath(playerAt(100, 100));

        assertTrue(artefact.getRenderables().isEmpty());
        assertEquals("ingame/artefacts/ringofdeath.tga", artefact.getTexturePath());
        assertEquals(1, artefact.hudToRender().size());
    }

    @Test
    void activateSpawnsFiftyBullets() {
        final RingOfDeath artefact = new RingOfDeath(playerAt(200, 150));

        artefact.activate();

        assertEquals(50, artefact.getRenderables().size());
    }

    @Test
    void updateKeepsFreshBulletsAlive() {
        final RingOfDeath artefact = new RingOfDeath(playerAt(0, 0));
        artefact.activate();

        // A single update tick cannot complete a collision animation, so none are pruned.
        artefact.update(0.016);
        artefact.fixedUpdate(0.016);

        assertEquals(50, artefact.getRenderables().size());
    }
}
