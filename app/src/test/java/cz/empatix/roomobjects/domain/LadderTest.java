package cz.empatix.roomobjects.domain;

import cz.empatix.common.RngService;
import cz.empatix.death.service.StatisticsService;
import cz.empatix.death.service.VictoryService;
import cz.empatix.enemies.service.DifficultyService;
import cz.empatix.items.service.ItemService;
import cz.empatix.map.service.MapLoaderService;
import cz.empatix.player.CoinService;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.Set;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.verifyNoInteractions;

/**
 * Headless coverage for the real {@link Ladder} (a {@code RoomObject} + {@code Interactable}). The
 * E-key interaction triggers a heavyweight procedural map regeneration (out of scope headlessly), so
 * these tests assert the always-true interaction predicate, the update/animation path, and that a
 * non-E interaction is a no-op that never touches the injected services.
 */
@ExtendWith(HeadlessAssets.class)
class LadderTest {

    private static Ladder newLadder(final MapLoaderService mapLoaderService,
                                    final ItemService itemService,
                                    final StatisticsService statisticsService,
                                    final CoinService coinService) {
        return new Ladder(mapLoaderService, itemService, statisticsService, coinService, new DifficultyService(), new RngService(0L), new VictoryService());
    }

    @Test
    void canInteractIsAlwaysTrueAndNotRemovable() {
        final Ladder ladder = newLadder(mock(MapLoaderService.class), mock(ItemService.class),
                mock(StatisticsService.class), new CoinService());

        assertTrue(ladder.canInteract(Set.of()));
        assertFalse(ladder.canRemove());
    }

    @Test
    void nonEnterInteractionDoesNotRegenerateMap() {
        final MapLoaderService mapLoaderService = mock(MapLoaderService.class);
        final ItemService itemService = mock(ItemService.class);
        final StatisticsService statisticsService = mock(StatisticsService.class);
        final Ladder ladder = newLadder(mapLoaderService, itemService, statisticsService, new CoinService());

        ladder.interact(Set.of(org.lwjgl.glfw.GLFW.GLFW_KEY_A));

        verifyNoInteractions(mapLoaderService, itemService, statisticsService);
    }

    @Test
    void updateAndGeometry() {
        final Ladder ladder = newLadder(mock(MapLoaderService.class), mock(ItemService.class),
                mock(StatisticsService.class), new CoinService());
        ladder.setPosition(70f, 80f);

        for (int i = 0; i < 5; i++) {
            ladder.update(0.05);
        }

        assertEquals(32 * 3, ladder.getWidth());
        assertEquals(32 * 3, ladder.getHeight());
        assertEquals(70f, ladder.getPosition().x, 0.0001f);
        assertEquals(80f, ladder.getPosition().y, 0.0001f);
    }
}
