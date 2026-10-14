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

import java.util.ArrayList;
import java.util.List;

public class Uzi extends AbstractGun implements Gun{
    private final static String WEAPON_TXT_PATH = "ingame/guns/uzi.tga";
    private final CommonImage gunHud;
    private final CommonText ammoHud;
    private final List<Bullet> bullets;
    private final BulletFactory bulletFactory;
    private final SubMachineClipHud clipHud;

    public Uzi(final BulletFactory bulletFactory, final AudioService audioService) {
        super(audioService);
        this.minDamage = 1;
        this.maxDamage = 2;

        this.inaccuracy = 0.4f;
        this.fireRateTime = 100;
        this.reloadTime = 700;

        this.clipSize = 20;
        this.currentClip = clipSize;
        this.maxAmmo = 200;
        this.ammo = maxAmmo;

        this.gunType = GunType.PISTOL;
        this.shooting = false;

        this.bullets = new ArrayList<>(clipSize);
        final TextureAtlas.Entry texture = AssetsService.getTexture(WEAPON_TXT_PATH);
        this.gunHud = new CommonImage(texture, new Vector2f(GunService.HUD_SLOT_CENTER_X - texture.width() * GunService.GUN_TEXTURE_SCALE / 2f, GunService.HUD_SLOT_CENTER_Y - texture.height() * GunService.GUN_TEXTURE_SCALE / 2f), GunService.GUN_TEXTURE_SCALE);
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
    protected void spawnBullets(final float shooterX, final float shooterY, final float targetX, final float targetY) {
        bullets.add(bulletFactory.createBullet(minDamage, maxDamage, inaccuracy, shooterX, shooterY, targetX, targetY));
    }

    @Override
    protected boolean isSemiAuto() {
        return false;
    }

    @Override
    protected SoundEffect shootSound() {
        return SoundEffect.UZI_SHOOT;
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
        final RenderableState state = new RenderableState(32);
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
