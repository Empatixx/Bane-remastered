package cz.empatix.artefacts.service;

import cz.empatix.artefacts.domain.Artefact;
import cz.empatix.items.domain.MovableItemDrop;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;
import cz.empatix.minimap.service.MinimapService;
import cz.empatix.player.Player;
import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.lwjgl.glfw.GLFW;

import java.lang.reflect.Field;

import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.ArgumentMatchers.anyFloat;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/**
 * Drives the real {@link ArtefactsService} headlessly: it builds the real artefact roster (each
 * fetches GPU textures) and a {@code CommonImage} HUD via the stubbed
 * {@link cz.empatix.assets.service.AssetsService} from {@link HeadlessAssets}. The static initializer
 * that fetches {@code ingame/artefacthud.tga} also runs under the active mockStatic. Render output
 * (HUD font/Entry math) is not exercised.
 */
@ExtendWith(HeadlessAssets.class)
class ArtefactsServiceTest {

    private Player player;
    private ItemService itemService;
    private MinimapService minimapService;

    private ArtefactsService newService() {
        player = mock(Player.class);
        itemService = mock(ItemService.class);
        minimapService = mock(MinimapService.class);
        return new ArtefactsService(player, itemService, minimapService);
    }

    private static Artefact currentArtefact(final ArtefactsService service) throws Exception {
        final Field field = ArtefactsService.class.getDeclaredField("currentArtefact");
        field.setAccessible(true);
        return (Artefact) field.get(service);
    }

    private static void setShouldActivate(final ArtefactsService service, final boolean value) throws Exception {
        final Field field = ArtefactsService.class.getDeclaredField("shouldActivate");
        field.setAccessible(true);
        field.set(service, value);
    }

    private static boolean shouldActivate(final ArtefactsService service) throws Exception {
        final Field field = ArtefactsService.class.getDeclaredField("shouldActivate");
        field.setAccessible(true);
        return (boolean) field.get(service);
    }

    @Test
    void startsWithTheRevealMapArtefactSelected() throws Exception {
        final ArtefactsService service = newService();
        assertNotNull(currentArtefact(service));
    }

    @Test
    void getUnusedRandomArtefactNeverReturnsTheCurrentlyHeldOne() throws Exception {
        final ArtefactsService service = newService();
        final Artefact current = currentArtefact(service);

        for (int i = 0; i < 30; i++) {
            final Artefact unused = service.getUnusedRandomArtefact();
            assertNotNull(unused);
            org.junit.jupiter.api.Assertions.assertNotSame(current, unused);
        }
    }

    @Test
    void keyPressedFFlagsActivationRequest() throws Exception {
        final ArtefactsService service = newService();
        setShouldActivate(service, false);

        service.keyPressed(GLFW.GLFW_KEY_F);

        org.junit.jupiter.api.Assertions.assertTrue(shouldActivate(service));
    }

    @Test
    void otherKeysDoNotFlagActivation() throws Exception {
        final ArtefactsService service = newService();
        setShouldActivate(service, false);

        service.keyPressed(GLFW.GLFW_KEY_G);

        org.junit.jupiter.api.Assertions.assertFalse(shouldActivate(service));
    }

    @Test
    void fixedUpdateActivatesAndConsumesCurrentArtefact() throws Exception {
        final ArtefactsService service = newService();
        final Artefact current = currentArtefact(service);
        setShouldActivate(service, true);

        service.fixedUpdate(0.016);

        // current artefact got activated and cleared; flag reset
        assertNull(currentArtefact(service));
        org.junit.jupiter.api.Assertions.assertFalse(shouldActivate(service));
        // the held artefact is a real RevealMap; activation runs without GL
    }

    @Test
    void addArtefactReplacesCurrentAndDropsTheOldOne() throws Exception {
        final ArtefactsService service = newService();

        // wire the drop path: dropCurrentArtefact() -> itemFactory.createMovableArtefactDrop(...)
        final ItemFactory itemFactory = mock(ItemFactory.class);
        when(itemService.getItemFactory()).thenReturn(itemFactory);
        final MovableItemDrop drop = mock(MovableItemDrop.class);
        when(itemFactory.createMovableArtefactDrop(any(), any())).thenReturn(drop);
        when(player.getPosition()).thenReturn(new Vector2f(100, 100));
        when(player.getWidth()).thenReturn(32);
        when(player.getHeight()).thenReturn(32);

        final Artefact incoming = mock(Artefact.class);
        service.addArtefact(incoming);

        // old artefact dropped into the world, new one held
        verify(itemFactory).createMovableArtefactDrop(any(), any());
        verify(drop).setPosition(anyFloat(), anyFloat());
        verify(itemService).add(drop);
        assertSame(incoming, currentArtefact(service));
    }

    @Test
    void addArtefactWithNoCurrentSimplyHoldsItWithoutDropping() throws Exception {
        final ArtefactsService service = newService();
        // clear the current artefact so addArtefact takes the no-drop branch
        final Field field = ArtefactsService.class.getDeclaredField("currentArtefact");
        field.setAccessible(true);
        field.set(service, null);

        final Artefact incoming = mock(Artefact.class);
        service.addArtefact(incoming);

        assertSame(incoming, currentArtefact(service));
        verify(itemService, org.mockito.Mockito.never()).add(any());
    }
}
