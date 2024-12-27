package cz.empatix.ui;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.function.Consumer;

public class Checkbox implements UIElement{
    private final static int DEFAULT_Z_INDEX = 1;
    private final static String DEFAULT_TEXTURE = "menu/checkbox.tga";
    private final static Vector2i DEFAULT_DIMENSIONS = new Vector2i(16,16);
    private final static float SCALE = 1.5f;
    private boolean checked;
    private Consumer<Boolean> onCheck;
    private final Vector2f position;
    private final Animation animation;
    private final Vector2i dimensions;
    private boolean hover;
    public Checkbox(){
        animation = Animation.builder()
                .setTotal(4,1)
                .addState(AnimationState.UNSELECTED, 0, 1, DEFAULT_DIMENSIONS.x(), DEFAULT_DIMENSIONS.y(), -1)
                .addState(AnimationState.SELECTED, 1, 1, DEFAULT_DIMENSIONS.x(), DEFAULT_DIMENSIONS.y(), -1)
                .addState(AnimationState.UNSELECTED_HOVER, 2, 1, DEFAULT_DIMENSIONS.x(), DEFAULT_DIMENSIONS.y(), -1)
                .addState(AnimationState.SELECTED_HOVER, 3, 1, DEFAULT_DIMENSIONS.x(), DEFAULT_DIMENSIONS.y(), -1)
                .texture(AssetsService.getTexture(DEFAULT_TEXTURE))
                .scale(SCALE)
                .build();
        animation.setState(AnimationState.UNSELECTED);
        this.position = new Vector2f();
        this.dimensions = new Vector2i((int)(DEFAULT_DIMENSIONS.x() * SCALE), (int)(DEFAULT_DIMENSIONS.y() * SCALE));
    }
    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position, getZIndex()),
                animation.getTextureId(),
                new CommonBinding()
        );
    }

    @Override
    public int getZIndex() {
        return DEFAULT_Z_INDEX;
    }

    @Override
    public boolean canRender() {
        return true;
    }

    @Override
    public void update(double dt, double mouseX, double mouseY) {
        final boolean hovered = mouseIntersectsUI(mouseX, mouseY);
        if (hovered){
            if (checked){
                animation.setState(AnimationState.SELECTED_HOVER);
            }else{
                animation.setState(AnimationState.UNSELECTED_HOVER);
            }
        }else{
            if (checked){
                animation.setState(AnimationState.SELECTED);
            }else{
                animation.setState(AnimationState.UNSELECTED);
            }
        }
    }

    @Override
    public void mousePressed(int button, double mouseX, double mouseY) {
        final boolean hovered = mouseIntersectsUI(mouseX, mouseY);
        if (!hovered){
            return;
        }
        checked = !checked;
        if (onCheck != null){
            onCheck.accept(checked);
        }
    }
    private boolean mouseIntersectsUI(final double mouseX, final double mouseY) {
        return mouseX >= position.x() && mouseX <= position.x() + dimensions.x() &&
                mouseY >= position.y() && mouseY <= position.y() + dimensions.y();
    }

    @Override
    public void mouseReleased(int button, double mouseX, double mouseY) {

    }

    @Override
    public void keyPressed(int key) {

    }

    @Override
    public void keyReleased(int key) {

    }

    @Override
    public void mouseScrolled(double xoffset, double yoffset) {

    }

    @Override
    public boolean isHovered() {
        return hover;
    }

    public void onCheck(final Consumer<Boolean> consumer) {
        onCheck = consumer;
    }

    public void setPosition(final float x, final float y) {
        position.set(x, y);
    }

    public Vector2f getPosition() {
        return position;
    }
    public Vector2i getDimensions(){
        return dimensions;
    }
    public int getHeight(){
        return getDimensions().y();
    }

    public void check(final boolean checked) {
        this.checked = checked;
        animation.setState(checked ? AnimationState.SELECTED : AnimationState.STATIC);
    }
}
