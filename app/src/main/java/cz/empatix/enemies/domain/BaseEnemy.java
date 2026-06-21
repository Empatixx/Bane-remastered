package cz.empatix.enemies.domain;

import cz.empatix.enemies.service.DifficultyScaling;
import cz.empatix.enemies.service.VitalityDisplayService;
import cz.empatix.enemies.targetting.AStarTargetting;
import cz.empatix.items.service.DroppableState;
import cz.empatix.collision.DamageSource;
import cz.empatix.collision.DamageSourceType;
import cz.empatix.collision.Damageable;
import cz.empatix.newphysics.attributes.InterpolatedPhysicsObject;
import cz.empatix.collision.CollisionType;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import cz.empatix.render.shaderbindings.OutlineBinding;
import cz.empatix.shaderbindings.SpawnBinding;
import org.joml.Vector2f;
import org.joml.Vector4f;


public abstract class BaseEnemy implements Drawable, Damageable, DamageSource, Enemy {
    protected final static int SPAWN_INVINCIBILITY = 2000;
    protected final static int TRAP_INVINCIBILITY = 1000;
    protected final VitalityDisplayService vitalityDisplayService = new VitalityDisplayService();
    protected final InterpolatedPhysicsObject physicsBody;
    protected long moveForce;
    protected DroppableState droppableState;
    protected final AStarTargetting aStarTargetting;
    protected final int width;
    protected final int height;

    protected Animation animation;

    protected  int maxHealth;
    protected int health;

    protected int damage;

    protected long lastHit;
    protected long trapInvincibility;

    protected long spawnTime;
    public BaseEnemy(final int width,
                     final int height){
        this.width = width;
        this.height = height;

        physicsBody = PhysicsObjectFactory.createInterpolated(width, height);
        physicsBody.setType(CollisionType.ENEMY);
        physicsBody.setUserData(this);

        droppableState = new DroppableState(this::canDrop);

        aStarTargetting = new AStarTargetting(physicsBody::getPhysicsTranslation, width, height);

        spawnTime = System.currentTimeMillis();
    }

    @Override
    @Deprecated
    public void onCollision() {

    }
    public boolean isSpawning(){
        return System.currentTimeMillis() - spawnTime < SPAWN_INVINCIBILITY;
    }

    @Override
    public void update(double dt){
        if (isSpawning()){
            return;
        }
        animation.update();
        aStarTargetting.update((float) dt);
        vitalityDisplayService.update(dt);
    }
    @Override
    public void fixedUpdate(double dt){
        if (isSpawning()){
            physicsBody.setEnabled(false);
            return;
        }
        if (!physicsBody.isEnabled()){
            physicsBody.setEnabled(true);
        }
        aStarTargetting.updatePath();
        aStarTargetting.update((float) dt);
        final Vector2f force = aStarTargetting.getDirectionTarget().mul(moveForce, moveForce);
        physicsBody.applyForce(force.x(), force.y());
    }


    abstract public void onDeath();
    abstract public void onSpawn();
    @Override
    public boolean canDropItems(){
        return droppableState.canDrop();
    }
    public boolean canDrop(){
        return droppableState != null && isDead() && animation.hasPlayedOnce();
    }
    public Vector2f getPosition(){
        return physicsBody.getPhysicsTranslation();
    }
    public float getX(){
        return physicsBody.getPhysicsTranslation().x();
    }
    public float getY(){
        return physicsBody.getPhysicsTranslation().y();
    }
    public void setPosition(final float x, final float y){
        physicsBody.setPhysicsTranslation(x, y);
    }
    public int getWidth(){
        return width;
    }
    public int getHeight(){
        return height;
    }
    public boolean isDead(){
        return health <= 0;
    }
    @Override
    public int hit(final int damage){
        if (isSpawning()){
            return 0;
        }
        if (isDead()) {
            return 0;
        }
        vitalityDisplayService.addChange(-damage, getPosition(), getWidth(), getHeight());
        this.health -= damage;
        if (this.health < 0) {
            this.health = 0;
        }
        if (isDead()) {
            physicsBody.destroy();
            onDeath();
        }
        lastHit = System.currentTimeMillis();
        return damage;
    }
    public void heal(final int amount){
        if (isSpawning()){
            return;
        }
        if (isDead()) {
            return;
        }
        vitalityDisplayService.addChange(amount, getPosition(), getWidth(), getHeight());
        this.health += amount;
        if (this.health > maxHealth) {
            this.health = maxHealth;
        }

    }
    @Override
    public void scaleToFloor(final int floor){
        maxHealth = DifficultyScaling.scaleStat(maxHealth, DifficultyScaling.healthMultiplier(floor));
        health = maxHealth;
        damage = DifficultyScaling.scaleStat(damage, DifficultyScaling.damageMultiplier(floor));
    }
    @Override
    public void setTarget(final Player player){
        aStarTargetting.target(player);
    }

    @Override
    public boolean canRender() {
        return Camera.getInstance().isInProjection(getPosition(), getWidth(), getHeight()) &&
                !(isDead() && animation.hasPlayedOnce());
    }

    @Override
    public RenderData renderData() {
        if (isSpawning()){
            return new RenderData(
                    animation.getVertices(physicsBody.getTranslation(), getZIndex(), aStarTargetting.isTargetLeftFromEnemy()),
                    animation.getTextureId(),
                    new SpawnBinding((float) (System.currentTimeMillis() - spawnTime) / SPAWN_INVINCIBILITY)
            );
        }
        final float outline = 1 - (System.currentTimeMillis() - lastHit) / 1000f;
        if (outline > 0) {
            return new RenderData(
                    animation.getVertices(physicsBody.getTranslation(), getZIndex(), aStarTargetting.isTargetLeftFromEnemy()),
                    animation.getTextureId(),
                    new OutlineBinding(new Vector2f(getWidth(), getHeight()),
                            animation.getNormalStartPositions(), animation.getNormalEndPositions(), new Vector4f(1, 0, 0, outline))
            );
        } else {
            return new RenderData(
                    animation.getVertices(physicsBody.getTranslation(), getZIndex(), aStarTargetting.isTargetLeftFromEnemy()),
                    animation.getTextureId(),
                    new CommonLightBinding()
            );
        }
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
        vitalityDisplayService.collectSubRenderables(state);
    }

    @Override
    public int getZIndex() {
        return 3;
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }

    @Override
    public int hit(DamageSourceType source, int damage) {
        if (canHit(source, damage)) {
            final int received = hit(damage);
            if (source == DamageSourceType.TRAP) {
                trapInvincibility = System.currentTimeMillis();
            }
            return received;
        }
        return 0;
    }
    private boolean canHit(final DamageSourceType source, final int damage) {
        if (source == DamageSourceType.TRAP){
            return System.currentTimeMillis() - trapInvincibility > TRAP_INVINCIBILITY;
        }
        return source != DamageSourceType.ENEMY;
    }
    @Override
    public DamageSourceType type() {
        return DamageSourceType.ENEMY;
    }

    @Override
    public int damage() {
        if (isSpawning()){
            return 0;
        }
        return damage;
    }

    @Override
    public int getHealth(){
        return health;
    }

    public boolean canRemove() {
        return isDead() && vitalityDisplayService.isEmpty() && animation.hasPlayedOnce();
    }

}
