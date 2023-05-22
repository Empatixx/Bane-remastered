package cz.empatix.guns.domain;

import cz.empatix.render.attributes.Renderable;

import java.util.List;

public interface Gun {
    void setShooting(final boolean shooting);

    boolean shoot(final float shooterX, final float shooterY, final float targetX, final float targetY);

    void reload();

    void update(double dt);

    List<Renderable> toRender();

    String getAmmoCounter();
}
