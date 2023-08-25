package cz.empatix.guns.factory;

import cz.empatix.guns.ammo.Bullet;

import java.util.concurrent.ThreadLocalRandom;

public class BulletFactory {
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    public static Bullet createBullet(final int minDamage, final int maxDamage, final float inaccuracy,
                               final float shooterX, final float shooterY, final float targetX, final float targetY) {
        final int damage = minDamage + (int) (RANDOM.nextFloat() * (maxDamage - minDamage));
        final float inaccuracyBullet = inaccuracy * (RANDOM.nextFloat() - 0.5f);
        final Bullet bullet = new Bullet(damage);
        bullet.setPosition(shooterX, shooterY);
        bullet.setDirection(shooterX, shooterY, targetX, targetY, inaccuracyBullet);
        return bullet;
    }
}
