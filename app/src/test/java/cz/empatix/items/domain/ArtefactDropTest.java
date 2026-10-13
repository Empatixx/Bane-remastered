package cz.empatix.items.domain;

import cz.empatix.artefacts.domain.Artefact;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.lwjgl.glfw.GLFW;

import java.util.Set;
import java.util.function.Consumer;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.eq;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/** Drives the real {@link ArtefactDrop} pickup logic headlessly (E-key gated, 2x scale). */
@ExtendWith(HeadlessAssets.class)
class ArtefactDropTest {

    @SuppressWarnings("unchecked")
    @Test
    void pickupOnEKeyAddsArtefactAndAppliesScale() {
        final Artefact artefact = mock(Artefact.class);
        when(artefact.getTexturePath()).thenReturn("artefacts/shield.tga");
        final Consumer<Artefact> addArtefact = mock(Consumer.class);

        final ArtefactDrop drop = new ArtefactDrop(DropSource.NORMAL, artefact, addArtefact);

        assertSame(DropSource.NORMAL, drop.getSource());
        // 16x16 fake entry, SCALE = 2
        assertEquals(32, drop.getWidth());
        assertEquals(32, drop.getHeight());

        drop.interact(Set.of(GLFW.GLFW_KEY_F));
        assertFalse(drop.isPickedUp());
        verify(addArtefact, never()).accept(artefact);

        assertTrue(drop.canInteract(Set.of(GLFW.GLFW_KEY_E)));
        drop.interact(Set.of(GLFW.GLFW_KEY_E));
        assertTrue(drop.isPickedUp());
        assertTrue(drop.canRemove());
        verify(addArtefact).accept(eq(artefact));
    }
}
