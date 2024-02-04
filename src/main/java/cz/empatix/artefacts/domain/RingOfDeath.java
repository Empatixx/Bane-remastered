package cz.empatix.artefacts.domain;

import cz.empatix.artefacts.service.ArtefactsService;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.guns.ammo.Bullet;
import cz.empatix.guns.service.GunService;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.List;

public class RingOfDeath implements Artefact {
    private final static String TEXTURE_PATH = "ingame/artefacts/ringofdeath.tga";
    private final static int ARTEFACT_SCALE = 3;
    private final CommonImage artefactImage;
    private final List<Bullet> bullets = new ArrayList<>(30);
    private final Player player;

    public RingOfDeath(Player player) {
        this.player = player;
        final TextureAtlas.Entry texture = AssetsService.getTexture(TEXTURE_PATH);
        artefactImage = new CommonImage(texture, new Vector2f(), ARTEFACT_SCALE);
        artefactImage.setPosition(ArtefactsService.HUD_SLOT_CENTER_X - texture.width() *ARTEFACT_SCALE / 2f, ArtefactsService.HUD_SLOT_CENTER_Y - texture.height() * ARTEFACT_SCALE / 2f);
    }

    @Override
    public void activate() {
        for (int i = 0; i < 50; i++) {
            final Bullet bullet = new Bullet(2);
            final float inaccuracy = 0.15f * i;
            bullet.setPosition(player.getPosition().x(), player.getPosition().y());
            bullet.setDirection(player.getPosition().x(), player.getPosition().y(), 0, 0, inaccuracy);
            bullets.add(bullet);
        }
    }

    @Override
    public void update(double dt) {
        bullets.forEach(bullet -> bullet.update(dt));
        bullets.removeIf(Bullet::shouldRemove);
    }

    @Override
    public void fixedUpdate(double dt) {

    }

    @Override
    public List<Renderable> getRenderables() {
        return List.copyOf(bullets);
    }

    @Override
    public List<Renderable> hudToRender() {
        return List.of(artefactImage);
    }

    @Override
    public String getTexturePath() {
        return TEXTURE_PATH;
    }
}
