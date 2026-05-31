package cz.empatix.guns.domain;

import cz.empatix.assets.service.AssetSource;
import cz.empatix.guns.ammo.PistolAmmoHud;
import cz.empatix.render.domain.RenderableState;
import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;
import java.util.function.Supplier;

public class SubMachineClipHud {
    private final static Vector2f START_POSITION = new Vector2f(936f, 515);
    private final static int STEP = 15;
    private final static int COLLUMN_STEP = 25;
    private List<PistolAmmoHud> ammoImages;
    private final ValueObserver<Integer> currentClipObserver;
    private final ValueObserver<Integer> maxClipObserver;
    private final AssetSource assetSource;

    public SubMachineClipHud(final AssetSource assetSource,
                             final Supplier<Integer> currentClipSupplier,
                             final Supplier<Integer> maxClipSupplier) {
        this.assetSource = assetSource;
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
            final PistolAmmoHud ammoImage = ammoImages.get(i);
            if (i < currentClip) {
                ammoImage.setVisible(true);
            } else {
                ammoImage.setVisible(false);
            }
        }
    }
    private void updatedSize(final int maxClipAmmo) {
        ammoImages = new ArrayList<>();
        ammoImages.clear();
        final int half = maxClipAmmo / 2;
        for (int collumn = 0; collumn < 2; collumn++) {
            for (int i = 0; i < half; i++) {
                final PistolAmmoHud ammoImage = new PistolAmmoHud(assetSource);
                ammoImage.setPosition(START_POSITION.x - collumn * COLLUMN_STEP, START_POSITION.y - i * STEP);
                ammoImages.add(ammoImage);
            }
        }
        ammoImages = List.copyOf(ammoImages); // Immutable list
    }

    public void collectSubRenderables(RenderableState state) {
        state.addAll(ammoImages);
    }
}

