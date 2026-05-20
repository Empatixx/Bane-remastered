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
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.particles.service.GazewindService;
import cz.empatix.enemies.targetting.Targetable;
import cz.empatix.postprocessing.domain.RedVignette;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.*;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import org.joml.Vector2f;
import org.joml.Vector3f;
import org.lwjgl.glfw.GLFW;

import java.util.List;
import java.util.Set;

public class Player implements Drawable, InteractEntity, Damageable, Targetable {
    private final static AudioService AUDIO_SERVICE = AudioService.getInstance();
    private final static int SCALE = 2;
    private final static int DEFAULT_MAX_HEALTH = 10;
    private final static int DEFAULT_MAX_ARMOR = 5;
    private final static int DIMENSION_WIDTH = 16 * SCALE;
    private final static int DIMENSION_HEIGHT = 38 * SCALE;
    private final static int MOVE_FORCE = 5_000_000;
    private final static float LOW_HP_THRESHOLD = 0.3f;
    private final static int GAZE_OFFSET = 16;
    private final static int GAZE_SPEED_PRELIMIT = 350;
    private final static int INVULNERABLE_TIME = 1000;
    private final StatusBar healthBar;
    private final StatusBar armorBar;
    private final Animation animation;
    private final InterpolatedPhysicsObject physicsBody;
    private final CommonInteractEntity interactEntity;
    private final LightSource lightSource;
    private final GazewindService gazewindService;
    private final DamageEventHookComposite damageHook;
    private final MoveForceEventHookComposite moveForceHook;
    private boolean left, right, top, bottom;
    private int maxHealth;
    private int health;
    private int maxArmor;
    private int armor;
    private long lastHitTime;

