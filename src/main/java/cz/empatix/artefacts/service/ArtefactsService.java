package cz.empatix.artefacts.service;

import cz.empatix.artefacts.domain.Artefact;
import cz.empatix.artefacts.domain.RingOfDeath;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import org.lwjgl.glfw.GLFW;

import java.util.List;

public class ArtefactsService {
    private final List<Artefact> allArtefacts;
    private Artefact currentArtefact;

    public ArtefactsService(final Player player) {
        this.allArtefacts = List.of(
                new RingOfDeath(player)
        );
        currentArtefact = null;
    }

    public void update(final double dt) {
        allArtefacts.forEach(artefact -> artefact.update(dt));
    }

    public void fixedUodate(final double dt) {
        allArtefacts.forEach(artefact -> artefact.fixedUpdate(dt));
    }

    public void keyPressed(final int keyCode) {
        if (GLFW.GLFW_KEY_F == keyCode) {
            currentArtefact = allArtefacts.get(0);
            currentArtefact.activate();
        }
    }

    public List<Renderable> toRender() {
        return allArtefacts.stream()
                .flatMap(artefact -> artefact.getRenderables().stream())
                .toList();
    }
}
