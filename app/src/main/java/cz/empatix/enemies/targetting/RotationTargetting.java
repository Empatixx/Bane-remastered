package cz.empatix.enemies.targetting;

import cz.empatix.player.Player;
import org.joml.Vector2f;

import java.util.function.Supplier;

public class RotationTargetting {
    private Player target;
    private final Supplier<Vector2f> positionSupplier;
    private final int width;
    private final int height;
    public RotationTargetting(final Supplier<Vector2f> positionSupplier,
                              final int width,
                              final int height) {
        this.positionSupplier = positionSupplier;
        this.width = width;
        this.height = height;
    }
    public float getRotation(){
        if (target == null) {
            return 0;
        }
        final Vector2f position = positionSupplier.get();
        final float dx = target.getX() + target.getWidth()/2f - (position.x());
        final float dy = target.getY() + target.getHeight()/2f - (position.y());
        return (float) Math.atan2(dy, dx);
    }

    public void target(final Player player) {
        this.target = player;
    }
}
