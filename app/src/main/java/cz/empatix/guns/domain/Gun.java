package cz.empatix.guns.domain;

import cz.empatix.render.attributes.Renderable;
import org.joml.Vector2f;
import org.joml.Vector3i;

import java.util.List;

public interface Gun {
    Vector3i AMMO_COUNTER_COLOR = new Vector3i(226, 153, 117);
    Vector2f AMMO_COUNTER_POSITION = new Vector2f(900, 515);

    void setShooting(final boolean shooting);

    boolean isReloading();

    boolean hasFullAmmo();

    boolean shoot(final float shooterX, final float shooterY, final float targetX, final float targetY);

    void reload();

    void update(double dt);

    List<Renderable> toRender();

    List<Renderable> hudToRender();

    GunType getType();
    String getTexturePath();

    void addPercentAmmo(final float percent);
    void setMaxAmmo(final int maxAmmo);
    int getMaxAmmo();

}
