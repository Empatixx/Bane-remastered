package cz.empatix.physics.service;

import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.render.Camera;
import org.lwjgl.opengl.GL30;

import java.util.ArrayList;
import java.util.List;

public class PhysicsEngine {
    private final static int MAX_ITERATIONS = 5;
    private static List<PhysicsBody> physicsBodies;

    public PhysicsEngine() {
        physicsBodies = new ArrayList<>();
    }
    public void update(final double dt){
        physicsBodies.forEach(body -> body.update((float)dt));
        for(int k = 0;k< MAX_ITERATIONS;k++){
            boolean collision = false;
            for(int i = 0; i < physicsBodies.size(); i++){
                for(int j = i + 1; j < physicsBodies.size(); j++){
                    if(physicsBodies.get(i).isIntersecting(physicsBodies.get(j))){
                        physicsBodies.get(i).collision(physicsBodies.get(j));
                        physicsBodies.get(j).collision(physicsBodies.get(i));
                        collision = true;
                    }
                }
            }
            if(!collision){
                break;
            }
        }
    }
    public void debugRender(){
       /* world.getBodies().forEach(body -> {
            body.getFixtures().forEach(fixture -> {
                Vector2[] vertices = ((Rectangle) fixture.getShape()).getVertices();
                GL30.glBegin(GL30.GL_LINE_LOOP);
                for (Vector2 vertex : vertices) {
                            GL30.glVertex2f((float) vertex.x + (float)body.getTransform().getTranslationX() - Camera.getX(), (float) vertex.y + (float)body.getTransform().getTranslationY() - Camera.getY());
                }
                GL30.glEnd();
            });
        });*/
    }
}
