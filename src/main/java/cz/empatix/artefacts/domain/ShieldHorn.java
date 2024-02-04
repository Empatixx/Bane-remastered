package cz.empatix.artefacts.domain;

import cz.empatix.artefacts.service.ArtefactsService;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.player.DamageEventHook;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import org.joml.Vector2f;

import java.util.List;

public class ShieldHorn implements Artefact {
    private final static String TEXTURE_PATH = "ingame/artefacts/shieldhorn.tga";
    private final static int ARTEFACT_SCALE = 2;
    private final CommonImage artefactImage;
    private final Player player;
    private final MagicShield magicShield;
    private ShieldDamageEventHook shieldDamageEventHook;

    public ShieldHorn(final Player player) {
        this.player = player;
        artefactImage = new CommonImage(TEXTURE_PATH, new Vector2f(), ARTEFACT_SCALE);
        magicShield = new MagicShield();
        artefactImage.setPosition(ArtefactsService.HUD_SLOT_CENTER_X - artefactImage.getWidth() / 2f, ArtefactsService.HUD_SLOT_CENTER_Y - artefactImage.getHeight() / 2f);
    }

    @Override
    public void activate() {
        shieldDamageEventHook = new ShieldDamageEventHook();
        player.addDamageHook(shieldDamageEventHook);
    }

    @Override
    public void update(double dt) {
        magicShield.setPosition(player.getRenderPosition().x() + player.getWidth() / 2f - magicShield.getWidth() / 2f,
                player.getRenderPosition().y() + player.getHeight() / 2f - magicShield.getHeight() / 2f);
        magicShield.update(dt);
    }

    @Override
    public void fixedUpdate(double dt) {
        if (shieldDamageEventHook != null && !shieldDamageEventHook.isActive()) {
            player.removeDamageHook(shieldDamageEventHook);
            shieldDamageEventHook = null;
        }
    }

    @Override
    public List<Renderable> getRenderables() {
        if (shieldDamageEventHook != null && shieldDamageEventHook.isActive()) {
            return List.of(magicShield);
        }
        return List.of();
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

