package cz.empatix.guns.factory;

import cz.empatix.death.service.StatisticsService;
import cz.empatix.guns.ammo.Bullet;
import cz.empatix.guns.service.GunDamageEventHook;

import java.util.concurrent.ThreadLocalRandom;

public class BulletFactory {
    private final StatisticsService statisticsService;
    private final GunDamageEventHook gunDamageEventHook;
    public BulletFactory(final StatisticsService statisticsService,
                         final GunDamageEventHook gunDamageEventHook) {
        this.statisticsService = statisticsService;
        this.gunDamageEventHook = gunDamageEventHook;
    }
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    public Bullet createBullet(final int minDamage, final int maxDamage, final float inaccuracy,
                               final float shooterX, final float shooterY, final float targetX, final float targetY) {
        final int damage = minDamage + (int) (RANDOM.nextFloat() * (maxDamage - minDamage));
        final int modifiedDamage = gunDamageEventHook.apply(damage);
        final float inaccuracyBullet = inaccuracy * (RANDOM.nextFloat() - 0.5f);
        final Bullet bullet = new Bullet(modifiedDamage);
        bullet.setPosition(shooterX, shooterY);
        bullet.setDirection(shooterX, shooterY, targetX, targetY, inaccuracyBullet);

        statisticsService.addBulletFired();
        return bullet;
    }
}
