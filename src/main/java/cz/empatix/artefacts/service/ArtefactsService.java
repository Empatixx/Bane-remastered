package cz.empatix.artefacts.service;

import cz.empatix.artefacts.domain.Artefact;
import cz.empatix.artefacts.domain.RingOfDeath;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;

import java.util.ArrayList;
import java.util.List;

public class ArtefactsService {
    private final List<Artefact> allArtefacts;
    private Artefact currentArtefact;
    private final CommonImage commonImage;
    private boolean shouldActivate = false;
    public ArtefactsService(final Player player) {
        this.allArtefacts = List.of(
                new RingOfDeath(player)
        );
        currentArtefact = allArtefacts.get(0);
        commonImage = new CommonImage("ingame/artefacthud.tga",new Vector2f(),1.5f);
        commonImage.setPosition(615, 450);
    }

    public void update(final double dt) {
        allArtefacts.forEach(artefact -> artefact.update(dt));

    }

    public void fixedUpdate(final double dt) {
        if (shouldActivate) {
            currentArtefact.activate();
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
        renderables.add(commonImage);
        renderables.addAll(allArtefacts.stream()
                .flatMap(artefact -> artefact.getRenderables().stream())
                .toList());
        return renderables;
    }
}
