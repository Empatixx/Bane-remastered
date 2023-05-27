package cz.empatix.player;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.hud.domain.StatusBarType;
import cz.empatix.hud.service.Image;
import cz.empatix.hud.service.StatusBar;
import cz.empatix.hud.service.Text;
import cz.empatix.lightning.domain.LightSource;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.domain.Rectangle;
import cz.empatix.physics.service.IntersectionResolver;
import cz.empatix.physics.service.PhysicsBodyFactory;
import cz.empatix.postprocessing.domain.RedVignette;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.joml.Vector3f;
import org.lwjgl.glfw.GLFW;

import java.util.List;
import java.util.stream.Stream;

public class Player implements Drawable {
    private final static int SCALE = 2;
    private final static int COIN_HUD_OFFSET = 25;
    private final static int DEFAULT_MAX_HEALTH = 10;
    private final static int DEFAULT_MAX_ARMOR = 5;
    private final static int DIMENSION_WIDTH = 16;
    private final static int DIMENSION_HEIGHT = 38;
    private final static int MOVE_FORCE = 30000000;
    private final static float LOW_HP_THRESHOLD = 0.3f;
    private final StatusBar healthBar;
    private final StatusBar armorBar;
    private final Image coinHud;
    private final Text coinText;
    private final Animation animation;
    private final PhysicsBody physicsBody;
    private final LightSource lightSource;
    private boolean left, right, top, bottom;

    private int maxHealth;
    private int health;
    private int maxArmor;
    private int armor;
    private int coins;

