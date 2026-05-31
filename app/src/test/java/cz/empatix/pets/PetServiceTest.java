package cz.empatix.pets;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mockStatic;

/**
 * Headless coverage for {@link PetService}: it owns exactly one {@link Pet}, snaps the pet to its
 * owner on construction, and surfaces the pet as the single renderable. Driven with a REAL
 * {@link Player} owner and a headless {@link PhysicsService}.
 */
class PetServiceTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new PhysicsService();
    }

    @Test
    void petIsSnappedToTheOwnerOnConstruction() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Player owner = new Player();
            owner.setPosition(640f, 480f);

            final PetService service = new PetService(owner);

            assertEquals(owner.getX(), service.getPet().getPosition().x, 1f);
            assertEquals(owner.getY(), service.getPet().getPosition().y, 1f);
        }
    }

    @Test
    void toRenderExposesExactlyThePetOwned() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Player owner = new Player();
            final PetService service = new PetService(owner);

            final List<? extends Renderable> renderables = service.toRender();

            assertEquals(1, renderables.size());
            assertSame(service.getPet(), renderables.get(0));
        }
    }

    @Test
    void updateDoesNotThrowAndKeepsTheSamePet() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Player owner = new Player();
            final PetService service = new PetService(owner);
            final Pet pet = service.getPet();

            service.update(0.016f);

            assertSame(pet, service.getPet());
        }
    }

    @Test
    void getPetReturnsANonNullPet() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Player owner = new Player();
            final PetService service = new PetService(owner);

            assertTrue(service.getPet() != null);
        }
    }
}
