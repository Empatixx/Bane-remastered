package cz.empatix.guns.domain;

import cz.empatix.assets.domain.font.Font;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.service.AudioService;
import cz.empatix.guns.ammo.Bullet;
import cz.empatix.guns.factory.BulletFactory;
import cz.empatix.textureatlas.domain.TextureAtlas;
import cz.empatix.textureatlas.domain.TextureAtlasInfo;
import org.joml.Vector2i;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.ArgumentMatchers.anyFloat;
import static org.mockito.ArgumentMatchers.anyInt;
import static org.mockito.ArgumentMatchers.anyString;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/**
 * Proves the Mockito static-asset stubbing approach: the <em>real</em> {@link Pistol} (and its
 * CommonImage/CommonText/PistolClipHud sub-objects, all of which fetch GPU textures/fonts in their
 * constructors) is instantiated and exercised headlessly by stubbing the static asset accessors.
 * A fake (GL-free) {@link TextureAtlas.Entry} is returned so the pixel-math in the HUD ctors works.
 */
class PistolMockedAssetsTest {

    /** A GL-free atlas entry: 16x16 sprite inside a 64x64 atlas, so width()/normalized math return real values. */
    private static TextureAtlas.Entry fakeEntry() {
        return new TextureAtlas.Entry(1, new Vector2i(64, 64), new TextureAtlasInfo(0, 0, 16, 16));
    }

    private static Pistol newPistol(final BulletFactory bulletFactory, final MockedStatic<AssetsService> assets) {
        assets.when(() -> AssetsService.getTexture(anyString())).thenReturn(fakeEntry());
        assets.when(() -> AssetsService.getFont(anyString(), any())).thenReturn(mock(Font.class));
        return new Pistol(bulletFactory, mock(AudioService.class));
    }

    @Test
    void realPistolConstructsAndFiresThroughTheTemplate() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            final BulletFactory bulletFactory = mock(BulletFactory.class);
            when(bulletFactory.createBullet(anyInt(), anyInt(), anyFloat(), anyFloat(), anyFloat(), anyFloat(), anyFloat()))
                    .thenReturn(mock(Bullet.class));

            final Pistol pistol = newPistol(bulletFactory, assets);
            final int clipBefore = pistol.currentClip;

            pistol.setShooting(true);
            final boolean fired = pistol.shoot(0, 0, 1, 1);

            assertTrue(fired);
            assertEquals(clipBefore - 1, pistol.currentClip);
            assertFalse(pistol.shooting); // pistol is semi-auto: trigger clears after one shot
            verify(bulletFactory).createBullet(anyInt(), anyInt(), anyFloat(), anyFloat(), anyFloat(), anyFloat(), anyFloat());
        }
    }

    @Test
    void realPistolEmptyClipClicksWithoutFiring() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            final BulletFactory bulletFactory = mock(BulletFactory.class);
            final Pistol pistol = newPistol(bulletFactory, assets);
            pistol.currentClip = 0;
            pistol.ammo = 0;

            pistol.setShooting(true);
            final boolean fired = pistol.shoot(0, 0, 1, 1);

            assertFalse(fired);
            verify(bulletFactory, never())
                    .createBullet(anyInt(), anyInt(), anyFloat(), anyFloat(), anyFloat(), anyFloat(), anyFloat());
        }
    }

    @Test
    void realPistolReloadCompletesAfterReloadTime() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            final Pistol pistol = newPistol(mock(BulletFactory.class), assets);
            pistol.currentClip = 2;
            pistol.ammo = 50;

            pistol.reload();
            assertTrue(pistol.reloading);

            pistol.lastReloadTime = System.currentTimeMillis() - (pistol.reloadTime + 1000);
            pistol.update(0.016); // drives updateReload() on the real class

            assertFalse(pistol.reloading);
            assertEquals(7, pistol.currentClip); // topped back up to clip size
        }
    }
}
