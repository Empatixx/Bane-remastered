package cz.empatix.artefacts.service;

import cz.empatix.artefacts.domain.ArmorRefill;
import cz.empatix.artefacts.domain.Artefact;
import cz.empatix.artefacts.domain.RingOfDeath;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.common.utils.PositionUtils;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;

import java.util.ArrayList;
import java.util.List;

public class ArtefactsService {
    private final static float HUD_SLOT_SCALE = 1.5f;
    public final static int HUD_SLOT_CENTER_X;
    public final static int HUD_SLOT_CENTER_Y;
    private final static Vector2f HUD_SLOT_POSITION = new Vector2f(615, 450);
    private final List<Artefact> allArtefacts;
    private Artefact currentArtefact;
    private final CommonImage commonImage;
    private boolean shouldActivate = false;

    public ArtefactsService(final Player player) {
        this.allArtefacts = List.of(
                new RingOfDeath(player),
                new ArmorRefill(player)
        );
        currentArtefact = allArtefacts.get(1);
        commonImage = new CommonImage("ingame/artefacthud.tga", new Vector2f(), HUD_SLOT_SCALE);
        commonImage.setPosition(HUD_SLOT_POSITION.x(), HUD_SLOT_POSITION.y());
    }

    static {
        final TextureAtlas.Entry hudSlotTexture = AssetsService.getTexture("ingame/artefacthud.tga");

        HUD_SLOT_CENTER_X = PositionUtils.getCenterXInRange((int) (HUD_SLOT_POSITION.x()), (int) (HUD_SLOT_POSITION.x() + hudSlotTexture.width() * HUD_SLOT_SCALE));
        HUD_SLOT_CENTER_Y = PositionUtils.getCenterYInRange((int) (HUD_SLOT_POSITION.y()), (int) (HUD_SLOT_POSITION.y() + hudSlotTexture.height() * HUD_SLOT_SCALE));
    }

    public void update(final double dt) {
        allArtefacts.forEach(artefact -> artefact.update(dt));

    }

    public void fixedUpdate(final double dt) {
        if (shouldActivate) {
            if (currentArtefact != null) {
                currentArtefact.activate();
                currentArtefact = null;
            }
            shouldActivate = false;
        }
        allArtefacts.forEach(artefact -> artefact.fixedUpdate(dt));
    }

    public void keyPressed(final int keyCode) {
        if (keyCode == GLFW.GLFW_KEY_F) {
            shouldActivate = true;
        }
    }

    public List<Renderable> toRender() {
        final ArrayList<Renderable> renderables = new ArrayList<>(allArtefacts.size() + 1);
        if (currentArtefact != null) {
            renderables.add(commonImage);
            renderables.addAll(currentArtefact.hudToRender());

        }
        renderables.addAll(allArtefacts.stream()
                .flatMap(artefact -> artefact.getRenderables().stream())
                .toList());
        return renderables;
    }
}
