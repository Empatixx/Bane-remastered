package cz.empatix.guns.domain;

import cz.empatix.audio.service.AudioService;
import cz.empatix.guns.ammo.Bullet;
import cz.empatix.guns.factory.BulletFactory;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.anyFloat;
import static org.mockito.ArgumentMatchers.anyInt;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.times;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/** The shotgun fires a spread of pellets from a single clip round (the one shoot()-template override point). */
@ExtendWith(HeadlessAssets.class)
class ShotgunPelletsHeadlessTest {

    @Test
    void oneShotSpawnsFourPelletsButConsumesOneClipRound() {
        final BulletFactory bulletFactory = mock(BulletFactory.class);
        // Shotgun uses the 8-arg (speed-carrying) createBullet overload, once per pellet.
        when(bulletFactory.createBullet(anyInt(), anyInt(), anyFloat(), anyFloat(), anyFloat(), anyFloat(), anyFloat(), anyFloat()))
                .thenReturn(mock(Bullet.class));

        final Shotgun shotgun = new Shotgun(bulletFactory, mock(AudioService.class));
        final int clipBefore = shotgun.currentClip;

        shotgun.setShooting(true);
        final boolean fired = shotgun.shoot(0, 0, 1, 0);

        assertTrue(fired);
        assertEquals(clipBefore - 1, shotgun.currentClip, "spread costs a single clip round");
        verify(bulletFactory, times(4))
                .createBullet(anyInt(), anyInt(), anyFloat(), anyFloat(), anyFloat(), anyFloat(), anyFloat(), anyFloat());
    }
}
