package cz.empatix.enemies.domain.vitality;

import cz.empatix.hud.service.text.AlternativeViewText;
import cz.empatix.hud.service.text.CommonText;
import cz.empatix.hud.service.text.MovableAlternativeViewText;
import cz.empatix.render.attributes.Renderable;
import org.joml.Vector2f;

import java.time.Instant;
import java.util.List;

public record VitalityChange(
        MovableAlternativeViewText text,
        Instant timeWhen
) {
    public List<Renderable> toRender(){
        return List.of(text);
    }
    public void update(final double dt){
        text.update(dt);
    }
}
