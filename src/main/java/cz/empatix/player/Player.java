package cz.empatix.player;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.physics.IntersectionResolver;
import cz.empatix.physics.Moveable;
import cz.empatix.render.Camera;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.shaderbindings.CommonShaderBinding;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;

public class Player implements Drawable, Moveable {
    private final static int DEFAULT_MAX_HEALTH = 10;
    private final static int DEFAULT_MAX_ARMOR = 5;
    private final static int DIMENSION_WIDTH = 32;
    private final static int DIMENSION_HEIGHT = 76;
    private final static int BASE_SPEED = 200;

    private boolean left, right, top, bottom;

    private int maxHealth;
    private int health;

    private int maxArmor;
    private int armor;

    private final Animation animation;
    private final Vector2f position;

    public Player(){
        maxHealth = DEFAULT_MAX_HEALTH;
        health = maxHealth;

        maxArmor = DEFAULT_MAX_ARMOR;
        armor = maxArmor;

        animation = Animation.builder()
                .addAnimationState(AnimationState.IDLE, 0, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addAnimationState(AnimationState.RIGHT_MOVE, 1, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addAnimationState(AnimationState.BOTTOM_MOVE, 2, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addAnimationState(AnimationState.TOP_MOVE, 3, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .texture(AssetsService.getTexture("ingame/player.tga"))
                .build();
        animation.setAnimationState(AnimationState.IDLE);

        position = new Vector2f();
    }

    public void update(final double dt){
        animation.update();
    }

    public void fixedUpdate(final double dt){
        move(dt);
    }

    public void setHealth(final int health){
        this.health = health;
        if(this.health > maxHealth){
            this.health = maxHealth;
        }
    }
    public void setArmor(final int armor){
        this.armor = armor;
        if(this.armor > maxArmor){
            this.armor = maxArmor;
        }
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position),
                animation.getTextureId(),
                new CommonShaderBinding(Camera.getProjection()),
                RenderData.RenderDataType.DYNAMIC
        );
    }

    @Override
    public boolean shouldRender() {
        return IntersectionResolver.newInstance()
                .addShape(position, new Vector2f(DIMENSION_WIDTH, DIMENSION_HEIGHT))
                .addShape(Camera.getPosition(), new Vector2f(Camera.getTargetWidth(), Camera.getTargetHeight()))
                .resolve();
    }

    @Override
    public int getZIndex() {
        return 2;
    }


    public void keyPressed(final int key){
        switch (key){
            case GLFW.GLFW_KEY_W -> {
                top = true;
                animation.setAnimationState(AnimationState.TOP_MOVE);
            }
            case GLFW.GLFW_KEY_S -> {
                bottom = true;
                animation.setAnimationState(AnimationState.BOTTOM_MOVE);
            }
            case GLFW.GLFW_KEY_A -> {
                left = true;
                animation.setAnimationState(AnimationState.RIGHT_MOVE);
            }
            case GLFW.GLFW_KEY_D -> {
                right = true;
                animation.setAnimationState(AnimationState.RIGHT_MOVE);
            }
        }
    }

    public void keyReleased(final int key) {
        switch (key){
            case GLFW.GLFW_KEY_W -> top = false;
            case GLFW.GLFW_KEY_S -> bottom = false;
            case GLFW.GLFW_KEY_A -> left = false;
            case GLFW.GLFW_KEY_D -> right = false;
        }
        if(!top && !bottom && !left && !right){
            animation.setAnimationState(AnimationState.IDLE);
        }
    }

    @Override
    public void move(double dt) {
        if(top){
            position.y -= BASE_SPEED * dt;
        }
        if(bottom){
            position.y += BASE_SPEED * dt;
        }
        if(left){
            position.x -= BASE_SPEED * dt;
        }
        if(right){
            position.x += BASE_SPEED * dt;
        }
    }
}
