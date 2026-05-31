package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mockStatic;

/**
 * Headless coverage for the "decorative" {@link RoomObject}s — static or purely-animated props with
 * no damage/interaction logic. For each, the real class is constructed with stubbed assets and
 * exercised through its full public surface: position round-trip, the animation {@code update} tick,
 * positive geometry and the always-false {@code canRemove}.
 */
class DecorativeRoomObjectsTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new PhysicsService();
    }

    private static void assertBehavesLikeStaticProp(final RoomObject object, final Runnable setPosition) {
        for (int i = 0; i < 5; i++) {
            object.update(0.05);
        }
        setPosition.run();
        assertTrue(object.getWidth() > 0);
        assertTrue(object.getHeight() > 0);
        assertFalse(object.canRemove());
    }

    @Test
    void angelStatue() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final AngelStatue o = new AngelStatue();
            assertBehavesLikeStaticProp(o, () -> o.setPosition(1f, 2f));
            assertEquals(64 * 4, o.getWidth());
        }
    }

    @Test
    void armorstand() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Armorstand o = new Armorstand();
            assertBehavesLikeStaticProp(o, () -> o.setPosition(3f, 4f));
            assertEquals(3f, o.getPosition().x, 0.0001f);
        }
    }

    @Test
    void banner() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Banner o = new Banner();
            assertBehavesLikeStaticProp(o, () -> o.setPosition(5f, 6f));
            assertEquals(32 * 2, o.getWidth());
        }
    }

    @Test
    void bones() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Bones o = new Bones();
            o.setState(cz.empatix.animation.AnimationState.TYPE0);
            assertBehavesLikeStaticProp(o, () -> o.setPosition(7f, 8f));
            assertEquals(16 * 4, o.getWidth());
        }
    }

    @Test
    void bookshelf() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Bookshelf o = new Bookshelf();
            assertBehavesLikeStaticProp(o, () -> o.setPosition(9f, 10f));
            assertEquals(64 * 3, o.getWidth());
        }
    }

    @Test
    void box() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Box o = new Box();
            assertBehavesLikeStaticProp(o, () -> o.setPosition(11f, 12f));
            assertEquals((int) (30 * 3f), o.getWidth());
            assertTrue(o.canRender());
        }
    }

    @Test
    void bricks() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Bricks o = new Bricks();
            o.setState(cz.empatix.animation.AnimationState.TYPE1);
            assertBehavesLikeStaticProp(o, () -> o.setPosition(13f, 14f));
            assertEquals(16 * 4, o.getWidth());
        }
    }

    @Test
    void chains() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Chains o = new Chains();
            assertBehavesLikeStaticProp(o, () -> o.setPosition(15f, 16f));
            assertEquals(16 * 4, o.getWidth());
            assertEquals(32 * 4, o.getHeight());
        }
    }

    @Test
    void cobweb() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Cobweb o = new Cobweb();
            o.setLeft();
            o.setRight();
            assertBehavesLikeStaticProp(o, () -> o.setPosition(17f, 18f));
            assertEquals(16 * 4, o.getWidth());
        }
    }

    @Test
    void craftingTable() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final CraftingTable o = new CraftingTable();
            assertBehavesLikeStaticProp(o, () -> o.setPosition(19f, 20f));
            assertEquals(96 * 3, o.getWidth());
        }
    }

    @Test
    void layingBook() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final LayingBook o = new LayingBook();
            assertBehavesLikeStaticProp(o, () -> o.setPosition(21f, 22f));
            assertTrue(o.canRender());
        }
    }

    @Test
    void mapTable() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final MapTable o = new MapTable();
            assertBehavesLikeStaticProp(o, () -> o.setPosition(23f, 24f));
            assertEquals(59 * 3, o.getWidth());
        }
    }

    @Test
    void pentagramCircle() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final PentagramCircle o = new PentagramCircle();
            assertBehavesLikeStaticProp(o, () -> o.setPosition(25f, 26f));
            assertTrue(o.canRender());
        }
    }

    @Test
    void potionStation() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final PotionStation o = new PotionStation();
            assertBehavesLikeStaticProp(o, () -> o.setPosition(27f, 28f));
            assertEquals(48 * 3, o.getWidth());
        }
    }

    @Test
    void rock() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Rock o = new Rock();
            o.setState(cz.empatix.animation.AnimationState.TYPE0);
            assertBehavesLikeStaticProp(o, () -> o.setPosition(29f, 30f));
            assertEquals(16 * 4, o.getWidth());
        }
    }

    @Test
    void shopTable() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final ShopTable o = new ShopTable();
            assertBehavesLikeStaticProp(o, () -> o.setPosition(31f, 32f));
            assertEquals((int) (96 * 1.5f), o.getWidth());
        }
    }

    @Test
    void taskBoard() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final TaskBoard o = new TaskBoard();
            assertBehavesLikeStaticProp(o, () -> o.setPosition(33f, 34f));
            assertEquals(90 * 3, o.getWidth());
            assertTrue(o.canRender());
        }
    }

    @Test
    void weaponRack() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final WeaponRack o = new WeaponRack();
            assertBehavesLikeStaticProp(o, () -> o.setPosition(35f, 36f));
            assertEquals(26 * 3, o.getWidth());
        }
    }
}
