package cz.empatix.guns.factory;

import cz.empatix.common.RngService;
import cz.empatix.death.service.StatisticsService;
import cz.empatix.guns.ammo.Bullet;
import cz.empatix.guns.service.GunDamageEventHook;

public class BulletFactory {
    private final StatisticsService statisticsService;
    private final GunDamageEventHook gunDamageEventHook;
    private final RngService rng;
    public BulletFactory(final StatisticsService statisticsService,
                         final GunDamageEventHook gunDamageEventHook,
                         final RngService rng) {
        this.statisticsService = statisticsService;
        this.gunDamageEventHook = gunDamageEventHook;
        this.rng = rng;
    }
    public Bullet createBullet(final int minDamage, final int maxDamage, final float inaccuracy,
                               final float shooterX, final float shooterY, final float targetX, final float targetY) {
        return createBullet(minDamage, maxDamage, inaccuracy, rng.nextFloat() - 0.5f, shooterX, shooterY, targetX, targetY);
    }
    public Bullet createBullet(final int minDamage, final int maxDamage, final float inaccuracy, final float inaccuracyPercent,
                               final float shooterX, final float shooterY, final float targetX, final float targetY) {
        final int damage = minDamage + (int) (rng.nextFloat() * (maxDamage - minDamage));
        final int modifiedDamage = gunDamageEventHook.apply(damage);
        final float inaccuracyBullet = inaccuracy * inaccuracyPercent;
        final Bullet bullet = new Bullet(modifiedDamage);
        bullet.setPosition(shooterX, shooterY);
        bullet.setDirection(shooterX, shooterY, targetX, targetY, inaccuracyBullet);

        statisticsService.addBulletFired();
        return bullet;
    }
}
