package cz.empatix.enemies.domain.vitality;

import cz.empatix.hud.service.text.MovableAlternativeViewText;
import cz.empatix.render.domain.RenderableState;

import java.time.Instant;

public record VitalityChange(
        MovableAlternativeViewText text,
        Instant timeWhen
) {
    public void collectSubRenderables(RenderableState state) {
        text.collectSubRenderables(state);
    }
    public void update(final double dt){
        text.update(dt);
    }
}
