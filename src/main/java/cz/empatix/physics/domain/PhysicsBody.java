package cz.empatix.physics.domain;

import cz.empatix.physics.domain.transform.CommonTransform;
import cz.empatix.physics.domain.transform.InterpolatedTransform;
import cz.empatix.physics.domain.transform.Transform;
import cz.empatix.physics.service.AABB;
import cz.empatix.physics.service.CollisionListener;
import org.joml.Vector2f;

public class PhysicsBody {
    private final CollisionListener collisionListener;
    private final Geometry geometry;
    private final Transform transform;
    private final Vector2f velocity;
    private final Vector2f force;
    private final float linearDamping;
    private final float restitution;
    private final int mass;
    public PhysicsBody(final Geometry geometry,
                       final Transform transform,
                       final int mass){
        collisionListener = new CollisionListener(this);
        this.geometry = geometry;
        this.transform = transform;
        velocity = new Vector2f();
        force = new Vector2f();
        linearDamping = 0.9f;
        restitution = 0.5f;
        this.mass = mass;
    }
    public PhysicsBody(final Geometry geometry,
                       final Transform transform){
        this(geometry, transform, geometry.getMass());
    }
    public static PhysicsBody createDynamic(final Geometry geometry) {
        return new PhysicsBody(
                geometry,
                new InterpolatedTransform()
        );
    }
    public static PhysicsBody createStatic(final Geometry geometry){
        return new PhysicsBody(
                geometry,
                new CommonTransform(),
                Integer.MAX_VALUE
        );
    }
    public void updateVelocity(final float dt) {
        this.velocity.x += (force.x / mass) * dt;
        this.velocity.y += (force.y / mass) * dt;

        this.force.set(0, 0);

        this.velocity.x *= linearDamping;
        this.velocity.y *= linearDamping;
    }
    public void update(final float dt) {
        transform.update(dt);
    }
    public void updateTranslation(float dt) {
        transform.translate(this.velocity.x() * dt, this.velocity.y() * dt);
    }
    public Vector2f getVelocity() {
        return velocity;
    }
    public void collision(final PhysicsBody body,final float dt) {
        collisionListener.onCollision(body, dt);
    }
    public boolean isIntersecting(final PhysicsBody other,
                                  final float dt) {
        if(getGeometry().canAABB() && other.getGeometry().canAABB()){
            return AABB.isIntersecting(this, other, dt);
        } else {
            //todo: SAT
            return false;
        }
    }
    public float getTranslationX(){
        return getTranslation().x();
    }
    public float getTranslationY(){
        return getTranslation().y();
    }
    public float getPhysicsTranslationX(){
        return getPhysicsTranslation().x();
    }
    public float getPhysicsTranslationY(){
        return getPhysicsTranslation().y();
    }
    public Vector2f getTranslation(){
        return transform.getTranslationToRender();
    }
    public Vector2f getPhysicsTranslation(){
        return transform.getTranslation();
    }

    public void setVelocity(final float x, final float y) {
        this.velocity.x = x;
        this.velocity.y = y;
    }
    public void setVelocity(final Vector2f velocity){
        this.velocity.x = velocity.x();
        this.velocity.y = velocity.y();
    }
    public void setTranslation(final float x, final float y) {
        this.transform.setTranslation(x, y);
    }
    public void applyForce(final float x, final float y){
        this.force.add(x,y);
    }
    public int getMass(){
        return mass;
    }
    public float getRestitution(){
        return restitution;
    }
    public Geometry getGeometry() {
        return geometry;
    }
}
