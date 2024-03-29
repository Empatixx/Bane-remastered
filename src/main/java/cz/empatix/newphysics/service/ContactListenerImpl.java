package cz.empatix.newphysics.service;

import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.dynamics.contact.Contact;
import org.dyn4j.dynamics.contact.SolvedContact;
import org.dyn4j.world.ContactCollisionData;
import org.dyn4j.world.listener.ContactListener;

public class ContactListenerImpl implements ContactListener<PhysicsObject> {
    @Override
    public void begin(ContactCollisionData<PhysicsObject> collision, Contact contact) {
    }

    @Override
    public void persist(ContactCollisionData<PhysicsObject> collision, Contact oldContact, Contact newContact) {
    }

    @Override
    public void end(ContactCollisionData<PhysicsObject> collision, Contact contact) {
    }

    @Override
    public void destroyed(ContactCollisionData<PhysicsObject> collision, Contact contact) {

    }

    @Override
    public void collision(ContactCollisionData<PhysicsObject> collision) {
        final PhysicsObject a = collision.getBody1();
        final PhysicsObject b = collision.getBody2();
        if (!canCollide(a, b)) {
            collision.getContactConstraint().setEnabled(false);
        } else {
            a.onCollision(b);
            b.onCollision(a);
        }
    }

    @Override
    public void preSolve(ContactCollisionData<PhysicsObject> collision, Contact contact) {

    }

    @Override
    public void postSolve(ContactCollisionData<PhysicsObject> collision, SolvedContact contact) {

    }
    private boolean canCollide(final PhysicsObject o1, final PhysicsObject o2){
        final CollisionType t1 = o1.getType();
        final CollisionType t2 = o2.getType();

        if (t1 == null || t2 == null) { // default
            return true;
        }
        return t1.collidesWith(t2);
    }
}
