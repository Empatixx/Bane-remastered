package cz.empatix.artefacts.domain;

import cz.empatix.guns.ammo.Bullet;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;

import java.util.ArrayList;
import java.util.List;

public class RingOfDeath implements Artefact{
    private final List<Bullet> bullets = new ArrayList<>(30);
    private final Player player;
    public RingOfDeath(Player player) {
        this.player = player;
    }

    @Override
    public void activate() {
        for (int i = 0; i < 30; i++) {
            final Bullet bullet = new Bullet(2);
            final float inaccuracy = 0.1f * i;
            bullet.setPosition(player.getPosition().x(), player.getPosition().y());
            bullet.setDirection(player.getPosition().x(), player.getPosition().y(),0,0, inaccuracy);
            bullets.add(bullet);
        }
    }

    @Override
    public void update(double dt) {
        bullets.forEach(bullet -> bullet.update(dt));
        bullets.removeIf(Bullet::shouldRemove);
    }

    @Override
    public void fixedUpdate(double dt) {

    }

    @Override
    public List<Renderable> getRenderables() {
        return List.copyOf(bullets);
    }
}