    public Player() {
        maxHealth = DEFAULT_MAX_HEALTH;
        health = maxHealth;

        maxArmor = DEFAULT_MAX_ARMOR;
        armor = maxArmor;

        coins = 0;

        animation = Animation.builder()
                .setTotal(4,6)
                .addState(AnimationState.IDLE, 0, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addState(AnimationState.RIGHT_MOVE, 1, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addState(AnimationState.LEFT_MOVE, 1, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100, true)
                .addState(AnimationState.BOTTOM_MOVE, 2, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addState(AnimationState.TOP_MOVE, 3, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .texture(AssetsService.getTexture("ingame/player.tga"))
                .scale(SCALE)
                .build();
        animation.setState(AnimationState.IDLE);
        physicsBody = PhysicsBodyFactory.createDynamic(Rectangle.from(DIMENSION_WIDTH * SCALE, DIMENSION_HEIGHT * SCALE));
        physicsBody.setTranslation(500, 500);

        lightSource = LightningService.getInstance().createLightSource(new Vector3f(216, 156, 36), 150f, 0.5f);

        healthBar = new StatusBar(this::getHealth, this::getMaxHealth, StatusBarType.HEALTH, new Vector2f(25, 25));
        armorBar = new StatusBar(this::getArmor, this::getMaxArmor, StatusBarType.ARMOR, new Vector2f(75, 62.5f));

        final TextureAtlas.Entry coinTxt = AssetsService.getTexture("ingame/banecoin.tga");
        coinHud = new Image(coinTxt, new Vector2f(COIN_HUD_OFFSET, WindowHandler.TARGET_HEIGHT-COIN_HUD_OFFSET-coinTxt.height()), 1f);
        coinText = new Text(String.valueOf(coins), "PixelFJVerdana12pt.ttf", new Vector2f(COIN_HUD_OFFSET+coinTxt.width()+15, 0f), coinHud.getZIndex());

        coinText.setCenterY(WindowHandler.TARGET_HEIGHT-COIN_HUD_OFFSET, WindowHandler.TARGET_HEIGHT-COIN_HUD_OFFSET-coinTxt.height());
        coinText.setColorRGB(255,216, 0);
    }

    public int getArmor() {
        return armor;
    }

    public int getMaxArmor() {
        return maxArmor;
    }

    public void update(final double dt) {
        animation.update();
        lightSource.setPosition(physicsBody.getTranslationX() + DIMENSION_WIDTH * SCALE / 2,
                physicsBody.getTranslationY() + DIMENSION_HEIGHT * SCALE / 2);
        healthBar.update(dt);
        armorBar.update(dt);

        if((float) health / maxHealth < LOW_HP_THRESHOLD) {
            RedVignette.setTimeBased(true);
        } else {
            RedVignette.setTimeBased(false);
        }
    }
    public void hit(final int damage){
        if(armor > 0){
            armor -= damage;
            if(armor < 0){
                health += armor;
                armor = 0;
            }
        } else {
            health -= damage;
        }
        RedVignette.setMaxVignette();
    }

    public void fixedUpdate(final double dt) {
        move(dt);
    }

    public void setHealth(final int health) {
        this.health = health;
        if (this.health > maxHealth) {
            this.health = maxHealth;
        }
        if (this.health < 0) {
            this.health = 0;
        }
    }

    public void setArmor(final int armor) {
        this.armor = armor;
        if (this.armor > maxArmor) {
            this.armor = maxArmor;
        }
        if (this.armor < 0) {
            this.armor = 0;
        }
    }

    @Override
    public List<Renderable> getSubRenderables() {
        List<? extends Renderable> renderables = Stream.of(coinText.getSubRenderables(), List.of(healthBar, armorBar, coinHud))
                .flatMap(List::stream)
                .toList();
        return List.copyOf(renderables);
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(physicsBody.getTranslation()),
                animation.getTextureId(),
                new CommonBinding(Camera.getProjection())
        );
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }


    @Override
    public boolean canRender() {
        return IntersectionResolver.newInstance()
                .addShape(physicsBody.getTranslation(), new Vector2f(DIMENSION_WIDTH, DIMENSION_HEIGHT))
                .addShape(Camera.getPosition(), new Vector2f(Camera.getTargetWidth(), Camera.getTargetHeight()))
                .resolve();
    }

    @Override
    public int getZIndex() {
        return 2;
    }


    public void keyPressed(final int key) {
        switch (key) {
            case GLFW.GLFW_KEY_W -> {
                top = true;
                bottom = false;
            }
            case GLFW.GLFW_KEY_S -> {
                bottom = true;
                top = false;
            }
            case GLFW.GLFW_KEY_A -> {
                left = true;
                right = false;
            }
            case GLFW.GLFW_KEY_D -> {
                right = true;
                left = false;
            }
        }
    }

    public void keyReleased(final int key) {
        switch (key) {
            case GLFW.GLFW_KEY_W -> top = false;
            case GLFW.GLFW_KEY_S -> bottom = false;
            case GLFW.GLFW_KEY_A -> left = false;
            case GLFW.GLFW_KEY_D -> right = false;
        }
    }

    private void move(double dt) {
        if (left) {
            physicsBody.applyForce(-MOVE_FORCE, 0);
        } else if (right) {
            physicsBody.applyForce(MOVE_FORCE, 0);
        }
        if (top) {
            physicsBody.applyForce(0, -MOVE_FORCE);
        } else if (bottom) {
            physicsBody.applyForce(0, MOVE_FORCE);
        }

        if (left) {
            animation.setState(AnimationState.LEFT_MOVE);
        } else if (right) {
            animation.setState(AnimationState.RIGHT_MOVE);
        } else if (top) {
            animation.setState(AnimationState.TOP_MOVE);
        } else if (bottom) {
            animation.setState(AnimationState.BOTTOM_MOVE);
        } else {
            animation.setState(AnimationState.IDLE);
        }
    }
    public Vector2f getPosition(){
        return physicsBody.getPhysicsTranslation();
    }
    public float getX() {
        return physicsBody.getPhysicsTranslationX();
    }

    public float getY() {
        return physicsBody.getPhysicsTranslationY();
    }

    public int getWidth() {
        return DIMENSION_WIDTH;
    }

    public int getHeight() {
        return DIMENSION_HEIGHT;
    }

    public int getHealth() {
        return health;
    }

    public int getMaxHealth() {
        return maxHealth;
    }

}
