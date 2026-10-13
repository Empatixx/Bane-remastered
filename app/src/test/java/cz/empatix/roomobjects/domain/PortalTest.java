package cz.empatix.roomobjects.domain;

import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.gamestate.service.GameStateController;
import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.Set;
import java.util.concurrent.atomic.AtomicReference;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Headless coverage for the real {@link Portal} (a {@code RoomObject} + {@code Interactable}). The
 * constructor builds a real (headless-safe) light source and registers with the interaction service.
 * These tests drive the update light-follow path and the interaction handler, which on the E key
 * requests the INGAME state via {@link GameStateController} (wired here to a capturing consumer).
 */
@ExtendWith(HeadlessAssets.class)
class PortalTest {

    @Test
    void interactWithEKeyRequestsIngameState() {
        final AtomicReference<GameStateType> requested = new AtomicReference<>();
        GameStateController.init(requested::set);
        final Portal portal = new Portal(new Vector2f(10f, 20f));

        portal.interact(Set.of(org.lwjgl.glfw.GLFW.GLFW_KEY_E));

        assertEquals(GameStateType.INGAME, requested.get());
    }

    @Test
    void interactWithoutEKeyDoesNothing() {
        final AtomicReference<GameStateType> requested = new AtomicReference<>();
        GameStateController.init(requested::set);
        final Portal portal = new Portal(new Vector2f());

        portal.interact(Set.of(org.lwjgl.glfw.GLFW.GLFW_KEY_A));

        assertNull(requested.get());
        assertTrue(portal.canInteract(Set.of()));
    }

    @Test
    void updateFollowsLightAndGeometryIsStable() {
        final Portal portal = new Portal(new Vector2f(5f, 6f));

        for (int i = 0; i < 5; i++) {
            portal.update(0.05);
        }

        assertEquals(96 * 2, portal.getWidth());
        assertEquals(90 * 2, portal.getHeight());
        assertEquals(5f, portal.getPosition().x, 0.0001f);
        assertEquals(6f, portal.getPosition().y, 0.0001f);
        assertFalse(portal.canRemove());
    }
}
