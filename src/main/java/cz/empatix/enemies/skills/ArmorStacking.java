package cz.empatix.enemies.skills;

import cz.empatix.render.attributes.Renderable;

import java.util.List;

public class ArmorStacking implements Skill {
    private static final int COOLDOWN_MS = 4000;
    private static final int ARMOR_STACKS = 3;
    private final Timer armorStackTimer;
    private int currentArmorStacks;
    private boolean wasActivated = false;
    public ArmorStacking(){
        this.currentArmorStacks = 0;
        armorStackTimer = new Timer(COOLDOWN_MS);
    }
    public void addArmorStack(){
        if(currentArmorStacks < ARMOR_STACKS){
            currentArmorStacks++;
        }
        if (currentArmorStacks == ARMOR_STACKS){
            currentArmorStacks = 0;
        }
    }

    @Override
    public void activate() {
        wasActivated = false;
        if(armorStackTimer.isFinished()){
            armorStackTimer.start();
            addArmorStack();
            wasActivated = true;
        }
    }

    public boolean wasActivated(){
        return wasActivated;
    }

    @Override
    public void update(float dt) {
    }

    @Override
    public List<Renderable> getSubRenderables() {
        return null;
    }
}
