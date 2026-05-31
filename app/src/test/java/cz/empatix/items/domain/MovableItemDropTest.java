package cz.empatix.items.domain;

import cz.empatix.audio.init.ALInit;
import cz.empatix.render.domain.RenderScene;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.util.Set;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/** Wraps a mocked {@link ItemDrop} in a real {@link MovableItemDrop} backed by a headless physics body. */
class MovableItemDropTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new cz.empatix.newphysics.service.PhysicsService();
    }

    @Test
    void delegatesQueriesToWrappedDropAndSyncsPositionOnUpdate() {
        final ItemDrop inner = mock(ItemDrop.class);
        when(inner.getWidth()).thenReturn(20);
        when(inner.getHeight()).thenReturn(20);
        when(inner.getZIndex()).thenReturn(3);
        when(inner.canRemove()).thenReturn(true);
        when(inner.isPickedUp()).thenReturn(true);
        when(inner.renderScene()).thenReturn(RenderScene.SCENE);
        when(inner.getSource()).thenReturn(DropSource.PLAYER);

        final MovableItemDrop drop = new MovableItemDrop(inner);

        assertEquals(20, drop.getWidth());
        assertEquals(20, drop.getHeight());
        assertEquals(3, drop.getZIndex());
        assertTrue(drop.canRemove());
        assertTrue(drop.isPickedUp());
        assertSame(RenderScene.SCENE, drop.renderScene());
        assertSame(DropSource.PLAYER, drop.getSource());

        drop.setPosition(5, 7);
        drop.update(0.016f);
        // update() pushes the physics translation back onto the wrapped drop.
        verify(inner).update(0.016f);
        verify(inner).setPosition(5f, 7f);

        drop.interact(Set.of());
        verify(inner).interact(Set.of());
    }
}
