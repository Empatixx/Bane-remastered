package cz.empatix.guns.domain;

import cz.empatix.audio.init.ALInit;
import cz.empatix.audio.service.AudioService;
import cz.empatix.guns.ammo.Bullet;
import cz.empatix.guns.factory.BulletFactory;
import cz.empatix.testsupport.FakeAssetSource;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.anyFloat;
import static org.mockito.ArgumentMatchers.anyInt;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/**
 * The real {@link Pistol} (with its CommonImage/CommonText/PistolClipHud sub-objects) constructed and
 * exercised headlessly by INJECTING a {@link FakeAssetSource} — no static asset mocking. This is the
 * render-data-injection pattern: a GL-free asset source replaces the GPU pipeline entirely.
 */
class PistolFakeAssetsTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false; // mock(Bullet.class) triggers Bullet's OpenAL-touching static init
    }

    private static Pistol newPistol(final BulletFactory bulletFactory) {
        return new Pistol(bulletFactory, new FakeAssetSource(), mock(AudioService.class));
    }

    @Test
    void realPistolConstructsAndFiresThroughTheTemplate() {
        final BulletFactory bulletFactory = mock(BulletFactory.class);
        when(bulletFactory.createBullet(anyInt(), anyInt(), anyFloat(), anyFloat(), anyFloat(), anyFloat(), anyFloat()))
                .thenReturn(mock(Bullet.class));

        final Pistol pistol = newPistol(bulletFactory);
        final int clipBefore = pistol.currentClip;

        pistol.setShooting(true);
        final boolean fired = pistol.shoot(0, 0, 1, 1);

        assertTrue(fired);
        assertEquals(clipBefore - 1, pistol.currentClip);
        assertFalse(pistol.shooting); // pistol is semi-auto: trigger clears after one shot
        verify(bulletFactory).createBullet(anyInt(), anyInt(), anyFloat(), anyFloat(), anyFloat(), anyFloat(), anyFloat());
    }

    @Test
    void realPistolEmptyClipClicksWithoutFiring() {
        final BulletFactory bulletFactory = mock(BulletFactory.class);
        final Pistol pistol = newPistol(bulletFactory);
        pistol.currentClip = 0;
        pistol.ammo = 0;

        pistol.setShooting(true);
        final boolean fired = pistol.shoot(0, 0, 1, 1);

        assertFalse(fired);
        verify(bulletFactory, never())
                .createBullet(anyInt(), anyInt(), anyFloat(), anyFloat(), anyFloat(), anyFloat(), anyFloat());
    }

    @Test
    void realPistolReloadCompletesAfterReloadTime() {
        final Pistol pistol = newPistol(mock(BulletFactory.class));
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
