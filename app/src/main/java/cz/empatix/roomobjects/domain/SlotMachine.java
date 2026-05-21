package cz.empatix.roomobjects.domain;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.sound.SoundEffect;
import cz.empatix.audio.service.AudioService;
import cz.empatix.common.utils.DelayedTask;
import cz.empatix.common.utils.Task;
import cz.empatix.hud.service.image.AlternativeViewImage;
import cz.empatix.hud.service.text.AlternativeViewText;
import cz.empatix.interaction.domain.Interactable;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.interaction.service.InteractionTracker;
import cz.empatix.items.domain.ItemRarity;
import cz.empatix.items.domain.MovableItemDrop;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;
import cz.empatix.player.CoinService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.animation.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;
import java.util.concurrent.ThreadLocalRandom;

public class SlotMachine implements RoomObject, Interactable {
    private final static int SCALE = 3;
    private final static int DIMENSION_WIDTH = 64 * SCALE;
    private final static int DIMENSION_HEIGHT = 64 * SCALE;
    private final Animation animation;
    private final Vector2f position;
    private final InteractionTracker interactionTracker;
    private final AlternativeViewText interactionText;
    private final MachineSlotService machineSlotService;
    public SlotMachine(final ItemService itemService,
                       final CoinService coinService) {
        machineSlotService = new MachineSlotService(coinService, itemService, this);
        animation = Animation.builder()
                .setTotal(3, 6)
                .addState(AnimationState.SPIN, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addState(AnimationState.WIN, 1, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addState(AnimationState.LOSE, 2, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .texture(AssetsService.getTexture("ingame/slotmachine.tga"))
                .build();
        animation.setState(AnimationState.LOSE);
        position = new Vector2f();
        interactionTracker = new InteractionTracker();

        this.interactionText = new AlternativeViewText("Press E to spin", "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_22);
        this.interactionText.setColorRGB(222, 112, 38);

        coinService.addCoins(100);

        InteractionService.getInstance().addInteractable(this);
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position, getZIndex()),
                animation.getTextureId(),
                new CommonLightBinding()
        );
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }

    @Override
    public int getZIndex() {
        return 3;
    }

    @Override
    public boolean canRender() {
        return Camera.getInstance().isInProjection(position, DIMENSION_WIDTH, DIMENSION_HEIGHT);
    }

    @Override
    public void update(double dt) {
        animation.update();
        machineSlotService.update((float) dt);
    }

    @Override
    public boolean canRemove() {
        return false;
    }

    @Override
    public int getWidth() {
        return DIMENSION_WIDTH;
    }

    @Override
    public int getHeight() {
        return DIMENSION_HEIGHT;
    }

    @Override
    public void interact(Set<Integer> keys) {
        if (keys.contains(GLFW.GLFW_KEY_E)) {
            machineSlotService.spin();
        }
    }

    @Override
    public boolean canInteract(Set<Integer> keys) {
        if (!machineSlotService.isSpinning()){
            interactionTracker.interaction();
        }
        return !machineSlotService.isSpinning() && keys.contains(GLFW.GLFW_KEY_E);
    }

    @Override
    public void destroy() {
        InteractionService.getInstance().removeInteractable(this);
    }

    @Override
    public Vector2f getPosition() {
        return position;
    }

    public void setPosition(final float x, final float y) {
        position.set(x, y);
        interactionText.setPosition(x,y + DIMENSION_HEIGHT - 10);
        interactionText.setCenterX(x, x + DIMENSION_WIDTH);
        machineSlotService.setPosition(x, y);
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
        machineSlotService.collectSubRenderables(state);
        if (interactionTracker.isInteracting()) {
            interactionText.collectSubRenderables(state);
        }
    }
    private static class MachineSlotService {
        private static final Logger LOGGER = LoggerFactory.getLogger(MachineSlotService.class);
        private static final ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
        private static final AudioService AUDIO_SERVICE = AudioService.getInstance();
        private static final int PRICE_COST = 5;
        private static final int SPINNING_TIME = 2;
        private final Task spinTask;
        private final CoinService coinService;
        private final ItemService itemService;
        private boolean spinning = false;
        private final SlotMachine slotMachine;
        private final NavigableMap<Integer, Runnable> rewards;

        private final AlternativeViewText priceText;
        private final AlternativeViewImage priceHud;

        public MachineSlotService(final CoinService coinService,
                                  final ItemService itemService,
                                  final SlotMachine slotMachine) {
            this.coinService = coinService;
            this.itemService = itemService;
            this.slotMachine = slotMachine;

            int currentChance = 0;
            rewards = new TreeMap<>();
            rewards.put(currentChance += 10, this::winRareItem);
            rewards.put(currentChance += 10, this::cashout);
            rewards.put(currentChance += 10, this::dropItems);
            rewards.put(currentChance += 10, this::winTriple);
            rewards.put(currentChance += 20, this::winDouble);
            rewards.put(currentChance += 90, this::lose);

            final int total = currentChance;
            spinTask = new DelayedTask(SPINNING_TIME, () -> {
                spinning = false;
                final int chance = RANDOM.nextInt(total);
                LOGGER.info("Chance: {}", chance);
                rewards.higherEntry(chance).getValue().run();
            });

            this.priceText = new AlternativeViewText(String.valueOf(PRICE_COST), "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_22);
            this.priceText.setColorRGB(212, 168, 8);
            this.priceHud = new AlternativeViewImage("ingame/shop_price_hud.tga", new Vector2f(), 1, 2, RenderScene.HUD);
            setPosition(0, 0);
        }

        public void setPosition(final float x, final float y) {
            priceHud.setPosition(x + slotMachine.getWidth() / 2f - priceHud.getWidth() / 2f, y);

            final float xMin = priceHud.getX();
            final float xMax = priceHud.getX() + priceHud.getWidth();
            priceText.setCenterX(xMin, xMax);

            final float yMin = priceHud.getY();
            final float yMax = priceHud.getY() + priceHud.getHeight();

            priceText.setCenterY(yMin, yMax);
        }

        private void dropItems() {
            dropCommonItems(5);
            AUDIO_SERVICE.play(SoundEffect.SLOTMACHINE_WIN);
            slotMachine.animation.setState(AnimationState.WIN);
        }

        private void dropCoins(final int coins) {
            for (int i = 0; i < coins; i++) {
                final MovableItemDrop drop = itemService.getItemFactory().createMovableCoinDrop();
                drop.setPosition(slotMachine.getPosition().x() + slotMachine.getWidth() / 2 - drop.getWidth() / 2,
                        slotMachine.getPosition().y() + slotMachine.getHeight() / 2 - drop.getHeight() / 2);
                // set direction from the center of the slot machine as circle in loop
                final float angle = (float) (Math.PI * 2 * i) / coins;
                final float forceX = (float) Math.cos(angle) * 25_000;
                final float forceY = (float) Math.sin(angle) * 25_000;
                drop.applyForce(forceX, forceY);
                itemService.add(drop);
            }
        }

        public void dropCommonItems(final int totalItems){
            for (int i = 0; i < totalItems; i++) {
                final MovableItemDrop drop = itemService.getItemFactory().createMovableItemDrop(ItemRarity.COMMON);
                drop.setPosition(slotMachine.getPosition().x() + slotMachine.getWidth() / 2 - drop.getWidth() / 2,
                        slotMachine.getPosition().y() + slotMachine.getHeight() / 2 - drop.getHeight() / 2);
                // set direction from the center of the slot machine as circle in loop
                final float angle = (float) (Math.PI * 2 * i) / totalItems;
                final float forceX = (float) Math.cos(angle) * 25_000;
                final float forceY = (float) Math.sin(angle) * 25_000;
                drop.applyForce(forceX, forceY);
                itemService.add(drop);
            }
        }

        private void lose() {
            LOGGER.info("Lose");
            AUDIO_SERVICE.play(SoundEffect.SLOTMACHINE_LOSE);
            slotMachine.animation.setState(AnimationState.LOSE);
        }

        private void winRareItem() {
            final ItemFactory factory = itemService.getItemFactory();
            final MovableItemDrop drop = factory.createMovableItemDrop(ItemRarity.UNCOMMON, ItemRarity.RARE);
            drop.setPosition(slotMachine.getPosition().x() + slotMachine.getWidth() / 2 - drop.getWidth() / 2,
                    slotMachine.getPosition().y() + slotMachine.getHeight() / 2 - drop.getHeight() / 2);
            drop.applyForce(0, 25_000);
            itemService.add(drop);
            AUDIO_SERVICE.play(SoundEffect.SLOTMACHINE_WIN);
            LOGGER.info("Win item: {}", drop);
            slotMachine.animation.setState(AnimationState.WIN);
        }

        public void cashout() {
            LOGGER.info("Cashout");
            dropCoins(10);
            AUDIO_SERVICE.play(SoundEffect.SLOTMACHINE_CASHOUT);
            slotMachine.animation.setState(AnimationState.WIN);
        }
        public void winTriple() {
            LOGGER.info("Win triple");
            dropCoins(PRICE_COST * 3);
            AUDIO_SERVICE.play(SoundEffect.SLOTMACHINE_WIN);
            slotMachine.animation.setState(AnimationState.WIN);
        }
        public void winDouble() {
            LOGGER.info("Win double");
            dropCoins(PRICE_COST * 2);
            AUDIO_SERVICE.play(SoundEffect.SLOTMACHINE_WIN);
            slotMachine.animation.setState(AnimationState.WIN);
        }

        public void update(final float dt) {
            spinTask.update(dt);
            if (slotMachine.animation.getCurrentState() == AnimationState.WIN && slotMachine.animation.hasPlayedOnce()) {
                slotMachine.animation.setCurrentFrame(0);
            }
        }

        public boolean isSpinning() {
            return spinning;
        }

        public void spin() {
            if (spinning) {
                return;
            }
            if (!coinService.hasEnoughCoins(PRICE_COST)) {
                return;
            }
            slotMachine.animation.setState(AnimationState.SPIN);
            coinService.removeCoins(PRICE_COST);
            AUDIO_SERVICE.play(SoundEffect.SLOTMACHINE_SPIN);
            spinTask.start();
            spinning = true;
        }
        public void collectSubRenderables(RenderableState state) {
            if (isSpinning()) {
                return;
            }
            priceText.collectSubRenderables(state);
            state.add(priceHud);
        }
    }
}
