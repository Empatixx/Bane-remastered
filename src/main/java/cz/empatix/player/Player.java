package cz.empatix.player;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.domain.SoundEffect;
import cz.empatix.audio.service.AudioService;
import cz.empatix.hud.domain.StatusBarType;
import cz.empatix.hud.service.StatusBar;
import cz.empatix.interaction.domain.CommonInteractEntity;
import cz.empatix.interaction.domain.InteractEntity;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.lightning.domain.LightSource;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.newphysics.attributes.DamageSourceType;
import cz.empatix.newphysics.attributes.Damageable;
import cz.empatix.newphysics.attributes.InterpolatedPhysicsObject;
import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.newphysics.service.CategoryFilter;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.particles.service.GazewindService;
import cz.empatix.postprocessing.domain.RedVignette;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.*;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.geometry.Mass;
import org.dyn4j.geometry.MassType;
import org.dyn4j.geometry.Rectangle;
import org.joml.Vector2f;
import org.joml.Vector3f;
import org.lwjgl.glfw.GLFW;

import java.util.List;
import java.util.Set;

public class Player implements Drawable, InteractEntity, Damageable {
    private final static AudioService AUDIO_SERVICE = AudioService.getInstance();
    private final static int SCALE = 2;
    private final static int DEFAULT_MAX_HEALTH = 10;
    private final static int DEFAULT_MAX_ARMOR = 5;
    private final static int DIMENSION_WIDTH = 16 * SCALE;
    private final static int DIMENSION_HEIGHT = 38 * SCALE;
    private final static int MOVE_FORCE = 3000000;
    private final static float LOW_HP_THRESHOLD = 0.3f;
    private final static int GAZE_OFFSET = 16;
    private final static int GAZE_SPEED_PRELIMIT = 400;
    private final static int INVULNERABLE_TIME = 1000;
    private final StatusBar healthBar;
    private final StatusBar armorBar;
    private final Animation animation;
    private final InterpolatedPhysicsObject physicsBody;
    private final CommonInteractEntity interactEntity;
    private final LightSource lightSource;
    private final GazewindService gazewindService;
    private final CoinService coinService;
    private boolean left, right, top, bottom;
    private int maxHealth;
    private int health;
    private int maxArmor;
    private int armor;
    private long lastHitTime;

