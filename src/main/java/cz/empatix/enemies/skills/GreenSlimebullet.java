package cz.empatix.enemies.skills;

import cz.empatix.render.attributes.Drawable;
import org.joml.Vector3f;

public class GreenSlimebullet extends BaseSlimebullet implements Drawable {
    private final static String TEXTURE = "ingame/green_slimebullet.tga";

    public GreenSlimebullet() {
        super(TEXTURE, new Vector3f(0, 200, 0));
    }
}