    public  Player() {
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
        physicsBody.setType(CollisionType.PLAYER);
        physicsBody.setUserData(this);

        interactEntity = new CommonInteractEntity(getWidth(), getHeight(), physicsBody.getPhysicsTranslation());
        InteractionService.getInstance().addEntity(this);

        lightSource = LightningService.getInstance().createLightSource(new Vector3f(216, 156, 36), 150f, 0.5f);

        healthBar = new StatusBar(this::getHealth, this::getMaxHealth, StatusBarType.HEALTH, new Vector2f(25, 25));
        armorBar = new StatusBar(this::getArmor, this::getMaxArmor, StatusBarType.ARMOR, new Vector2f(75, 62.5f));

        gazewindService = new GazewindService();

        damageHook = new DamageEventHookComposite();
        moveForceHook = new MoveForceEventHookComposite();
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
            if (!RedVignette.isTimeBased()) {
                RedVignette.setTimeBased(true);
                AUDIO_SERVICE.play(SoundEffect.LOW_HEALTH);
            }
        } else {
            RedVignette.setTimeBased(false);
        }
        gazewindService.update((float) dt);
    }

    public int hit(final int rawDamage) {
        final int damage = damageHook.onDamage(rawDamage);
        if (isDead()) {
            return 0;
        }
        if (isInvulnerable()) {
            return 0;
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
            return 0;
        }
        AUDIO_SERVICE.play(SoundEffect.PLAYER_HURT);
        RedVignette.setMaxVignette();
        lastHitTime = System.currentTimeMillis();
        return damage;
    }

    public void fixedUpdate(final double dt) {
        if (isDead()) {
            return;
        }
        move(dt);
        final Vector2f velocity = physicsBody.getVelocity();
        if (Math.abs(velocity.x()) >= GAZE_SPEED_PRELIMIT || Math.abs(velocity.y()) >= GAZE_SPEED_PRELIMIT) {
            final Vector2f nextGazePosition = getPossibleGazePosition();
            gazewindService.spawn(nextGazePosition.x(), nextGazePosition.y());
        }
    }
    public Vector2f getPossibleGazePosition(final long offset) {
        final float nextGazeX;
        final float nextGazeY;
        if ((top || bottom) && !left && !right) {
            nextGazeX = getX() + DIMENSION_WIDTH / 2 + GAZE_OFFSET * (float) Math.sin(2 * Math.PI * (((System.currentTimeMillis() + offset) % 1000) / 1000d));
            nextGazeY = getY() + DIMENSION_HEIGHT;
        } else if ((right || left) && !top && !bottom) {
            nextGazeX = getX() + DIMENSION_WIDTH / 2;
            nextGazeY = getY() + DIMENSION_HEIGHT + GAZE_OFFSET * (float) Math.sin(Math.PI * (1 + (((System.currentTimeMillis() + offset) % 1000) / 1000d)));
        } else {
            nextGazeX = getX() + DIMENSION_WIDTH / 2;
            nextGazeY = getY() + DIMENSION_HEIGHT;
        }
        return new Vector2f(nextGazeX, nextGazeY);
    }
    public Vector2f getPossibleGazePosition() {
        return getPossibleGazePosition(0);
    }

    public void addHealth(final int health) {
        setHealth(getHealth() + health);
        AUDIO_SERVICE.play(SoundEffect.HEAL);
    }

    public boolean hasFullHealth() {
        return getHealth() == getMaxHealth();
    }

    public void addArmor(final int armor) {
        setArmor(getArmor() + armor);
        AUDIO_SERVICE.play(SoundEffect.HEAL);
    }

    public boolean hasFullArmor() {
        return getArmor() == getMaxArmor();
    }

    public void setMaxHealth(final int maxHealth) {
        final int healthDiff = maxHealth - this.maxHealth;
        this.maxHealth = maxHealth;
        if (healthDiff > 0) {
            health += healthDiff;
        }
        if (this.health > maxHealth) {
            this.health = maxHealth;
        }
    }
    public void setMaxArmor(final int maxArmor) {
        final int armorDiff = maxArmor - this.maxArmor;
        this.maxArmor = maxArmor;
        if (armorDiff > 0) {
            armor += armorDiff;
        }
        if (this.armor > maxArmor) {
            this.armor = maxArmor;
        }
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
    public void collectSubRenderables(RenderableState state) {
        state.add(healthBar).add(armorBar);
        gazewindService.collectSubRenderables(state);
    }

    @Override
    public RenderData renderData() {
        final List<Vertex> vertices = animation.getVertices(physicsBody.getTranslation(), getZIndex());
        vertices.forEach(v -> v.rotateAround(physicsBody.getTranslation(), physicsBody.getRotation()));
        return new RenderData(
                vertices,
                animation.getTextureId(),
                new CommonLightBinding()
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
        final Vector2f cameraPos = Camera.getInstance().getPosition();
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
        final long moveForce = moveForceHook.onMoveForce(MOVE_FORCE);
        if (left) {
            physicsBody.applyForce(-moveForce, 0);
        } else if (right) {
            physicsBody.applyForce(moveForce, 0);
        }
        if (top) {
            physicsBody.applyForce(0, -moveForce);
        } else if (bottom) {
            physicsBody.applyForce(0, moveForce);
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
    public int hit(DamageSourceType source, int damage) {
        if (canHit(source, damage)) {
            return hit(damage);
        }
        return 0;
    }

    public void addDamageHook(final DamageEventHook hook) {
        damageHook.addHook(hook);
    }

    public void removeDamageHook(final DamageEventHook hook) {
        damageHook.removeHook(hook);
    }

    public Vector2f getRenderPosition() {
        return physicsBody.getTranslation();
    }

    public void addHook(final MoveForceEventHook hook) {
        moveForceHook.addHook(hook);
    }

    public void removeHook(final MoveForceEventHook hook) {
        moveForceHook.removeHook(hook);
    }

    public Vector2f getVelocity() {
        return physicsBody.getVelocity();
    }

    public void stopMoving() {
        bottom = false;
        top = false;
        left = false;
        right = false;
    }

    public LightSource getLightSource() {
        return lightSource;
    }

    public boolean hasEnoughHealth(final int damage) {
        return health - damage >= 0;
    }

    // use this, if you want remove raw health without any hooks
    public void removeHealth(final int damage) {
        health -= damage;
        AUDIO_SERVICE.play(SoundEffect.PLAYER_HURT);
        RedVignette.setMaxVignette();
        lastHitTime = System.currentTimeMillis();
    }
}
