package cz.empatix.enemies.skills;

import cz.empatix.render.attributes.Drawable;
import org.joml.Vector3f;

public class PurpleSlimebullet extends BaseSlimebullet implements Drawable {
    private final static String TEXTURE = "ingame/purple_slimebullet.tga";
    public PurpleSlimebullet() {
        super(TEXTURE, new Vector3f(255, 0, 255));
    }
}
