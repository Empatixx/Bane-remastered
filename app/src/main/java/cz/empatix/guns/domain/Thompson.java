package cz.empatix.guns.domain;

import cz.empatix.audio.service.AudioService;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.sound.SoundEffect;
import cz.empatix.guns.ammo.Bullet;
import cz.empatix.guns.factory.BulletFactory;
import cz.empatix.guns.service.GunService;
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

public class Thompson extends AbstractGun implements Gun{
    private final static String WEAPON_TXT_PATH = "ingame/guns/thompson.tga";
    private final static Logger LOGGER = LoggerFactory.getLogger(Thompson.class);
    private final CommonImage gunHud;
    private final CommonText ammoHud;
    private final List<Bullet> bullets;
    private final BulletFactory bulletFactory;
    private final SubMachineClipHud clipHud;
    public Thompson(final BulletFactory bulletFactory, final AudioService audioService) {
        super(audioService);
        this.minDamage = 1;
        this.maxDamage = 3;

        this.inaccuracy = 0.05f;
        this.fireRateTime = 200;
        this.reloadTime = 850;

        this.clipSize = 30;
        this.currentClip = clipSize;
        this.maxAmmo = 300;
        this.ammo = maxAmmo;

        this.gunType = GunType.PISTOL;
        this.shooting = false;

        this.bullets = new ArrayList<>(clipSize);
        final TextureAtlas.Entry texture = AssetsService.getTexture(WEAPON_TXT_PATH);
        this.gunHud = new CommonImage(texture, new Vector2f(GunService.HUD_SLOT_CENTER_X - texture.width() / 2f, GunService.HUD_SLOT_CENTER_Y - texture.height() / 2f), 1);
        this.ammoHud = new CommonText("", "PixelFJVerdana12pt.ttf", new Vector2f(AMMO_COUNTER_POSITION), 10);
        this.ammoHud.setColorRGB(AMMO_COUNTER_COLOR);

        this.bulletFactory = bulletFactory;

        this.clipHud = new SubMachineClipHud(() -> currentClip, () -> clipSize);
    }

    @Override
    public void setShooting(final boolean shooting) {
        this.shooting = shooting;
    }

    @Override
    public boolean shoot(float shooterX, float shooterY, float targetX, float targetY
    ) {
        if(!shooting || reloading){
            return false;
        }
        if(System.currentTimeMillis() - lastShootTime < fireRateTime){
            return false;
        }
        if (currentClip > 0) {
            LOGGER.info("Shooting from thompson clip: {}, ammo: {}", currentClip, ammo);
            final Bullet bullet = bulletFactory.createBullet(minDamage, maxDamage, inaccuracy, shooterX, shooterY, targetX, targetY);
            bullets.add(bullet);
            currentClip--;
            lastShootTime = System.currentTimeMillis();
            audioService.play(SoundEffect.PISTOL_SHOOT);
            return true;
        } else {
            audioService.play(SoundEffect.EMPTY_GUN_SHOOT);
            lastShootTime = System.currentTimeMillis();
            return false;
        }
    }

    @Override
    protected SoundEffect reloadSound() {
        return SoundEffect.PISTOL_RELOAD;
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
