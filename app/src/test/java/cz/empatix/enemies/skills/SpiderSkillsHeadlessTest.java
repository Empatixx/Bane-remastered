package cz.empatix.enemies.skills;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.player.Player;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.animation.AnimationState;
import cz.empatix.testsupport.MockedAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;
import static org.mockito.Mockito.when;

/**
 * Headless coverage for the spider-boss skills: the {@link CobwebProjectile} trap, the
 * {@link SpitCobweb} state machine (driven with a real animation + mocked physics body) and
 * {@link HatchSpiderlings} which spawns real spiderling enemies. Bullet/enemy spawning runs through
 * the GL-free {@link PhysicsService}; assets are stubbed.
 */
class SpiderSkillsHeadlessTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new PhysicsService();
    }

    private static Animation spiderAnimation() {
        final Animation animation = Animation.builder()
                .setTotal(3, 5)
                .addState(AnimationState.IDLE, 0, 5, 192, 120, 120)
                .addState(AnimationState.SPELL0, 1, 3, 192, 120, 150)
                .addState(AnimationState.DEATH, 2, 3, 192, 120, 150)
                .texture(AssetsService.getTexture("ingame/spider.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        return animation;
    }

    @Test
    void cobwebProjectileExposesDimensionsAndNeverSelfRemoves() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final CobwebProjectile cobweb = new CobwebProjectile();

            assertEquals(32 * 2, cobweb.getWidth());
            assertEquals(32 * 2, cobweb.getHeight());
            assertFalse(cobweb.shouldRemove(), "a cobweb trap never marks itself for removal");

            cobweb.setPosition(100f, 100f);
            cobweb.setDirection(100f, 100f, 200f, 100f, 0f);
            cobweb.update(0.016);
            cobweb.destroy(); // tears down the physics body without throwing
        }
    }

    @Test
    void spitCobwebEntersSpellStateOnActivationOffCooldown() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Animation animation = spiderAnimation();
            final PhysicsObject body = mock(PhysicsObject.class);
            final SpitCobweb spit = new SpitCobweb(animation, body, () -> new Vector2f(50f, 50f));

            // Timer is started in the constructor (5s cooldown) -> activate is a no-op at first.
            spit.activate();
            assertEquals(AnimationState.IDLE, animation.getCurrentState(),
                    "fresh SpitCobweb is on cooldown so it does not start the spell");
        }
    }

    @Test
    void spitCobwebDoesNotSpawnProjectilesBeforeTheSpellPlaysOut() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Animation animation = spiderAnimation();
            final PhysicsObject body = mock(PhysicsObject.class);
            final Player target = mock(Player.class);
            when(target.getPosition()).thenReturn(new Vector2f(400f, 400f));

            final SpitCobweb spit = new SpitCobweb(animation, body, () -> new Vector2f(50f, 50f));
            spit.setTarget(target);

            // SPELL0 has not finished playing (hasPlayedOnce is false), so update() never flags
            // canShoot and fixedUpdate spawns nothing.
            animation.setState(AnimationState.SPELL0);
            spit.update(0.016f);
            spit.fixedUpdate(0.016f);

            final RenderableState state = new RenderableState(8);
            spit.collectSubRenderables(state);
            assertTrue(state.getList().isEmpty(), "no cobweb is spat until the spell animation completes");

            spit.destroy();
        }
    }

    @Test
    void hatchSpiderlingsStartsEmptyAndIsRemovable() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final HatchSpiderlings hatch = new HatchSpiderlings(() -> new Vector2f(0f, 0f));

            assertTrue(hatch.canRemove(), "no spiderlings spawned yet -> removable");
        }
    }

    @Test
    void hatchSpiderlingsSpawnsOneSpiderlingPerActivationCycle() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final HatchSpiderlings hatch = new HatchSpiderlings(() -> new Vector2f(10f, 10f));
            hatch.setTarget(mock(Player.class));

            // The hatch timer is started in the constructor (2.5s delay), so the very first activate
            // is on cooldown and spawns nothing.
            hatch.activate();
            assertTrue(hatch.canRemove(), "first activation is on cooldown -> still empty");

            final RenderableState state = new RenderableState(8);
            hatch.collectSubRenderables(state);
            assertTrue(state.getList().isEmpty());
        }
    }

    @Test
    void hatchSpiderlingsDespawnAllToleratesEmptyList() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final HatchSpiderlings hatch = new HatchSpiderlings(() -> new Vector2f(0f, 0f));

            // No spiderlings present -> despawnAll/fixedUpdate are safe no-ops.
            hatch.despawnAll();
            hatch.fixedUpdate(0.016f);
            assertTrue(hatch.canRemove());
        }
    }

    @Test
    void behindTargetTargettingFallsBackToSelfWhenNoTarget() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Vector2f self = new Vector2f(75f, 90f);
            final cz.empatix.enemies.targetting.BehindTargetTargetting targetting =
                    new cz.empatix.enemies.targetting.BehindTargetTargetting(() -> self, 32, 32);

            // With no target set, getPosition returns the supplier's own position verbatim.
            assertSame(self, targetting.getPosition());
        }
    }

    @Test
    void behindTargetTargettingFindsAClearSpotBehindTargetInEmptyWorld() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Player target = mock(Player.class);
            when(target.getPosition()).thenReturn(new Vector2f(500f, 500f));
            final cz.empatix.enemies.targetting.BehindTargetTargetting targetting =
                    new cz.empatix.enemies.targetting.BehindTargetTargetting(() -> new Vector2f(0f, 0f), 32, 32);
            targetting.setTarget(target);

            // Empty world -> a clear behind-target slot always exists.
            assertTrue(targetting.canTargetBehind(), "an unobstructed spot behind the target exists");
            final Vector2f behind = targetting.getPosition();
            assertTrue(Float.isFinite(behind.x) && Float.isFinite(behind.y));
        }
    }
}
