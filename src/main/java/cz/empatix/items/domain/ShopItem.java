package cz.empatix.items.domain;

import cz.empatix.interaction.domain.Interactable;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Set;
import java.util.concurrent.ThreadLocalRandom;
import java.util.function.Consumer;
import java.util.function.Predicate;
import java.util.function.Supplier;

public class ShopItem implements ItemDrop {
    private final static Logger LOGGER = LoggerFactory.getLogger(ShopItem.class);
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final ItemDrop drop;
    private final Predicate<Integer> hasEnoughCoinsPredicate;
    private final Consumer<Integer> removeCoinsConsumer;
    private final int price;
    private boolean bought = false;

    public ShopItem(final ItemDrop itemDrop,
                    final Predicate<Integer> hasEnoughCoinsPredicate,
                    final Consumer<Integer> removeCoinsConsumer) {
        this.hasEnoughCoinsPredicate = hasEnoughCoinsPredicate;
        this.removeCoinsConsumer = removeCoinsConsumer;
        this.drop = itemDrop;
        this.price = RANDOM.nextInt(10) + 3;
    }

    @Override
    public void update(float dt) {
        drop.update(dt);
    }

    @Override
    public boolean canRemove() {
        return drop.canRemove() && bought;
    }

    @Override
    public void setPosition(float x, float y) {
        drop.setPosition(x, y);
    }

    @Override
    public void interact(final Set<Integer> keys) {
        if (!bought && keys.contains(GLFW.GLFW_KEY_E)) {
            return;
        }
        if (!hasEnoughCoinsPredicate.test(price)) {
            LOGGER.info("Unsuccessfully bought {} , do no have {} coins", drop, price);
            return;
        }
        LOGGER.info("Successfully bought {} for {}", drop, price);
        removeCoinsConsumer.accept(price);
        if (drop.canInteract()) {
            drop.interact(keys);
        }
        bought = true;

    }

    @Override
    public boolean canInteract() {
        return !bought || drop.canInteract();
    }

    @Override
    public Vector2f getPosition() {
        return drop.getPosition();
    }

    @Override
    public int getWidth() {
        return drop.getWidth();
    }

    @Override
    public int getHeight() {
        return drop.getHeight();
    }

    @Override
    public void destroy() {
        drop.destroy();
    }

    @Override
    public ItemRarity getRarity() {
        return ItemRarity.UNDEFINED;
    }

    @Override
    public RenderData renderData() {
        return drop.renderData();
    }

    @Override
    public RenderScene renderScene() {
        return drop.renderScene();
    }

    @Override
    public int getZIndex() {
        return drop.getZIndex();
    }

    @Override
    public boolean canRender() {
        return drop.canRender();
    }
}
