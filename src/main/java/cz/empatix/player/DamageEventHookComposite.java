package cz.empatix.player;

import java.util.ArrayList;
import java.util.List;

public class DamageEventHookComposite implements DamageEventHook{
    private final List<DamageEventHook> hooks = new ArrayList<>();
    @Override
    public int onDamage(int damage) {
        for (DamageEventHook hook : hooks) {
            damage = hook.onDamage(damage);
        }
        return damage;
    }

    public void addHook(final DamageEventHook hook) {
        hooks.add(hook);
    }

    public void removeHook(DamageEventHook hook) {
        hooks.remove(hook);
    }
}
