package cz.empatix.items.domain;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.audio.domain.SoundEffect;
import cz.empatix.audio.service.AudioService;
import cz.empatix.hud.service.image.AlternativeViewImage;
import cz.empatix.hud.service.text.AlternativeViewText;
import cz.empatix.interaction.service.InteractionTracker;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.Set;
import java.util.concurrent.ThreadLocalRandom;
import java.util.function.Consumer;
import java.util.function.Predicate;

public class ShopItem implements ItemDrop {
    private final static AudioService AUDIO_SERVICE = AudioService.getInstance();
    private final static int ITEM_OFFSET = 15;
    private static final int INTERACTION_TEXT_OFFSET = 20;
    private final static Logger LOGGER = LoggerFactory.getLogger(ShopItem.class);
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final ItemDrop drop;
    private final Predicate<Integer> hasEnoughCoinsPredicate;
    private final Consumer<Integer> removeCoinsConsumer;
    private final AlternativeViewText text;
    private final AlternativeViewImage image;
    private final InteractionTracker interactionTracker = new InteractionTracker();
    private final AlternativeViewText interactionText;
    private final int price;
    private boolean bought = false;

    public ShopItem(final ItemDrop itemDrop,
                    final Predicate<Integer> hasEnoughCoinsPredicate,
                    final Consumer<Integer> removeCoinsConsumer) {
        this.hasEnoughCoinsPredicate = hasEnoughCoinsPredicate;
        this.removeCoinsConsumer = removeCoinsConsumer;
        this.drop = itemDrop;
        this.price = 1;
        this.text = new AlternativeViewText(String.valueOf(price), "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_22, Camera.getProjection());
        this.text.setColorRGB(212, 168, 8);
        this.text.setPosition(drop.getPosition().x() + drop.getWidth() / 2f - this.text.getWidth() / 2f,
                drop.getPosition().y() + drop.getHeight() + 10);
        this.image = new AlternativeViewImage("ingame/shop_price_hud.tga", new Vector2f(), 1,2, RenderScene.HUD, Camera.getProjection());
        this.image.setPosition(drop.getPosition().x() + drop.getWidth() / 2f - this.image.getWidth() / 2f,
                drop.getPosition().y() + drop.getHeight() + 10);
        this.interactionText = new AlternativeViewText("Press E to buy", "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_32, Camera.getProjection());
        this.interactionText.setColorRGB(222, 112, 38);
    }

    @Override
    public List<Renderable> getSubRenderables() {
        if(bought){
           return List.of();
        }
        final List<Renderable> textRenderables = text.getSubRenderables();
        textRenderables.add(image);
        if (interactionTracker.isInteracting()) {
            textRenderables.addAll(interactionText.getSubRenderables());
        }
        return textRenderables;
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
        drop.setPosition(x, y - ITEM_OFFSET);
    }
    public void setHudPosition(final float x, final float y) {
        this.text.setCenterX(x - image.getWidth()/2f, x + image.getWidth()/2f);
        this.text.setCenterY(y - image.getHeight() - ITEM_OFFSET, y - ITEM_OFFSET);
        this.image.setPosition(x - image.getWidth()/2f, y - image.getHeight() - ITEM_OFFSET);

        this.interactionText.setCenterY(y - ITEM_OFFSET, y - ITEM_OFFSET);
        this.interactionText.setCenterX(x,x);
    }

    @Override
    public void interact(final Set<Integer> keys) {
        interactionTracker.interaction();
        if (!bought && !keys.contains(GLFW.GLFW_KEY_E)) {
            return;
        }
        if (!bought && !hasEnoughCoinsPredicate.test(price)) {
            LOGGER.info("Unsuccessfully bought {}, do no have {} coins", drop, price);
            return;
        } else {
            LOGGER.info("Successfully bought {} for {}", drop, price);
            removeCoinsConsumer.accept(price);
            bought = true;
            AUDIO_SERVICE.play(SoundEffect.BUY);
        }
        if (drop.canInteract()) {
            drop.interact(keys);
        }
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
        return true;
    }

    @Override
    public void setDespawnable(final boolean despawn) {
        drop.setDespawnable(despawn);
    }

    public void setInteractionText(final float xMin, final float xMax, final float y) {
        this.interactionText.setCenterX(xMin, xMax);
        this.interactionText.setPosition(interactionText.getX(), y + INTERACTION_TEXT_OFFSET);
    }
}
