package cz.empatix.rooms.service;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.testsupport.MockedAssets;
import cz.empatix.tiles.domain.TileImpl;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;

/**
 * Headless coverage for the real {@link PathRoom}: the tile-map-driven width/height, the explicit
 * min/max bounds passed at construction (used by the camera), the PATH room type and the
 * direction-aware {@link PathRoom#isInRoom} intersection rule that widens the corridor mouth along
 * its travel axis.
 */
class PathRoomTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new PhysicsService();
    }

    private static final int SCALED = TileImpl.TILE_SIZE * TileImpl.SCALE;

    @Test
    void widthHeightFollowTheBackingTileMapAndBoundsEchoConstructorArgs() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            // horizontal corridor, 4 rows x 6 cols, offset (16, 48)
            final PathRoom room = new PathRoom(16, 48, 4, 6,
                    10, 20, 300, 400, true, RoomDirection.RIGHT);

            assertEquals(6 * SCALED, room.getWidth());
            assertEquals(4 * SCALED, room.getHeight());
            assertEquals(16, room.getX());
            assertEquals(48, room.getY());
            assertEquals(10, room.getMinX());
            assertEquals(20, room.getMinY());
            assertEquals(300, room.getMaxX());
            assertEquals(400, room.getMaxY());
            assertEquals(RoomType.PATH, room.getRoomType());
            assertTrue(room.getRoomObjects().isEmpty());
        }
    }

    @Test
    void horizontalCorridorExtendsTheXAcceptanceWindowByOneTile() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            // RIGHT travel -> xRoomIntersectionOffset = +SCALED. With offset 0 the accepted X
            // window starts at SCALED and ends at width - SCALED.
            final PathRoom room = new PathRoom(0, 0, 4, 6,
                    0, 0, 0, 0, true, RoomDirection.RIGHT);

            final float midY = 2 * SCALED;
            // Just inside the left edge of the widened window.
            assertTrue(room.isInRoom(SCALED, midY));
            // Just before the window starts -> rejected.
            assertFalse(room.isInRoom(SCALED - 1, midY));
        }
    }

    @Test
    void getNeighboursMapIsUnsupportedForACorridor() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final PathRoom room = new PathRoom(0, 0, 4, 6,
                    0, 0, 0, 0, false, RoomDirection.TOP);

            // addNeighbour is a documented no-op; the getter rejects access.
            room.addNeighbour(RoomDirection.LEFT, mock(Room.class));
            assertThrows(IllegalStateException.class, room::getNeighboursMap);
        }
    }
}
