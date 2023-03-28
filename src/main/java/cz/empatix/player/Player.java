package cz.empatix.player;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.physics.IntersectionResolver;
import cz.empatix.physics.Moveable;
import cz.empatix.render.Camera;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.InterpolatedPosition;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.shaderbindings.CommonShaderBinding;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;

public class Player implements Drawable, Moveable {
    private final static int SCALE = 2;
    private final static int DEFAULT_MAX_HEALTH = 10;
    private final static int DEFAULT_MAX_ARMOR = 5;
    private final static int DIMENSION_WIDTH = 16;
    private final static int DIMENSION_HEIGHT = 38;
    private final static int BASE_SPEED = 200;

    private boolean left, right, top, bottom;

    private int maxHealth;
    private int health;

    private int maxArmor;
    private int armor;

    private final Animation animation;
    private final InterpolatedPosition position;

    public Player(){
        maxHealth = DEFAULT_MAX_HEALTH;
        health = maxHealth;

        maxArmor = DEFAULT_MAX_ARMOR;
        armor = maxArmor;

        animation = Animation.builder()
                .addAnimationState(AnimationState.IDLE, 0, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addAnimationState(AnimationState.RIGHT_MOVE, 1, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addAnimationState(AnimationState.LEFT_MOVE, 1, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100, true)
                .addAnimationState(AnimationState.BOTTOM_MOVE, 2, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addAnimationState(AnimationState.TOP_MOVE, 3, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .texture(AssetsService.getTexture("ingame/player.tga"))
                .scale(SCALE)
                .build();
        animation.setAnimationState(AnimationState.IDLE);

        position = new InterpolatedPosition();
    }

    public void update(final double dt){
        animation.update();
        position.update((float)dt);
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
                animation.getVertices(position.getInterpolatedPosition()),
                animation.getTextureId(),
                new CommonShaderBinding(Camera.getProjection()),
                RenderData.RenderDataType.DYNAMIC
        );
    }

    @Override
    public boolean shouldRender() {
        return IntersectionResolver.newInstance()
                .addShape(position.getInterpolatedPosition(), new Vector2f(DIMENSION_WIDTH, DIMENSION_HEIGHT))
                .addShape(Camera.getPosition(), new Vector2f(Camera.getTargetWidth(), Camera.getTargetHeight()))
                .resolve();
    }

    @Override
    public int getZIndex() {
        return 2;
    }


    public void keyPressed(final int key){
        switch (key){
            case GLFW.GLFW_KEY_W ->{
                top = true;
                bottom = false;
            }
            case GLFW.GLFW_KEY_S ->{
                bottom = true;
                top = false;
            }
            case GLFW.GLFW_KEY_A ->{
                left = true;
                right = false;
            }
            case GLFW.GLFW_KEY_D ->{
                right = true;
                left = false;
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
    }

    @Override
    public void move(double dt) {
        float x = 0, y = 0;
        if(top){
            y -= BASE_SPEED * dt;
        }
        if(bottom){
            y += BASE_SPEED * dt;
        }
        if(left){
            x -= BASE_SPEED * dt;
        }
        if(right){
            x += BASE_SPEED * dt;
        }

        if (left) {
            animation.setAnimationState(AnimationState.LEFT_MOVE);
        } else if (right){
            animation.setAnimationState(AnimationState.RIGHT_MOVE);
        } else if (top) {
            animation.setAnimationState(AnimationState.TOP_MOVE);
        } else if (bottom){
            animation.setAnimationState(AnimationState.BOTTOM_MOVE);
        } else {
            animation.setAnimationState(AnimationState.IDLE);
        }


        position.setPosition(position.getX() + x, position.getY() + y);
    }
    public float getX(){
        return position.getX();
    }
    public float getY(){
        return position.getY();
    }
    public int getWidth(){
        return DIMENSION_WIDTH;
    }
    public int getHeight(){
        return DIMENSION_HEIGHT;
    }
}
