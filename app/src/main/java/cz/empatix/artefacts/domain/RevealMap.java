package cz.empatix.artefacts.domain;

import cz.empatix.artefacts.service.ArtefactsService;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.minimap.service.MinimapService;
import cz.empatix.particles.service.GazewindService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import org.joml.Vector2f;

import java.util.List;

public class RevealMap implements Artefact{
    private static final String TEXTURE_PATH = "ingame/artefacts/reveal_map.tga";
    private final static int ARTEFACT_SCALE = 2;
    private final CommonImage artefactImage;
    private final MinimapService minimapService;
    public RevealMap(final MinimapService minimapService) {
        artefactImage = new CommonImage(TEXTURE_PATH, new Vector2f(), ARTEFACT_SCALE);
        artefactImage.setPosition(ArtefactsService.HUD_SLOT_CENTER_X - artefactImage.getWidth() / 2f, ArtefactsService.HUD_SLOT_CENTER_Y - artefactImage.getHeight() / 2f);
        this.minimapService = minimapService;
    }
    @Override
    public void activate() {
        minimapService.visitAll();
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

    @Override
    public String getTexturePath() {
        return TEXTURE_PATH;
    }
}
