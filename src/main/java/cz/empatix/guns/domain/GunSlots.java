package cz.empatix.guns.domain;

import java.util.ArrayList;
import java.util.List;

public class GunSlots {
    private final static int DEFAULT_MAX_SLOTS = 2;
    private final List<Gun> slots;
    private int maxSlots;
    private int currentSlot;

    public GunSlots() {
        slots = new ArrayList<>(DEFAULT_MAX_SLOTS);
        maxSlots = DEFAULT_MAX_SLOTS;
        currentSlot = 0;
    }

    public Gun getCurrentGun() {
        return slots.get(currentSlot);
    }

    public void addGun(final Gun gun) {
        slots.add(gun);
    }

    public boolean hasFullSlots() {
        return slots.size() == maxSlots;
    }
    public void switchSlot(final int slot){
        if(slot < maxSlots && slot >= 0){
            return;
        }
        currentSlot = slot;
    }
    public void switchNextSlot(){
        currentSlot = (currentSlot + 1) % maxSlots;
    }
    public void switchPreviousSlot(){
        currentSlot = (currentSlot - 1) % maxSlots;
    }

    public Gun dropCurrentGun() {
        final Gun remove = slots.remove(currentSlot);
        if (currentSlot >= slots.size()) {
            currentSlot = slots.size() - 1;
        }
        return remove;
    }
}