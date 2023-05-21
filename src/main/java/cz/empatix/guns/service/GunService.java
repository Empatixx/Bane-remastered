package cz.empatix.guns.service;

import cz.empatix.guns.domain.Gun;
import cz.empatix.guns.domain.GunSlots;
import cz.empatix.guns.domain.Pistol;
import cz.empatix.render.attributes.Renderable;

import java.util.List;

public class GunService {
    private final List<Gun> guns;
    private final GunSlots slots;

    public GunService() {
        guns = List.of(
                new Pistol()
        );
        slots = new GunSlots();
        slots.addGun(guns.get(0));
    }

    public void update(final double dt) {
        guns.forEach(gun -> gun.update(dt));
    }

    public void fixedUpdate(final float shooterX, final float shooterY,
                            final float targetX, final float targetY, final double dt) {
        slots.getCurrentGun().shoot(shooterX, shooterY, targetX, targetY);
    }

    public void setShooting(final boolean shooting) {
        slots.getCurrentGun().setShooting(shooting);
    }

    public void switchNextSlot() {
        slots.switchNextSlot();
    }

    public void switchPreviousSlot() {
        slots.switchPreviousSlot();
    }

    public void reload() {
        slots.getCurrentGun().reload();
    }

    public boolean shoot(final float shooterX, final float shooterY, final float targetX, final float targetY) {
        return slots.getCurrentGun().shoot(shooterX, shooterY, targetX, targetY);
    }

    public void addGun(final Gun gun) {
        if (slots.hasFullSlots()) {
            final Gun drop = slots.dropCurrentGun();
            slots.addGun(gun);
            return;
        }
        slots.addGun(gun);
    }

    public List<Renderable> toRender(final double dt) {
        return guns.stream()
                .flatMap(gun -> gun.toRender().stream())
                .toList();
    }
}
