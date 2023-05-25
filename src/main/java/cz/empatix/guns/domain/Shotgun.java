package cz.empatix.guns.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.guns.ammo.Bullet;
import cz.empatix.guns.service.GunService;
import cz.empatix.hud.service.Image;
import cz.empatix.hud.service.Text;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;
import java.util.stream.Stream;

public class Shotgun extends AbstractGun implements Gun {
    private final static int BULLETS_PER_SHOT = 4;
    private final static Logger LOGGER = LoggerFactory.getLogger(Shotgun.class);
    private final Image gunHud;
    private final Text ammoHud;
    private final List<Bullet> bullets;

    public Shotgun() {
        this.minDamage = 2;
        this.maxDamage = 3;

        this.inaccuracy = 0.15f;
        this.fireRateTime = 500;
        this.reloadTime = 700;

        this.clipSize = 6;
        this.currentClip = clipSize;
        this.maxAmmo = 48;
        this.ammo = maxAmmo;

        this.gunType = GunType.SHOTGUN;
        this.shooting = false;

        this.bullets = new ArrayList<>(clipSize);
        final TextureAtlas.Entry texture = AssetsService.getTexture("ingame/guns/shotgun.tga");
        this.gunHud = new Image(texture, new Vector2f(GunService.HUD_SLOT_CENTER_X - texture.width() / 2f, GunService.HUD_SLOT_CENTER_Y - texture.height() / 2f), 1);
        this.ammoHud = new Text("", "PixelFJVerdana12pt.ttf",new Vector2f(AMMO_COUNTER_POSITION), 10);
        this.ammoHud.setColorRGB(AMMO_COUNTER_COLOR);

    }

    @Override
    public void setShooting(final boolean shooting) {
        this.shooting = shooting;
    }

    @Override
    public boolean shoot(float shooterX, float shooterY, float targetX, float targetY
    ) {
        if(!shooting){
            return false;
        }
        if(System.currentTimeMillis() - lastShootTime < fireRateTime){
            return false;
        }
        if (currentClip > 0) {
            LOGGER.info("Shooting from pistol clip: {}, ammo: {}", currentClip, ammo);
            currentClip--;
            for(int i = 0;i<BULLETS_PER_SHOT;i++){
                final float inaccuracy = this.inaccuracy * (float) Math.pow(-1, i) * (i + 1) / BULLETS_PER_SHOT;
                final int damage = minDamage + (int) (ThreadLocalRandom.current().nextFloat() * (maxDamage - minDamage));
                final Bullet bullet = new Bullet();
                bullet.setPosition(shooterX, shooterY);
                bullet.setDirection(shooterX, shooterY, targetX, targetY, inaccuracy);
                bullets.add(bullet);
            }
            lastShootTime = System.currentTimeMillis();
            shooting = false;
            return true;
        }
        return false;
    }

    @Override
    public void reload() {
        if(reloading){
            return;
        }
        if (ammo > 0 && currentClip < clipSize) {
            LOGGER.info("Reloading clip: {}, ammo: {}", currentClip, ammo);
            reloading = true;
            lastReloadTime = System.currentTimeMillis();
        }
    }

    @Override
    public void update(final double dt) {
        bullets.forEach(b -> b.update(dt));
        if(reloading){
            if(System.currentTimeMillis() - lastReloadTime > reloadTime){
                reloading = false;
                reloadAmmo();
            }
        }
    }

    @Override
    public List<Renderable> toRender() {
        return List.copyOf(bullets);
    }

    @Override
    public List<Renderable> hudToRender() {
        ammoHud.setText(String.format("%d/%d", currentClip, ammo));
        ammoHud.setPosition(AMMO_COUNTER_POSITION.x() - ammoHud.getWidth(), AMMO_COUNTER_POSITION.y());
        return Stream.of(List.of(gunHud), ammoHud.getSubRenderables())
                .flatMap(Collection::stream)
                .collect(java.util.stream.Collectors.toList());
    }

}
