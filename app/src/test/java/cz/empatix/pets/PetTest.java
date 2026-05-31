package cz.empatix.pets;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.collision.DamageSourceType;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.testsupport.MockedAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import java.lang.reflect.Field;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mockStatic;

/**
 * Headless domain-logic coverage for the real {@link Pet}: the damage-source contract (it is a
 * PLAYER-typed {@code DamageSource} dealing 1 damage), the post-attack cooldown gate (forced via the
 * {@code lastAttack} field), the gaze-wind feet anchor math, and the {@code ShadowCaster} geometry
 * accessors. Pathfinding-heavy {@code fixedUpdate} is intentionally not driven here. Physics bodies
 * are built against a headless {@link PhysicsService}; audio is disabled.
 */
class PetTest {

    private static final int DAMAGE = 1;
    private static final int WIDTH = 32 * 3;
    private static final int HEIGHT = 32 * 3;

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new PhysicsService();
    }

    private static void setLastAttack(final Pet pet, final long value) throws Exception {
        final Field field = Pet.class.getDeclaredField("lastAttack");
        field.setAccessible(true);
        field.setLong(pet, value);
    }

    @Test
    void freshPetDealsItsBaseDamage() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Pet pet = new Pet();

            assertEquals(DAMAGE, pet.damage(), "a pet that has not just attacked deals full damage");
        }
    }

    @Test
    void damageIsOnCooldownImmediatelyAfterAnAttack() throws Exception {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Pet pet = new Pet();

            pet.postDamage(DAMAGE); // stamps lastAttack = now
            setLastAttack(pet, System.currentTimeMillis()); // make the window unambiguously "just now"

            assertEquals(0, pet.damage(), "within the cooldown window the pet deals no damage");
        }
    }

    @Test
    void damageIsAvailableAgainAfterTheCooldownElapses() throws Exception {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Pet pet = new Pet();
            pet.postDamage(DAMAGE);

            // force the 200ms cooldown to have long since passed
            setLastAttack(pet, System.currentTimeMillis() - 10_000);

            assertEquals(DAMAGE, pet.damage());
        }
    }

    @Test
    void petIsAPlayerAlignedDamageSource() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Pet pet = new Pet();

            assertEquals(DamageSourceType.PLAYER, pet.type(), "the pet's hits count as friendly/player damage");
        }
    }

    @Test
    void positionSettersAndGettersRoundTrip() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Pet pet = new Pet();

            pet.setPosition(250f, 175f);

            assertEquals(250f, pet.getPosition().x, 0.001f);
            assertEquals(175f, pet.getPosition().y, 0.001f);
        }
    }

    @Test
    void shadowGeometryMatchesSpriteDimensions() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Pet pet = new Pet();

            assertEquals(WIDTH, pet.getWidth());
            assertEquals(HEIGHT, pet.getHeight());
            assertEquals(WIDTH, pet.getShadowWidth());
            assertEquals(HEIGHT, pet.getShadowHeight());
            // feet offset lifts the shadow anchor up by 30% of the sprite height
            assertEquals(-HEIGHT * 0.3f, pet.getShadowFeetOffsetY(), 0.001f);
        }
    }

    @Test
    void shadowPositionTracksThePhysicsBody() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Pet pet = new Pet();
            pet.setPosition(400f, 500f);

            final Vector2f shadowPos = pet.getShadowPosition();

            // render translation is the interpolated body position; right after a hard set it equals it
            assertEquals(400f, shadowPos.x, 1f);
            assertEquals(500f, shadowPos.y, 1f);
        }
    }

    @Test
    void gazeWindPositionIsAnchoredAtTheBottomCentreOfTheSprite() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Pet pet = new Pet();
            pet.setPosition(100f, 100f);

            final Vector2f gaze = pet.getGazeWindPosition();

            // x is centred (+WIDTH/2), y is at the feet (+HEIGHT)
            assertEquals(100f + WIDTH / 2f, gaze.x, 1f);
            assertEquals(100f + HEIGHT, gaze.y, 1f);
        }
    }

    @Test
    void zIndexAndRemovalContract() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Pet pet = new Pet();

            assertEquals(3, pet.getZIndex());
            assertFalse(pet.canRemove(), "a pet is never auto-removed");
        }
    }

    @Test
    void interactKeysAreEmptyByDefault() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Pet pet = new Pet();

            assertTrue(pet.getInteractKeys().isEmpty());
        }
    }

    @Test
    void petExposesItsPhysicsObject() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Pet pet = new Pet();

            assertTrue(pet.getPhysicsObject() != null);
        }
    }
}
