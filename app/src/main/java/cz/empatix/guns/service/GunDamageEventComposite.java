package cz.empatix.guns.service;

import java.util.ArrayList;
import java.util.List;

public class GunDamageEventComposite implements GunDamageEventHook{
    private final List<GunDamageEventHook> hooks = new ArrayList<>();

    @Override
    public int apply(int damage) {
        for (GunDamageEventHook hook : hooks) {
            damage = hook.apply(damage);
        }
        return damage;
    }
    public void addHook(GunDamageEventHook hook) {
        hooks.add(hook);
    }
    public void removeHook(GunDamageEventHook hook) {
        hooks.remove(hook);
    }
}
