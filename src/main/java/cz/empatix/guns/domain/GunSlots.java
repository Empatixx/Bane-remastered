package cz.empatix.guns.domain;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.Collections;

public class GunSlots {
    private final Logger LOGGER = LoggerFactory.getLogger(GunSlots.class);
    private final static int DEFAULT_MAX_SLOTS = 2;
    private final ArrayList<Gun> slots;
    private final int maxSlots;
    private int currentSlot;

    public GunSlots() {
        slots = new ArrayList<>(Collections.nCopies(DEFAULT_MAX_SLOTS, null));
        maxSlots = DEFAULT_MAX_SLOTS;
        currentSlot = 0;
    }

    public Gun getCurrentGun() throws IllegalStateException {

        if (slots.get(currentSlot) == null) {
            LOGGER.warn("No gun in slot {}", currentSlot);
            throw new IllegalStateException("No gun in slot " + currentSlot);
        }
        return slots.get(currentSlot);
    }

    public void addGun(final Gun gun) {
        if (slots.get(currentSlot) == null){
            slots.set(currentSlot, gun);
            return;
        }
        for(int i = 0; i < maxSlots; i++){
            if(slots.get(i) == null){
                slots.set(i, gun);
                return;
            }
        }
    }

    public boolean hasFullSlots() {
        return !slots.contains(null);
    }

    public void switchSlot(final int slot) {
        LOGGER.info("Switching slot to {}", slot);
        if (slot < maxSlots && slot >= 0) {
            currentSlot = slot;
        }
    }

    public void switchNextSlot() {
        currentSlot = (currentSlot + 1) % maxSlots;
    }

    public void switchPreviousSlot() {
        if (currentSlot == 0) {
            currentSlot = maxSlots - 1;
        } else {
            currentSlot = (currentSlot - 1) % maxSlots;
        }
    }

    public Gun dropCurrentGun() throws IllegalStateException {
        if (slots.get(currentSlot) == null) {
            LOGGER.warn("No gun in slot {}", currentSlot);
            throw new IllegalStateException("No gun in slot " + currentSlot);
        }
        final Gun remove = slots.get(currentSlot);
        slots.set(currentSlot, null);
        return remove;
    }
    public boolean hasNotFullAmmo(final GunType type){
        return slots
                .stream()
                .anyMatch(gun -> gun != null && gun.getType() == type && !gun.hasFullAmmo());
    }

    public void addAmmo(final GunType gunType,final float percent) {
        slots
                .stream()
                .filter(gun -> gun != null && gun.getType() == gunType)
                .forEach(gun -> gun.addPercentAmmo(percent));
    }
    public boolean containsGun(final Gun gun){
        return slots.contains(gun);
    }
}