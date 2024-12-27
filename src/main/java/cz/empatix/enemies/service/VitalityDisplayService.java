package cz.empatix.enemies.service;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.enemies.domain.vitality.VitalityChange;
import cz.empatix.hud.service.text.AlternativeViewText;
import cz.empatix.hud.service.text.CommonText;
import cz.empatix.hud.service.text.MovableAlternativeViewText;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.joml.Vector3i;

import java.time.Instant;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public class VitalityDisplayService {
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final static Vector3i COLOR_RED = new Vector3i(255, 0, 0);
    private final static Vector3i COLOR_GREEN = new Vector3i(0, 255, 0);
    private final static long TTL_MILIS = 500L;

    private List<VitalityChange> changes = new ArrayList<>();
    public void addChange(final int change, final Vector2f position, final int width, final int height){
        if (change == 0) {
            return;
        }
        if (change > 0){
            final AlternativeViewText text = new AlternativeViewText("+" + change, "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_22);
            text.setPosition(position.x() + RANDOM.nextFloat(width), position.y() + height/5f);
            text.setColorRGB(COLOR_GREEN);

            final MovableAlternativeViewText movableText = new MovableAlternativeViewText(text, new Vector2f(0, -30f));

            final Instant now = Instant.now();
            changes.add(new VitalityChange(movableText, now));
        } else {
            final AlternativeViewText text = new AlternativeViewText(String.valueOf(Math.abs(change)), "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_22);
            text.setPosition(position.x() + RANDOM.nextFloat(width), position.y() + height/5f);
            text.setColorRGB(COLOR_RED);

            final MovableAlternativeViewText movableText = new MovableAlternativeViewText(text, new Vector2f(0, -30f));

            final Instant now = Instant.now();
            changes.add(new VitalityChange(movableText, now));
        }
    }
    public void update(final double dt){
        changes.forEach(c -> c.update(dt));
        changes.removeIf(e -> e.timeWhen().plusMillis(TTL_MILIS).isBefore(Instant.now()));
    }
    public List<Renderable> getRenderables(){
        final List<Renderable> renderables = new ArrayList<>();
        for (final VitalityChange change : changes) {
            renderables.addAll(change.toRender());
        }
        return renderables;
    }
    public boolean isEmpty(){
        return changes.isEmpty();
    }

}
