package cz.empatix.roomobjects.resolvers;

import cz.empatix.collision.CollisionType;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.roomobjects.domain.RoomObject;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.Room;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.geometry.MassType;
import org.dyn4j.geometry.Rectangle;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Map;
import java.util.function.Predicate;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

class PositionPredicateFactoryTest {

    private PhysicsService physics;

    @BeforeEach
    void setUp() {
        physics = new PhysicsService();
    }

    /** Adds a body occupying [x, x+size] x [y, y+size]. */
    private void addBody(final CollisionType type, final int x, final int y, final int size) {
        final PhysicsObject body = new PhysicsObject();
        final Rectangle rectangle = new Rectangle(size, size);
        rectangle.translate(size / 2f, size / 2f);
        body.addFixture(new BodyFixture(rectangle));
        body.setMass(MassType.NORMAL);
        body.setType(type);
        body.setUserData(type);
        body.setTranslation(x, y);
        physics.add(body);
        physics.updateBroadphase();
    }

    // ---- withoutAllCollisions -------------------------------------------------

    @Test
    void withoutAllCollisionsIsTrueInEmptyWorld() {
        final Predicate<Vector2f> predicate =
                PositionPredicateFactory.withoutAllCollisions(new Vector2i(10, 10));

        assertTrue(predicate.test(new Vector2f(0, 0)));
        assertTrue(predicate.test(new Vector2f(500, 500)));
    }

    @Test
    void withoutAllCollisionsIsFalseWhenBodyCoversSpot() {
        addBody(CollisionType.ROOM_OBJECT, 0, 0, 20);

        final Predicate<Vector2f> predicate =
                PositionPredicateFactory.withoutAllCollisions(new Vector2i(10, 10));

        assertFalse(predicate.test(new Vector2f(0, 0)));
    }

    @Test
    void withoutAllCollisionsIsTrueAwayFromBody() {
        addBody(CollisionType.ROOM_OBJECT, 0, 0, 20);

        final Predicate<Vector2f> predicate =
                PositionPredicateFactory.withoutAllCollisions(new Vector2i(10, 10));

        assertTrue(predicate.test(new Vector2f(1000, 1000)));
    }

    @Test
    void withoutAllCollisionsAppliesOffset() {
        addBody(CollisionType.ROOM_OBJECT, 0, 0, 20);

        // Offset shifts the query box right by 1000, away from the body.
        final Predicate<Vector2f> withOffset =
                PositionPredicateFactory.withoutAllCollisions(new Vector2i(10, 10), new Vector2i(1000, 1000));

        assertTrue(withOffset.test(new Vector2f(0, 0)));

        // No offset -> query box overlaps the body at origin.
        final Predicate<Vector2f> noOffset =
                PositionPredicateFactory.withoutAllCollisions(new Vector2i(10, 10), new Vector2i(0, 0));

        assertFalse(noOffset.test(new Vector2f(0, 0)));
    }

    // ---- withoutRoomObjectCollisions -----------------------------------------

    @Test
    void withoutRoomObjectCollisionsTrueWhenNoOverlap() {
        final RoomObject candidate = new FakeRoomObject(0, 0, 10, 10);
        final RoomObject existing = new FakeRoomObject(500, 500, 10, 10);
        final Room room = new FakeRoom(List.of(existing));

        final Predicate<Vector2f> predicate =
                PositionPredicateFactory.withoutRoomObjectCollisions(candidate, room);

        assertTrue(predicate.test(new Vector2f(0, 0)));
    }

    @Test
    void withoutRoomObjectCollisionsFalseWhenOverlap() {
        final RoomObject candidate = new FakeRoomObject(0, 0, 20, 20);
        final RoomObject existing = new FakeRoomObject(5, 5, 20, 20);
        final Room room = new FakeRoom(List.of(existing));

        final Predicate<Vector2f> predicate =
                PositionPredicateFactory.withoutRoomObjectCollisions(candidate, room);

        assertFalse(predicate.test(new Vector2f(0, 0)));
    }

    @Test
    void withoutRoomObjectCollisionsIgnoresTheCandidateItself() {
        // The only room object is the candidate -> equals() skip -> no collision.
        final RoomObject candidate = new FakeRoomObject(0, 0, 20, 20);
        final Room room = new FakeRoom(List.of(candidate));

        final Predicate<Vector2f> predicate =
                PositionPredicateFactory.withoutRoomObjectCollisions(candidate, room);

        assertTrue(predicate.test(new Vector2f(0, 0)));
    }

    // ---- stubs ----------------------------------------------------------------

    /** Minimal RoomObject; only geometry methods are exercised by the predicate. */
    private static final class FakeRoomObject implements RoomObject {
        private final Vector2f position;
        private final int width;
        private final int height;

        FakeRoomObject(final float x, final float y, final int width, final int height) {
            this.position = new Vector2f(x, y);
            this.width = width;
            this.height = height;
        }

        @Override
        public void update(final double dt) {
        }

        @Override
        public boolean canRemove() {
            return false;
        }

        @Override
        public int getWidth() {
            return width;
        }

        @Override
        public int getHeight() {
            return height;
        }

        @Override
        public Vector2f getPosition() {
            return position;
        }

        @Override
        public RenderData renderData() {
            return null;
        }

        @Override
        public RenderScene renderScene() {
            return null;
        }

        @Override
        public int getZIndex() {
            return 0;
        }

        @Override
        public boolean canRender() {
            return false;
        }
    }

    /** Minimal Room exposing only the room-object list used by the predicate. */
    private static final class FakeRoom implements Room {
        private final List<RoomObject> roomObjects;

        FakeRoom(final List<RoomObject> roomObjects) {
            this.roomObjects = roomObjects;
        }

        @Override
        public void enterEvent(final Player p) {
        }

        @Override
        public void leaveEvent(final Player p) {
        }

        @Override
        public void init() {
        }

        @Override
        public void cleanUp() {
        }

        @Override
        public void update(final double dt) {
        }

        @Override
        public void fixedUpdate(final double dt) {
        }

        @Override
        public List<Renderable> toRender() {
            return List.of();
        }

        @Override
        public int getWidth() {
            return 0;
        }

        @Override
        public int getHeight() {
            return 0;
        }

        @Override
        public int getX() {
            return 0;
        }

        @Override
        public int getY() {
            return 0;
        }

        @Override
        public void addNeighbour(final RoomDirection direction, final Room room) {
        }

        @Override
        public Map<RoomDirection, Room> getNeighboursMap() {
            return Map.of();
        }

        @Override
        public RoomType getRoomType() {
            return null;
        }

        @Override
        public List<RoomObject> getRoomObjects() {
            return roomObjects;
        }
    }
}
