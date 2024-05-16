package cz.empatix.roomobjects.domain;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.domain.SoundEffect;
import cz.empatix.audio.service.AudioService;
import cz.empatix.common.utils.DelayedTask;
import cz.empatix.common.utils.Task;
import cz.empatix.hud.service.text.AlternativeViewText;
import cz.empatix.interaction.domain.Interactable;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.interaction.service.InteractionTracker;
import cz.empatix.items.domain.ItemDrop;
import cz.empatix.items.service.ItemService;
import cz.empatix.player.CoinService;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;

import java.util.List;
import java.util.Set;
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
        machineSlotService = new MachineSlotService(coinService, itemService);
        animation = Animation.builder()
                .setTotal(1, 1)
                .addState(AnimationState.IDLE, 0, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .texture(AssetsService.getTexture("ingame/slotmachine.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        position = new Vector2f();
        interactionTracker = new InteractionTracker();

        this.interactionText = new AlternativeViewText("Press E to buy", "PixelFJVerdana12pt.ttf", new Vector2f(), 2, FontSize.FONTSIZE_22, Camera.getProjection());
        this.interactionText.setColorRGB(222, 112, 38);

        coinService.addCoins(100);

        InteractionService.getInstance().addInteractable(this);
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position),
                animation.getTextureId(),
                new CommonBinding(Camera.getProjection())
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
        return Camera.isInProjection(position, DIMENSION_WIDTH, DIMENSION_HEIGHT);
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
        interactionTracker.interaction();
        if (keys.contains(GLFW.GLFW_KEY_E)) {
            machineSlotService.spin();
        }
    }

    @Override
    public boolean canInteract() {
        return !machineSlotService.isSpinning();
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
    }

    @Override
    public List<Renderable> getSubRenderables() {
        if (interactionTracker.isInteracting()) {
            return interactionText.getSubRenderables();
        }
        return List.of();
    }
    private static class MachineSlotService {
        private static final ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
        private static final int CASHOUT_CHANCE = 5;
        private static final int WIN_CHANCE = 33;
        private static final int LOSE_CHANCE = 66;
        private static final AudioService AUDIO_SERVICE = AudioService.getInstance();
        private static final int PRICE_COST = 2;
        private static final int SPINNING_TIME = 3;
        private final Task spinTask;
        private final CoinService coinService;
        private final ItemService itemService;
        private boolean spinning = false;
        public MachineSlotService(final CoinService coinService,
                                  final ItemService itemService) {
            this.coinService = coinService;
            this.itemService = itemService;
            spinTask = new DelayedTask(SPINNING_TIME, () -> {
                spinning = false;
                final int chance = RANDOM.nextInt(WIN_CHANCE + LOSE_CHANCE + CASHOUT_CHANCE);
                if (chance < WIN_CHANCE) {
                    coinService.addCoins(10);
                    AUDIO_SERVICE.play(SoundEffect.SLOTMACHINE_WIN);
                } else if (chance < WIN_CHANCE + LOSE_CHANCE) {
                    AUDIO_SERVICE.play(SoundEffect.SLOTMACHINE_LOSE);
                } else {
                    final ItemDrop drop = itemService.getItemFactory().createItemDrop();
                    AUDIO_SERVICE.play(SoundEffect.SLOTMACHINE_CASHOUT);
                }
            });
        }
        public void update(final float dt) {
            spinTask.update(dt);
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
            coinService.removeCoins(PRICE_COST);
            AUDIO_SERVICE.play(SoundEffect.SLOTMACHINE_SPIN);
            spinTask.start();
            spinning = true;
        }
    }
}
