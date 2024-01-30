package cz.empatix.artefacts.domain;

import cz.empatix.artefacts.domain.Artefact;
import cz.empatix.artefacts.service.ArtefactsService;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import org.joml.Vector2f;

import java.util.List;

public class ArmorRefill implements Artefact {
    private final static int ARTEFACT_SCALE = 2;
    private final CommonImage artefactImage;
    private final Player player;

    public ArmorRefill(final Player player) {
        this.player = player;
        artefactImage = new CommonImage("ingame/artefacts/armorrefill.tga", new Vector2f(), ARTEFACT_SCALE);
        artefactImage.setPosition(ArtefactsService.HUD_SLOT_CENTER_X - artefactImage.getWidth() / 2f, ArtefactsService.HUD_SLOT_CENTER_Y - artefactImage.getHeight() / 2f);
    }

    @Override
    public void activate() {
        player.setArmor(player.getMaxArmor());
    }

    @Override
    public void update(double dt) {
    }

    @Override
    public void fixedUpdate(double dt) {

    }

    @Override
    public List<Renderable> getRenderables() {
        return List.of();
    }

    @Override
    public List<Renderable> hudToRender() {
        return List.of(artefactImage);
    }
}