    public Player() {
        maxHealth = DEFAULT_MAX_HEALTH;
        health = maxHealth;

        maxArmor = DEFAULT_MAX_ARMOR;
        armor = maxArmor;

        animation = Animation.builder()
                .setTotal(4, 6)
                .addState(AnimationState.IDLE, 0, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addState(AnimationState.RIGHT_MOVE, 1, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addState(AnimationState.LEFT_MOVE, 1, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100, true)
                .addState(AnimationState.BOTTOM_MOVE, 2, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addState(AnimationState.TOP_MOVE, 3, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .texture(AssetsService.getTexture("ingame/player.tga"))
                .build();
        animation.setState(AnimationState.IDLE);

        physicsBody = PhysicsObjectFactory.createInterpolated(DIMENSION_WIDTH, DIMENSION_HEIGHT);
        physicsBody.getFixture(0).setFilter(CategoryFilter.PLAYER);
        physicsBody.setUserData(this);

        interactEntity = new CommonInteractEntity(getWidth(), getHeight(), physicsBody.getPhysicsTranslation());
        InteractionService.getInstance().addEntity(this);

        lightSource = LightningService.getInstance().createLightSource(new Vector3f(216, 156, 36), 150f, 0.5f);

        healthBar = new StatusBar(this::getHealth, this::getMaxHealth, StatusBarType.HEALTH, new Vector2f(25, 25));
        armorBar = new StatusBar(this::getArmor, this::getMaxArmor, StatusBarType.ARMOR, new Vector2f(75, 62.5f));

        gazewindService = new GazewindService();
        coinService = new CoinService();
    }

    private boolean canHit(final DamageSourceType source, final int damage) {
        return source != DamageSourceType.PLAYER && !isInvulnerable() && !isDead();
    }

    private boolean isInvulnerable() {
        return System.currentTimeMillis() - lastHitTime < INVULNERABLE_TIME;
    }

    public int getArmor() {
        return armor;
    }

    public int getMaxArmor() {
        return maxArmor;
    }

    public void update(final double dt) {
        animation.update();
        lightSource.setPosition(physicsBody.getTranslationX() + DIMENSION_WIDTH / 2,
                physicsBody.getTranslationY() + DIMENSION_HEIGHT / 2);
        healthBar.update(dt);
        armorBar.update(dt);

        if ((float) health / maxHealth < LOW_HP_THRESHOLD && !isDead()) {
            RedVignette.setTimeBased(true);
        } else {
            RedVignette.setTimeBased(false);
        }
        gazewindService.update((float) dt);
    }

    public void hit(final int damage) {
        if (isDead()) {
            return;
        }
        if (isInvulnerable()) {
            return;
        }
        if (armor > 0) {
            armor -= damage;
            if (armor < 0) {
                health += armor;
                armor = 0;
            }
        } else {
            health -= damage;
        }
        if (isDead()) {
            animation.setState(AnimationState.IDLE); // player death anim todo
            right = false;
            left = false;
            top = false;
            bottom = false;
            physicsBody.setVelocity(0, 0);
            AUDIO_SERVICE.play(SoundEffect.DEATH);
            return;
        }
        AUDIO_SERVICE.play(SoundEffect.PLAYER_HURT);
        RedVignette.setMaxVignette();
        lastHitTime = System.currentTimeMillis();
    }

    public void fixedUpdate(final double dt) {
        if (isDead()) {
            return;
        }
        move(dt);
        final Vector2f velocity = physicsBody.getVelocity();
        if (Math.abs(velocity.x()) >= GAZE_SPEED_PRELIMIT || Math.abs(velocity.y()) >= GAZE_SPEED_PRELIMIT) {
            final float nextGazeX;
            final float nextGazeY;
            if ((top || bottom) && !left && !right) {
                nextGazeX = getX() + DIMENSION_WIDTH / 2 + GAZE_OFFSET * (float) Math.sin(2 * Math.PI * ((System.currentTimeMillis() % 1000) / 1000d));
                nextGazeY = getY() + DIMENSION_HEIGHT;
            } else if ((right || left) && !top && !bottom) {
                nextGazeX = getX() + DIMENSION_WIDTH / 2;
                nextGazeY = getY() + DIMENSION_HEIGHT + GAZE_OFFSET * (float) Math.sin(Math.PI * (1 + ((System.currentTimeMillis() % 1000) / 1000d)));
            } else {
                nextGazeX = getX() + DIMENSION_WIDTH / 2;
                nextGazeY = getY() + DIMENSION_HEIGHT;
            }
            gazewindService.spawn(nextGazeX, nextGazeY);
        }
    }

    public void addCoins(final int coins) {
        coinService.addCoins(coins);
    }

    public void removeCoins(final int coins) {
        coinService.removeCoins(coins);
    }

    public boolean hasEnoughCoins(final int requiredCoins) {
        return coinService.hasEnoughCoins(requiredCoins);
    }

    public void addHealth(final int health) {
        setHealth(getHealth() + health);
    }

    public boolean hasFullHealth() {
        return getHealth() == getMaxHealth();
    }

    public void addArmor(final int armor) {
        setArmor(getArmor() + armor);
    }

    public boolean hasFullArmor() {
        return getArmor() == getMaxArmor();
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
        final List<Renderable> renderables = coinService.toRender();
        renderables.addAll(List.of(healthBar, armorBar));
        renderables.addAll(gazewindService.toRender());
        return renderables;
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

    public void cleanUp() {
        RedVignette.reset();
    }

    @Override
    public boolean canRender() {
        if (isInvulnerable() && System.currentTimeMillis() / 100 % 2 == 0) {
            return false;
        }
        final Vector2f cameraPos = Camera.getPosition();
        final Vector2f cameraDimensions = new Vector2f(Camera.getTargetWidth(), Camera.getTargetHeight());
        final Vector2f position = physicsBody.getTranslation();
        return position.x() + getWidth() > cameraPos.x()
                && position.x() < cameraPos.x() + cameraDimensions.x()
                && position.y() + getHeight() > cameraPos.y()
                && position.y() < cameraPos.y() + cameraDimensions.y();
    }

    @Override
    public int getZIndex() {
        return 4;
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
        interactEntity.addKey(key);
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

    @Override
    public Set<Integer> getInteractKeys() {
        return interactEntity.getInteractKeys();
    }

    public Vector2f getPosition() {
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

    @Override
    public boolean canRemove() {
        return isDead();
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

    public boolean isDead() {
        return health <= 0;
    }

    public void setPosition(final float x, final float y) {
        physicsBody.setTranslation(x, y);
    }

    @Override
    public void hit(DamageSourceType source, int damage) {
        if (canHit(source, damage)) {
            hit(damage);
        }
    }
}
