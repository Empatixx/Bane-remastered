package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.items.service.ItemService;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;
import static org.mockito.Mockito.verifyNoInteractions;

/**
 * Headless coverage for the real {@link Chest}. The open transition is driven by a dyn4j PLAYER
 * collision callback which is not reachable headlessly (the chest does not expose its physics body),
 * so these tests assert the closed-state invariants: a freshly built chest is not removable, never
 * drops items while closed, and reports stable geometry for both {@link ChestType} variants.
 */
class ChestTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new PhysicsService();
    }

    @Test
    void freshChestIsClosedAndNotRemovable() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final ItemService itemService = mock(ItemService.class);
            final Chest chest = new Chest(itemService, ChestType.WEAPON);

            assertFalse(chest.canRemove());
        }
    }

    @Test
    void closedChestAnimatesWithoutDroppingItems() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final ItemService itemService = mock(ItemService.class);
            final Chest chest = new Chest(itemService, ChestType.ARTEFACT);

            for (int i = 0; i < 20; i++) {
                chest.update(0.05);
            }

            assertFalse(chest.canRemove());
            verifyNoInteractions(itemService); // no factory pull, no drops while closed
        }
    }

    @Test
    void geometryAndPositionAreConsistentForBothTypes() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            for (final ChestType type : ChestType.values()) {
                final Chest chest = new Chest(mock(ItemService.class), type);
                chest.setPosition(40f, 80f);

                assertEquals(16 * 4, chest.getWidth());
                assertEquals(16 * 4, chest.getHeight());
                assertEquals(40f, chest.getPosition().x, 0.0001f);
                assertEquals(80f, chest.getPosition().y, 0.0001f);
            }
        }
    }
}
