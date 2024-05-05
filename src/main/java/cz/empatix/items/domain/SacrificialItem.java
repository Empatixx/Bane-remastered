package cz.empatix.items.domain;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.audio.domain.SoundEffect;
import cz.empatix.audio.service.AudioService;
import cz.empatix.hud.service.image.AlternativeViewImage;
import cz.empatix.hud.service.text.AlternativeViewText;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.interaction.service.InteractionTracker;
import cz.empatix.lightning.domain.LightSource;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import org.dyn4j.geometry.Vector3;
import org.joml.Vector2f;
import org.joml.Vector3f;
import org.lwjgl.glfw.GLFW;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.Set;
import java.util.concurrent.ThreadLocalRandom;
import java.util.function.Consumer;
import java.util.function.Predicate;

public class SacrificialItem implements ItemDrop{
    private static final int MAX_HEALTH_PRICE = 3;
    private final static Runnable NOOP = () -> {
    };
    private final static AudioService AUDIO_SERVICE = AudioService.getInstance();
    private final static int ITEM_OFFSET = 15;
    private static final int INTERACTION_TEXT_OFFSET = 20;
    private final static Logger LOGGER = LoggerFactory.getLogger(ShopItem.class);
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final ItemDrop drop;
    private final Predicate<Integer> hasEnoughHealthPredicate;
    private final Consumer<Integer> removeHealthConsumer;
    private final AlternativeViewText saleText;
    private final AlternativeViewText priceText;
    private final AlternativeViewImage image;
    private final InteractionTracker interactionTracker = new InteractionTracker();
    private final AlternativeViewText interactionText;
    private final LightSource lightSource;
    private Runnable onBuy;
    private int price;
    private boolean bought = false;

    public SacrificialItem(final ItemDrop itemDrop,
                    final Predicate<Integer> hasEnoughHealthPredicate,
                    final Consumer<Integer> removeHealthConsumer) {
        this.hasEnoughHealthPredicate = hasEnoughHealthPredicate;
        this.removeHealthConsumer = removeHealthConsumer;
        this.drop = itemDrop;
        this.price = RANDOM.nextInt(1, MAX_HEALTH_PRICE + 1);
        this.priceText = new AlternativeViewText(String.valueOf(price), "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_22, Camera.getProjection());
        this.priceText.setColorRGB(212, 160, 0);
        this.priceText.setPosition(drop.getPosition().x() + drop.getWidth() / 2f - this.priceText.getWidth() / 2f,
                drop.getPosition().y() + drop.getHeight() + 10);
        this.image = new AlternativeViewImage("ingame/sacrifice_price_hud.tga", new Vector2f(), 1, 2, RenderScene.HUD, Camera.getProjection());
        this.image.setPosition(drop.getPosition().x() + drop.getWidth() / 2f - this.image.getWidth() / 2f,
                drop.getPosition().y() + drop.getHeight() + 10);
        this.interactionText = new AlternativeViewText("Press E to buy", "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_22, Camera.getProjection());
        this.interactionText.setColorRGB(222, 112, 38);

        this.saleText = new AlternativeViewText("", "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_22, Camera.getProjection());
        this.saleText.setColorRGB(122, 248, 18);
        onBuy = NOOP;

        lightSource = LightningService.getInstance().createLightSource(
                new Vector3f(212, 0, 0),
                250,
                1f
        );
    }


    @Override
    public void interact(Set<Integer> keys) {
        interactionTracker.interaction();
        if (bought) {
            if (drop.canInteract()) {
                drop.interact(keys);
            }
            return;
        }
        if (!keys.contains(GLFW.GLFW_KEY_E)) {
            return;
        }
        if (!hasEnoughHealthPredicate.test(price)) {
            LOGGER.info("Unsuccessfully bought {}, do no have {} health", drop, priceText);
            return;
        } else {
            LOGGER.info("Successfully bought {} for {}", drop, priceText);
            removeHealthConsumer.accept(price);
            bought = true;
            AUDIO_SERVICE.play(SoundEffect.SACRIFICE);
            onBuy.run();
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
    public void update(float dt) {
        drop.update(dt);
        lightSource.setPosition(drop.getPosition().x() + drop.getWidth() / 2f, drop.getPosition().y() + drop.getHeight() / 2f);
    }

    @Override
    public boolean canRemove() {
        return drop.canRemove();
    }

    @Override
    public boolean isPickedUp() {
        return drop.isPickedUp();
    }

    @Override
    public void setPosition(float x, float y) {
        drop.setPosition(x, y - ITEM_OFFSET);
    }

    @Override
    public void setDespawnable(boolean despawn) {
        drop.setDespawnable(despawn);
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
        lightSource.destroy();
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
    public List<Renderable> getSubRenderables() {
        if (bought) {
            return List.of();
        }
        final List<Renderable> textRenderables = priceText.getSubRenderables();
        textRenderables.add(image);
        textRenderables.addAll(saleText.getSubRenderables());
        if (interactionTracker.isInteracting()) {
            textRenderables.addAll(interactionText.getSubRenderables());
        }
        return textRenderables;
    }

    public void setHudPosition(final float x, final float y){
        this.priceText.setCenterX(x - image.getWidth() / 2f, x + image.getWidth() / 2f);
        this.priceText.setCenterY(y - image.getHeight() - ITEM_OFFSET, y - ITEM_OFFSET);

        this.saleText.setCenterX(x - image.getWidth() / 2f, x + image.getWidth() / 2f);
        this.saleText.setCenterY(y - image.getHeight() - ITEM_OFFSET - 15, y - image.getHeight() - ITEM_OFFSET - 15);

        this.image.setPosition(x - image.getWidth() / 2f, y - image.getHeight() - ITEM_OFFSET);

        this.interactionText.setCenterY(y - ITEM_OFFSET, y - ITEM_OFFSET);
        this.interactionText.setCenterX(x, x);
    }

    public void setInteractionText(final float xMin, final float xMax, final float y) {
        this.interactionText.setCenterX(xMin, xMax);
        this.interactionText.setPosition(interactionText.getX(), y + INTERACTION_TEXT_OFFSET);
    }
}
