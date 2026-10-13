package cz.empatix.particles.service;

import cz.empatix.particles.domain.GazewindParticle;
import cz.empatix.render.domain.RenderableState;

import java.util.ArrayList;
import java.util.List;

public class GazewindService {
    private final static String DEFAULT_GAZE_WIND_PARTICLE = "ingame/gazewind_particle.tga";
    private final static int MAX_GAZE_WIND_PARTICLES = 4;
    private final static long GAZE_WIND_COOLDOWN = 200;
    private long lastGazeWindTime = 0;
    private final List<GazewindParticle> gazewindParticles = new ArrayList<>(MAX_GAZE_WIND_PARTICLES);
    private final String gazewindTexture;
    public GazewindService(final String gazewindTexture) {
        this.gazewindTexture = gazewindTexture;
    }
    public GazewindService() {
        this(DEFAULT_GAZE_WIND_PARTICLE);
    }
    public void update(final float dt){
        gazewindParticles.removeIf(GazewindParticle::shouldRemove);
        gazewindParticles.forEach(p -> p.update(dt));
    }
    public void spawn(final float x, final float y){
        if (System.currentTimeMillis() - lastGazeWindTime > GAZE_WIND_COOLDOWN) {
            lastGazeWindTime = System.currentTimeMillis();
            final GazewindParticle particle = new GazewindParticle(gazewindTexture);
            particle.setPosition(x - particle.getWidth()/2f, y - particle.getHeight()/2f);
            gazewindParticles.add(particle);
        }
    }
    public void clear(){
        gazewindParticles.clear();
    }
    public void collectSubRenderables(RenderableState state) {
        state.addAll(gazewindParticles);
    }

}
