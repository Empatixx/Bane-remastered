package cz.empatix.physics.domain;

import cz.empatix.physics.domain.geometry.Geometry;
import cz.empatix.physics.domain.transform.CommonTransform;
import cz.empatix.physics.domain.transform.InterpolatedTransform;
import cz.empatix.physics.domain.transform.Transform;
import org.joml.Vector2f;

public class PhysicsBody {
    private final Geometry geometry;
    private final Transform transform;
    private final Vector2f velocity;
    private final Vector2f force;
    private float linearDamping;
    private float restitution;
    private final int mass;
    private final float inversedMass;
    private boolean remove;
    private boolean immmovable;
    public PhysicsBody(final Geometry geometry,
                       final Transform transform,
                       final boolean immmovable){
        this.geometry = geometry;
        this.transform = transform;
        velocity = new Vector2f();
        force = new Vector2f();
        linearDamping = 0.9f;
        restitution = 0f;
        this.immmovable = immmovable;
        if(this.immmovable){
            inversedMass = 0;
            mass = Integer.MAX_VALUE;
        } else {
            final int area = geometry.getArea();
            mass = area;
            inversedMass = 1f / area;
        }
    }
    public static PhysicsBody createDynamic(final Geometry geometry) {
        return new PhysicsBody(
                geometry,
                new InterpolatedTransform(),
                false
        );
    }

    public boolean isImmmovable() {
        return immmovable;
    }

    public static PhysicsBody createStatic(final Geometry geometry){
        return new PhysicsBody(
                geometry,
                new CommonTransform(),
                true
        );
    }
    public void updateVelocity(final float dt) {
        this.velocity.x += (force.x / mass) * dt;
        this.velocity.y += (force.y / mass) * dt;

        this.force.set(0, 0);

        this.velocity.x *= linearDamping;
        this.velocity.y *= linearDamping;
    }

    public void setLinearDamping(final float linearDamping) {
        this.linearDamping = linearDamping;
    }
    public void translate(final Vector2f translation){
        transform.translate(translation.x, translation.y);
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
    public void setTranslation(final float x, final float y) {
        this.transform.setTranslation(x, y);
    }
    public void applyForce(final float x, final float y){
        this.force.add(x,y);
    }
    public int getMass(){
        return mass;
    }
    public void destroy() {
        remove = true;
    }
    public boolean canBeRemoved(){
        return remove;
    }

    public Vector2f getForce() {
        return force;
    }

    public float getLinearDamping() {
        return linearDamping;
    }

    public Geometry getGeometry() {
        return geometry;
    }

    public float getRestitution() {
        return restitution;
    }

    public void setRestitution(float restitution) {
        this.restitution = restitution;
    }

    public float getInversedMass() {
        return inversedMass;
    }
}
