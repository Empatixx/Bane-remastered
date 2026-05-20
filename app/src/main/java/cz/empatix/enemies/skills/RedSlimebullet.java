package cz.empatix.enemies.skills;

import cz.empatix.render.attributes.Drawable;
import org.joml.Vector3f;

public class RedSlimebullet extends BaseSlimebullet implements Drawable {
    private final static String TEXTURE = "ingame/red_slimebullet.tga";
    public RedSlimebullet() {
        super(TEXTURE, new Vector3f(255, 215, 0));
    }
}
