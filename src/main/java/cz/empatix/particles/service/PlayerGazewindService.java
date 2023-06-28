package cz.empatix.particles.service;

import cz.empatix.particles.domain.GazewindParticle;
import cz.empatix.render.attributes.Renderable;

import java.util.ArrayList;
import java.util.List;

public class PlayerGazewindService {
    private final static int MAX_GAZE_WIND_PARTICLES = 4;
    private final static long GAZE_WIND_COOLDOWN = 1000;
    private long lastGazeWindTime = 0;
    private final List<GazewindParticle> gazewindParticles = new ArrayList<>(MAX_GAZE_WIND_PARTICLES);

    public void updateGazeWind(final float x, final float y) {
        if (System.currentTimeMillis() - lastGazeWindTime > GAZE_WIND_COOLDOWN) {
            lastGazeWindTime = System.currentTimeMillis();
            gazewindParticles.add(new GazewindParticle(null, null));
        }
        gazewindParticles.removeIf(GazewindParticle::shouldRemove);
    }
    public List<Renderable> toRender(final double dt) {
        return List.copyOf(gazewindParticles);
    }

}
