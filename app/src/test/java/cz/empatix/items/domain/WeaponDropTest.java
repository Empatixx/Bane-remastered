package cz.empatix.items.domain;

import cz.empatix.guns.domain.Gun;
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

/** Drives the real {@link WeaponDrop} pickup logic headlessly (E-key gated). */
@ExtendWith(HeadlessAssets.class)
class WeaponDropTest {

    @SuppressWarnings("unchecked")
    @Test
    void pickupOnlyHappensOnEKeyAndAddsTheGun() {
        final Gun gun = mock(Gun.class);
        when(gun.getTexturePath()).thenReturn("guns/pistol.tga");
        final Consumer<Gun> addGun = mock(Consumer.class);

        final WeaponDrop drop = new WeaponDrop(DropSource.PLAYER, gun, addGun);

        assertSame(DropSource.PLAYER, drop.getSource());
        // 16x16 fake entry from MockedAssets.entry()
        assertEquals(16, drop.getWidth());
        assertEquals(16, drop.getHeight());

        // Non-E interaction does nothing.
        drop.interact(Set.of(GLFW.GLFW_KEY_W));
        assertFalse(drop.isPickedUp());
        verify(addGun, never()).accept(gun);

        // E interaction picks up.
        assertTrue(drop.canInteract(Set.of(GLFW.GLFW_KEY_E)));
        drop.interact(Set.of(GLFW.GLFW_KEY_E));
        assertTrue(drop.isPickedUp());
        assertTrue(drop.canRemove());
        verify(addGun).accept(eq(gun));
    }

    @SuppressWarnings("unchecked")
    @Test
    void canInteractIsFalseWithoutE() {
        final Gun gun = mock(Gun.class);
        when(gun.getTexturePath()).thenReturn("guns/pistol.tga");

        final WeaponDrop drop = new WeaponDrop(DropSource.NORMAL, gun, g -> {});

        assertFalse(drop.canInteract(Set.of()));
    }
}
