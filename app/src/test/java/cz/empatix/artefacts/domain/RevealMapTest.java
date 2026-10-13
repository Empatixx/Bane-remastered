package cz.empatix.artefacts.domain;

import cz.empatix.minimap.service.MinimapService;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.verify;

/** Covers the reveal-all effect of {@link RevealMap}. */
@ExtendWith(HeadlessAssets.class)
class RevealMapTest {

    @Test
    void activateRevealsTheWholeMinimap() {
        final MinimapService minimapService = mock(MinimapService.class);
        final RevealMap artefact = new RevealMap(minimapService);

        artefact.activate();

        verify(minimapService).visitAll();
    }

    @Test
    void renderablesEmptyHudHasIconAndPathExposed() {
        final RevealMap artefact = new RevealMap(mock(MinimapService.class));

        assertTrue(artefact.getRenderables().isEmpty());
        assertEquals(1, artefact.hudToRender().size());
        assertEquals("ingame/artefacts/reveal_map.tga", artefact.getTexturePath());
        // update / fixedUpdate are no-ops and must not throw.
        artefact.update(0.5);
        artefact.fixedUpdate(0.5);
    }
}
