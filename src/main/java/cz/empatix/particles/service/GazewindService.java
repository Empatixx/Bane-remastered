package cz.empatix.particles.service;

import cz.empatix.particles.domain.GazewindParticle;
import cz.empatix.render.attributes.Renderable;

import java.util.ArrayList;
import java.util.List;

public class GazewindService {
    private final static int MAX_GAZE_WIND_PARTICLES = 4;
    private final static long GAZE_WIND_COOLDOWN = 200;
    private long lastGazeWindTime = 0;
    private final List<GazewindParticle> gazewindParticles = new ArrayList<>(MAX_GAZE_WIND_PARTICLES);

    public void update(final float dt){
        gazewindParticles.removeIf(GazewindParticle::shouldRemove);
        gazewindParticles.forEach(p -> p.update(dt));
    }
    public void spawn(final float x, final float y){
        if (System.currentTimeMillis() - lastGazeWindTime > GAZE_WIND_COOLDOWN) {
            lastGazeWindTime = System.currentTimeMillis();
            final GazewindParticle particle = new GazewindParticle();
            particle.setPosition(x - particle.getWidth()/2f, y - particle.getHeight()/2f);
            gazewindParticles.add(particle);
        }
    }
    public List<Renderable> toRender() {
        return List.copyOf(gazewindParticles);
    }

}
