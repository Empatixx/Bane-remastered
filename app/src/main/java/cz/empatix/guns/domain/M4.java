package cz.empatix.guns.domain;

import cz.empatix.audio.service.AudioService;
import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.service.AssetSource;
import cz.empatix.sound.SoundEffect;
import cz.empatix.guns.factory.BulletFactory;
import cz.empatix.guns.service.GunService;
import cz.empatix.guns.ammo.Bullet;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.hud.service.text.CommonText;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public class M4 extends AbstractGun implements Gun{
    private final static String WEAPON_TXT_PATH = "ingame/guns/M4.tga";
    private final static Logger LOGGER = LoggerFactory.getLogger(M4.class);
    private final static int BULLETS_PER_SHOT = 4;
    private final static int BURST_BULLET_DELAY = 50;
    private final CommonImage gunHud;
    private final CommonText ammoHud;
    private final List<Bullet> bullets;
    private int bulletsToShoot;
    private float lastTargetX;
    private float lastTargetY;
    private float lastShooterX;
    private float lastShooterY;
    private final BulletFactory bulletFactory;
    private final SubMachineClipHud clipHud;

    public M4(final BulletFactory bulletFactory, final AssetSource assetSource, final AudioService audioService) {
        super(audioService);
        this.minDamage = 2;
        this.maxDamage = 2;

        this.inaccuracy = 0.f;
        this.fireRateTime = 750;
        this.reloadTime = 1300;

        this.clipSize = 32;
        this.currentClip = clipSize;
        this.maxAmmo = 250;
        this.ammo = maxAmmo;

        this.gunType = GunType.PISTOL;
        this.shooting = false;

        this.bullets = new ArrayList<>(clipSize);
        final TextureAtlas.Entry texture = assetSource.getTexture(WEAPON_TXT_PATH);
        this.gunHud = new CommonImage(texture, new Vector2f(GunService.HUD_SLOT_CENTER_X - texture.width() / 2f, GunService.HUD_SLOT_CENTER_Y - texture.height() / 2f), 1);
        this.ammoHud = new CommonText("", assetSource.getFont("PixelFJVerdana12pt.ttf", FontSize.FONTSIZE_22), new Vector2f(AMMO_COUNTER_POSITION), 10);
        this.ammoHud.setColorRGB(AMMO_COUNTER_COLOR);
        this.bulletFactory = bulletFactory;
        this.clipHud = new SubMachineClipHud(assetSource, () -> currentClip, () -> clipSize);
    }

    @Override
    public void setShooting(final boolean shooting) {
        this.shooting = shooting;
    }

    @Override
    public boolean shoot(float shooterX, float shooterY, float targetX, float targetY
    ) {
        if (bulletsToShoot > 0) {
            if (System.currentTimeMillis() - lastShootTime > BURST_BULLET_DELAY) {
                if(currentClip <= 0){
                    bulletsToShoot = 0;
                    return false;
                }
                lastShootTime = System.currentTimeMillis();
                LOGGER.info("Shooting from M4 clip: {}, ammo: {}", currentClip, ammo);
                final int damage = minDamage + (int) (ThreadLocalRandom.current().nextFloat() * (maxDamage - minDamage));
                final Bullet bullet = new Bullet(damage);
                bullet.setPosition(shooterX, shooterY);
                bullet.setDirection(this.lastShooterX, this.lastShooterY, this.lastTargetX, this.lastTargetY, inaccuracy);
                bullets.add(bullet);
                bulletsToShoot--;
                currentClip--;
                return true;
            }
            return false;
        }
        if(!shooting || reloading){
            return false;
        }
        if(System.currentTimeMillis() - lastShootTime < fireRateTime){
            return false;
        }
        if (currentClip > 0) {
            LOGGER.info("Shooting from pistol clip: {}, ammo: {}", currentClip, ammo);
            final Bullet bullet = bulletFactory.createBullet(minDamage, maxDamage, inaccuracy, shooterX, shooterY, targetX, targetY);
            bullets.add(bullet);
            currentClip--;
            bulletsToShoot = BULLETS_PER_SHOT - 1;
            lastShootTime = System.currentTimeMillis();

            audioService.play(SoundEffect.M4_SHOOT);

            lastTargetX = targetX;
            lastTargetY = targetY;
            lastShooterX = shooterX;
            lastShooterY = shooterY;
            return true;
        } else {
            audioService.play(SoundEffect.EMPTY_GUN_SHOOT);
            lastShootTime = System.currentTimeMillis();
        }
        return false;
    }

    @Override
    protected SoundEffect shootSound() {
        return SoundEffect.M4_SHOOT;
    }

    // M4 overrides shoot() for its delayed burst, so spawnBullets() is only here to satisfy the
    // AbstractGun contract; it mirrors a single round of the burst.
    @Override
    protected void spawnBullets(final float shooterX, final float shooterY, final float targetX, final float targetY) {
        bullets.add(bulletFactory.createBullet(minDamage, maxDamage, inaccuracy, shooterX, shooterY, targetX, targetY));
    }

    @Override
    protected SoundEffect reloadSound() {
        return SoundEffect.M4_RELOAD;
    }

    @Override
    public void reload() {
        // can't reload mid-burst
        if (bulletsToShoot > 0) {
            return;
        }
        super.reload();
    }

    @Override
    public void update(final double dt) {
        clipHud.update(dt);
        bullets.forEach(b -> b.update(dt));
        bullets.removeIf(Bullet::shouldRemove);
        updateReload();
    }


    @Override
    public List<Renderable> toRender() {
        return List.copyOf(bullets);
    }

    @Override
    public List<Renderable> hudToRender() {
        ammoHud.setText(String.format("%d/%d", currentClip, ammo));
        ammoHud.setPosition(AMMO_COUNTER_POSITION.x() - ammoHud.getWidth(), AMMO_COUNTER_POSITION.y());
        final RenderableState state = new RenderableState(40);
        state.add(gunHud);
        ammoHud.collectSubRenderables(state);
        clipHud.collectSubRenderables(state);
        return state.getList();
    }
    @Override
    public boolean isReloading(){
        return reloading;
    }
    @Override
    public GunType getType() {
        return gunType;
    }

    @Override
    public String getTexturePath() {
        return WEAPON_TXT_PATH;
    }

    @Override
    public boolean hasFullAmmo() {
        return ammo == maxAmmo;
    }
    @Override
    public void addPercentAmmo(float percent) {
         ammo += maxAmmo * percent;
        if(ammo > maxAmmo){
            ammo = maxAmmo;
        }
    }
    @Override
    public void setMaxAmmo(int maxAmmo) {
        this.maxAmmo = maxAmmo;
    }

    @Override
    public int getMaxAmmo() {
        return maxAmmo;
    }
}
