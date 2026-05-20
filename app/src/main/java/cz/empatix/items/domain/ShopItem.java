package cz.empatix.items.domain;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.audio.domain.SoundEffect;
import cz.empatix.audio.service.AudioService;
import cz.empatix.hud.service.image.AlternativeViewImage;
import cz.empatix.hud.service.text.AlternativeViewText;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.interaction.service.InteractionTracker;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Set;
import java.util.concurrent.ThreadLocalRandom;
import java.util.function.Consumer;
import java.util.function.Predicate;

public class ShopItem implements ItemDrop {
    private final static Runnable NOOP = () -> {
    };
    private final static AudioService AUDIO_SERVICE = AudioService.getInstance();
    private final static int ITEM_OFFSET = 15;
    private static final int INTERACTION_TEXT_OFFSET = 20;
    private final static Logger LOGGER = LoggerFactory.getLogger(ShopItem.class);
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final ItemDrop drop;
    private final Predicate<Integer> hasEnoughCoinsPredicate;
    private final Consumer<Integer> removeCoinsConsumer;
    private final AlternativeViewText saleText;
    private final AlternativeViewText priceText;
    private final AlternativeViewImage image;
    private final InteractionTracker interactionTracker = new InteractionTracker();
    private final AlternativeViewText interactionText;
    private Runnable onBuy;
    private int price;
    private boolean bought = false;

    public ShopItem(final ItemDrop itemDrop,
                    final Predicate<Integer> hasEnoughCoinsPredicate,
                    final Consumer<Integer> removeCoinsConsumer) {
        this.hasEnoughCoinsPredicate = hasEnoughCoinsPredicate;
        this.removeCoinsConsumer = removeCoinsConsumer;
        this.drop = itemDrop;
        this.price = 0;
        this.priceText = new AlternativeViewText(String.valueOf(price), "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_22);
        this.priceText.setColorRGB(212, 168, 8);
        this.priceText.setPosition(drop.getPosition().x() + drop.getWidth() / 2f - this.priceText.getWidth() / 2f,
                drop.getPosition().y() + drop.getHeight() + 10);
        this.image = new AlternativeViewImage("ingame/shop_price_hud.tga", new Vector2f(), 1, 2, RenderScene.HUD);
        this.image.setPosition(drop.getPosition().x() + drop.getWidth() / 2f - this.image.getWidth() / 2f,
                drop.getPosition().y() + drop.getHeight() + 10);
        this.interactionText = new AlternativeViewText("Press E to buy", "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_22);
        this.interactionText.setColorRGB(222, 112, 38);

        this.saleText = new AlternativeViewText("", "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_22);
        this.saleText.setColorRGB(122, 248, 18);
        onBuy = NOOP;
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
        if (bought) {
            return;
        }
        priceText.collectSubRenderables(state);
        state.add(image);
        saleText.collectSubRenderables(state);
        if (interactionTracker.isInteracting()) {
            interactionText.collectSubRenderables(state);
        }
    }

    public void setOnBuy(final Runnable onBuy) {
        this.onBuy = onBuy;
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
    public boolean isPickedUp() {
        return drop.isPickedUp();
    }

    @Override
    public void setPosition(float x, float y) {
        drop.setPosition(x, y - ITEM_OFFSET);
    }

    public void setHudPosition(final float x, final float y) {
        this.priceText.setCenterX(x - image.getWidth() / 2f, x + image.getWidth() / 2f);
        this.priceText.setCenterY(y - image.getHeight() - ITEM_OFFSET, y - ITEM_OFFSET);

        this.saleText.setCenterX(x - image.getWidth() / 2f, x + image.getWidth() / 2f);
        this.saleText.setCenterY(y - image.getHeight() - ITEM_OFFSET - 15, y - image.getHeight() - ITEM_OFFSET - 15);

        this.image.setPosition(x - image.getWidth() / 2f, y - image.getHeight() - ITEM_OFFSET);

        this.interactionText.setCenterY(y - ITEM_OFFSET, y - ITEM_OFFSET);
        this.interactionText.setCenterX(x, x);
    }

    @Override
    public void interact(final Set<Integer> keys) {
        interactionTracker.interaction();
        if (bought) {
            if (drop.canInteract(keys)) {
                drop.interact(keys);
            }
            return;
        }
        if (!keys.contains(GLFW.GLFW_KEY_E)) {
            return;
        }
        if (!hasEnoughCoinsPredicate.test(price)) {
            LOGGER.info("Unsuccessfully bought {}, do no have {} coins", drop, priceText);
            return;
        } else {
            LOGGER.info("Successfully bought {} for {}", drop, priceText);
            removeCoinsConsumer.accept(price);
            bought = true;
            AUDIO_SERVICE.play(SoundEffect.BUY);
            onBuy.run();
        }
        if (drop.canInteract(keys)) {
            drop.interact(keys);
        }
    }

    @Override
    public boolean canInteract(final Set<Integer> keys) {
        return !bought || drop.canInteract(keys);
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
        InteractionService.getInstance().removeInteractable(this);
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

    @Override
    public void setDespawnable(final boolean despawn) {
        drop.setDespawnable(despawn);
    }

    public void setInteractionText(final float xMin, final float xMax, final float y) {
        this.interactionText.setCenterX(xMin, xMax);
        this.interactionText.setPosition(interactionText.getX(), y + INTERACTION_TEXT_OFFSET);
    }

    public void discount(double discountPercent) {
        LOGGER.info("Discounting {} by {}", drop, discountPercent);
        price = (int) (price * discountPercent);
        if (price < 1) {
            price = 1;
        }
        saleText.setText("SALE %d%%".formatted((int) (discountPercent * 100)));
        saleText.setCenterX(saleText.getX() - saleText.getWidth() / 2f, saleText.getX() + saleText.getWidth() / 2f);

        final float priceTextX = priceText.getX() + priceText.getWidth() / 2;
        priceText.setText(String.valueOf(price));
        priceText.setCenterX(priceTextX - priceText.getWidth() / 2f, priceTextX + priceText.getWidth() / 2f);
        priceText.setColorRGB(122, 248, 18);
    }

    public void setPrice(final ShopType type, final int floor) {
        final int price = (int)(RANDOM.nextInt(1, 5) * (floor + 1) * type.getMultiplier());
        this.price = price;
        this.priceText.setText(String.valueOf(price));
        this.priceText.setCenterX(priceText.getX() - priceText.getWidth() / 2f, priceText.getX() + priceText.getWidth() / 2f);
    }
}
