package cz.empatix.physics.domain;

import cz.empatix.physics.domain.geometry.Geometry;
import cz.empatix.physics.domain.transform.CommonTransform;
import cz.empatix.physics.domain.transform.InterpolatedTransform;
import cz.empatix.physics.domain.transform.Transform;
import cz.empatix.physics.events.consumers.EventConsumer;
import cz.empatix.physics.events.consumers.FallbackConsumer;
import cz.empatix.physics.events.domain.Event;
import cz.empatix.physics.events.domain.EventType;
import cz.empatix.physics.events.suppliers.EventSupplier;
import org.joml.Vector2f;

import java.util.EnumMap;
import java.util.List;
import java.util.Map;

public class PhysicsBody {
    private final static FallbackConsumer FALLBACK_CONSUMER = new FallbackConsumer();
    private final Geometry geometry;
    private final Transform transform;
    private final Vector2f velocity;
    private final Vector2f force;
    private float linearDamping;
    private float restitution;
    private final int mass;
    private final float inversedMass;
    private boolean remove;
    private final boolean immovable;
    private final Map<EventType, EventConsumer> eventConsumers;
    private final Map<EventType, EventSupplier> eventSuppliers;

    public PhysicsBody(final Geometry geometry,
                       final Transform transform,
                       final boolean immovable) {
        this.geometry = geometry;
        this.transform = transform;
        velocity = new Vector2f();
        force = new Vector2f();
        linearDamping = 0.9f;
        restitution = 0f;
        this.immovable = immovable;
        if (this.immovable) {
            inversedMass = 0;
            mass = Integer.MAX_VALUE;
        } else {
            final int area = geometry.getArea();
            mass = area;
            inversedMass = 1f / area;
        }
        eventConsumers = new EnumMap<>(EventType.class);
        eventSuppliers = new EnumMap<>(EventType.class);
    }

    public void addEventConsumer(final EventConsumer eventConsumer) {
        eventConsumers.put(eventConsumer.getType(), eventConsumer);
    }

    public void addEventSupplier(final EventSupplier eventSupplier) {
        eventSuppliers.put(eventSupplier.getType(), eventSupplier);
    }
    public void removeEventSupplier(final EventType eventType) {
        eventSuppliers.remove(eventType);
    }

    public void consumeEvents(final List<Event> events) {
        events.forEach(event -> eventConsumers.getOrDefault(event.getType(), FALLBACK_CONSUMER).consume(event));
    }

    public List<Event> produceEvents(final PhysicsBody a, final PhysicsBody b) {
        return eventSuppliers.values()
                .stream()
                .map(s -> s.supply(a, b))
                .toList();
    }

    public static PhysicsBody createDynamic(final Geometry geometry) {
        return new PhysicsBody(
                geometry,
                new InterpolatedTransform(),
                false
        );
    }

    public boolean isImmovable() {
        return immovable;
    }

    public static PhysicsBody createStatic(final Geometry geometry) {
        return new PhysicsBody(
                geometry,
                new CommonTransform(),
                true
        );
    }

    public void setLinearDamping(final float linearDamping) {
        this.linearDamping = linearDamping;
    }

    public void translate(final Vector2f translation) {
        translate(translation.x(), translation.y());
    }
    public void translate(final float x, final float y) {
        transform.translate(x, y);
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

    public float getTranslationX() {
        return getTranslation().x();
    }

    public float getTranslationY() {
        return getTranslation().y();
    }

    public float getPhysicsTranslationX() {
        return getPhysicsTranslation().x();
    }

    public float getPhysicsTranslationY() {
        return getPhysicsTranslation().y();
    }

    public Vector2f getTranslation() {
        return transform.getTranslationToRender();
    }

    public Vector2f getPhysicsTranslation() {
        return transform.getTranslation();
    }

    public void setVelocity(final float x, final float y) {
        this.velocity.x = x;
        this.velocity.y = y;
    }

    public void setTranslation(final float x, final float y) {
        this.transform.setTranslation(x, y);
    }

    public void applyForce(final float x, final float y) {
        this.force.add(x, y);
    }

    public int getMass() {
        return mass;
    }

    public void destroy() {
        remove = true;
    }

    public boolean canBeRemoved() {
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
