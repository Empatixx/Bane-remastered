package cz.empatix.guns.domain;

import cz.empatix.guns.ammo.PistolAmmoHud;
import cz.empatix.guns.ammo.ShotgunAmmoHud;
import cz.empatix.render.domain.RenderableState;
import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;
import java.util.function.Supplier;

public class ShotgunClipHud {
    private final static Vector2f START_POSITION = new Vector2f(922.5f, 515);
    private final static int STEP = 20;
    private List<ShotgunAmmoHud> ammoImages;
    private final ValueObserver<Integer> currentClipObserver;
    private final ValueObserver<Integer> maxClipObserver;

    public ShotgunClipHud(final Supplier<Integer> currentClipSupplier,
                          final Supplier<Integer> maxClipSupplier) {
        maxClipObserver = new ValueObserver<>(maxClipSupplier, this::updatedSize);
        currentClipObserver = new ValueObserver<>(currentClipSupplier, this::updatedClip);
    }
    private static class ValueObserver<E> {
        private final Supplier<E> supplier;
        private final Consumer<E> consumer;
        private E value;

        public ValueObserver(Supplier<E> supplier, Consumer<E> consumer) {
            this.supplier = supplier;
            this.consumer = consumer;
            this.value = supplier.get();
            consumer.accept(value);
        }

        public void update() {
            final E newValue = supplier.get();
            if (!value.equals(newValue)) {
                value = newValue;
                consumer.accept(value);
            }
        }
    }
    public void update(final double dt){
        currentClipObserver.update();
        maxClipObserver.update();
        ammoImages.forEach(ammoImage -> ammoImage.update((float) dt));
    }
    private void updatedClip(final int currentClip) {
        for (int i = 0; i < ammoImages.size(); i++) {
            final ShotgunAmmoHud ammoImage = ammoImages.get(i);
            if (i < currentClip) {
                ammoImage.setVisible(true);
            } else {
                ammoImage.setVisible(false);
            }
        }
    }
    private void updatedSize(final int maxClipAmmo) {
        ammoImages = new ArrayList<>();
        for (int i = 0; i < maxClipAmmo; i++) {
            final ShotgunAmmoHud ammoImage = new ShotgunAmmoHud();
            ammoImage.setPosition(START_POSITION.x, START_POSITION.y - i * STEP);
            ammoImages.add(ammoImage);
        }
        ammoImages = List.copyOf(ammoImages);
    }

    public void collectSubRenderables(RenderableState state) {
        state.addAll(ammoImages);
    }
}

