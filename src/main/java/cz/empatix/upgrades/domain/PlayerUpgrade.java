package cz.empatix.upgrades.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import cz.empatix.render.shaderbindings.HudCommonBinding;
import org.joml.Vector2f;

import java.util.function.Supplier;

import static cz.empatix.render.domain.AnimationState.*;

public class PlayerUpgrade implements Drawable {
    private final static int SCALE = 2;
    private final static int DIMENSION_WIDTH = 32 * SCALE;
    private final static int DIMENSION_HEIGHT = 32 * SCALE;
    private final Animation animation;
    private final Vector2f position;
    private boolean bought;
    private final CommonImage upgradeImage;
    private final Supplier<PlayerUpgrade> deepCopySupplier;
    private int price;
    private String title;
    private String description;
    private PlayerUpgrade parent;
    private PlayerUpgradeName name;

    public PlayerUpgrade(final String upgradeImg,
                         final QualityLevel level,
                         final boolean isBought) {
        position = new Vector2f();
        bought = isBought;
        upgradeImage = new CommonImage(upgradeImg, new Vector2f(), 2, 3, RenderScene.OVERLAYS);
        animation = Animation.builder()
                .texture(AssetsService.getTexture("ingame/talent_frame.tga"))
                .setTotal(6, 1)
                .addState(TYPE0, 0, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .addState(TYPE1, 1, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .addState(TYPE2, 2, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .addState(TYPE3, 3, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .addState(TYPE4, 4, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .addState(TYPE5, 5, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .build();
        if (!isBought) {
            switch (level) {
                case COMMON -> animation.setState(AnimationState.TYPE1);
                case RARE -> animation.setState(AnimationState.TYPE3);
                case EPIC -> animation.setState(AnimationState.TYPE5);
            }
        } else {
            switch (level) {
                case COMMON -> animation.setState(TYPE0);
                case RARE -> animation.setState(TYPE2);
                case EPIC -> animation.setState(TYPE4);
            }
        }

        this.deepCopySupplier = () -> {
            final PlayerUpgrade copy = new PlayerUpgrade(upgradeImg, level, isBought());
            copy.setTitle(getTitle());
            copy.setDescription(getDescription());
            copy.setPrice(getPrice());
            copy.setPosition(position.x(), position.y());
            copy.setParent(getParent());
            return copy;
        };
        description = "";
        title = "";
        name = PlayerUpgradeName.UNKNOWN;
    }
    public void setName(final PlayerUpgradeName name){
        this.name = name;
    }
    public boolean canBuy(){
        if (parent == null){
            return true;
        }
        return parent.isBought();
    }
    public void setParent(final PlayerUpgrade parent) {
        this.parent = parent;
    }
    public PlayerUpgrade getParent() {
        return parent;
    }
    public void setPrice(final int price) {
        this.price = price;
    }

    public void setTitle(final String title) {
        this.title = title;
    }

    public void setDescription(final String description) {
        this.description = description;
    }

    public String getDescription() {
        return description;
    }

    public String getTitle() {
        return title;
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position, getZIndex()),
                animation.getTextureId(),
                new HudCommonBinding()
        );
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.OVERLAYS;
    }

    @Override
    public int getZIndex() {
        return 2;
    }

    @Override
    public boolean canRender() {
        return true;
    }

    public void setPosition(final float x, final float y) {
        position.set(x, y);
        upgradeImage.setPosition(x + 16, y + 16);
    }

    public boolean isBought() {
        return bought;
    }

    public boolean isPressed(final float mouseX, final float mouseY) {
        return mouseX >= position.x() && mouseX <= position.x() + DIMENSION_WIDTH
                && mouseY >= position.y() && mouseY <= position.y() + DIMENSION_HEIGHT;
    }

    public int getWidth() {
        return DIMENSION_WIDTH;
    }

    public int getHeight() {
        return DIMENSION_HEIGHT;
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
        state.add(upgradeImage);
    }


    public PlayerUpgrade deepCopy() {
        return deepCopySupplier.get();
    }

    public int getPrice() {
        return price;
    }

    public void setBought(final boolean b) {
        bought = b;
        if (bought){
            switch (animation.getCurrentState()) {
                case TYPE1 -> animation.setState(AnimationState.TYPE0);
                case TYPE3 -> animation.setState(AnimationState.TYPE2);
                case TYPE5 -> animation.setState(AnimationState.TYPE4);
            }
        } else {
            switch (animation.getCurrentState()) {
                case TYPE0 -> animation.setState(AnimationState.TYPE1);
                case TYPE2 -> animation.setState(AnimationState.TYPE3);
                case TYPE4 -> animation.setState(AnimationState.TYPE5);
            }
        }
    }

    public PlayerUpgradeName getName() {
        return name;
    }
}
