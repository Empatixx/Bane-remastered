package cz.empatix.roomobjects.domain;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.hud.service.image.AlternativeViewImage;
import cz.empatix.hud.service.text.AlternativeViewText;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import org.joml.Vector2f;
import org.joml.Vector3i;

import java.time.Duration;
import java.time.Instant;

public class Shopkeeper implements RoomObject {
    private static final int SHOPKEEPER_WIDTH = 80;
    private static final int SHOPKEEPER_HEIGHT = 90;
    private static final int BOX_SCALE = 6;
    private final static int WIDTH_INSIDE_BOX = 40 * BOX_SCALE;
    private final static int HEIGHT_INSIDE_BOX = 22 * BOX_SCALE;

    private static final int SCALE = 4;
    private final Animation animation;
    private final Vector2f position;
    private final AlternativeViewText sayText;
    private final AlternativeViewImage sayBox;
    private Instant lastTextChange;

    public Shopkeeper() {
        animation = Animation.builder()
                .setTotal(1, 10)
                .addState(AnimationState.IDLE, 0, 10, SHOPKEEPER_WIDTH, SHOPKEEPER_HEIGHT, 170)
                .addState(AnimationState.REVERSE, 0, 10, SHOPKEEPER_WIDTH, SHOPKEEPER_HEIGHT, 170, false, true)
                .texture(AssetsService.getTexture("ingame/shopkeeper.tga"))
                .scale(SCALE)
                .build();
        animation.setState(AnimationState.IDLE);
        position = new Vector2f(0, 0);
        sayText = new AlternativeViewText("", "PixelFJVerdana12pt.ttf", new Vector2f(), 5, FontSize.FONTSIZE_22);
        sayBox = new AlternativeViewImage("ingame/shopkeeper_saybox.tga", new Vector2f(), BOX_SCALE, 1, RenderScene.HUD);
        sayText.setColorRGB(new Vector3i(200, 92, 18));


        lastTextChange = Instant.now().minus(Duration.ofSeconds(3));
    }

    public void setPosition(final float x, final float y) {
        position.x = x;
        position.y = y;
        sayText.setPosition(position.x() - sayBox.getWidth() / 2f, position.y() + HEIGHT_INSIDE_BOX / 2f - 25);
        sayBox.setPosition(position.x() - sayBox.getWidth() / 2f, position.y() - 25);
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
        return 2;
    }

    @Override
    public boolean canRender() {
        return Camera.getInstance().isInProjection(position, SHOPKEEPER_WIDTH * SCALE, SHOPKEEPER_HEIGHT * SCALE);
    }

    @Override
    public void update(double dt) {
        animation.update();
        if (animation.hasPlayedOnce()) {
            if (animation.getCurrentState() == AnimationState.REVERSE) {
                animation.setState(AnimationState.IDLE);
            } else {
                animation.setState(AnimationState.REVERSE);
            }
        }
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
        if (lastTextChange.isBefore(Instant.now().minus(Duration.ofSeconds(3)))) {
            return;
        }
        state.add(sayBox);
        sayText.collectSubRenderables(state);
    }

    @Override
    public boolean canRemove() {
        return false;
    }
    @Override
    public int getWidth() {
        return SHOPKEEPER_WIDTH * SCALE;
    }
    @Override
    public int getHeight() {
        return SHOPKEEPER_HEIGHT * SCALE;
    }

    @Override
    public Vector2f getPosition() {
        return position;
    }

    public void setInteractionText(final String text) {
        sayText.setText(text);
        sayText.setCenterX(position.x() - sayBox.getWidth() / 2f, position.x() - sayBox.getWidth() / 2f + WIDTH_INSIDE_BOX);
        lastTextChange = Instant.now();
    }
}
