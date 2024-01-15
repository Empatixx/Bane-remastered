package cz.empatix.ui;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;

public class Slider implements UIElement{
    private final Vector2f position;
    private final Animation animation;
    public Slider(){
        this.position = new Vector2f();
        this.animation = Animation.builder()
                .setTotal(1, 1)
                .addState(AnimationState.STATIC, 0, 1, 128, 8, -1)
                .texture(AssetsService.getTexture("menu/slider.tga"))
                .scale(2)
                .build();
        animation.setState(AnimationState.STATIC);
    }
    public void setPosition(final float x, final float y){
        this.position.set(x, y);
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position),
                animation.getTextureId(),
                new CommonBinding(Camera.getOriginProjection())
        );
    }

    @Override
    public int getZIndex() {
        return 2;
    }

    @Override
    public boolean canRender() {
        return true;
    }

    @Override
    public void update(double dt, double mouseX, double mouseY) {

    }

    @Override
    public void mousePressed(int button, double mouseX, double mouseY) {

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
        return false;
    }
}
