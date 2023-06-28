package cz.empatix.particles.domain;

import cz.empatix.physics.domain.transform.CommonTransform;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;

public record GazewindParticle(
        CommonTransform transform,
        Animation animation
        ) implements Drawable {

    @Override
    public RenderData renderData() {
        return null;
    }

    @Override
    public RenderScene renderScene() {
        return null;
    }

    @Override
    public int getZIndex() {
        return 0;
    }

    @Override
    public boolean canRender() {
        return false;
    }
    public boolean shouldRemove(){
        return false;
    }
}
