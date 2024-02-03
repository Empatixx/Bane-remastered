package cz.empatix.artefacts.domain;

import cz.empatix.player.DamageEventHook;

public class ShieldDamageEventHook implements DamageEventHook {
    private boolean active = true;
    @Override
    public int onDamage(int damage) {
        active = false;
        return 0;
    }
    public boolean isActive() {
        return active;
    }

}
