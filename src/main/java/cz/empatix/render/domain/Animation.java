package cz.empatix.render.domain;

import org.joml.Vector2f;
import org.joml.Vector2i;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Animation {
    private final static Logger LOGGER = LoggerFactory.getLogger(Animation.class);
    private final Map<AnimationState, AnimationPart> animationParts;
    private final Vector2i textureDimensions;
    private AnimationState currentState;
    private int currentTextureIndex;
    private long lastAnimation;

    public Animation(final Map<AnimationState, AnimationPart> animationParts,
                     final Vector2i textureDimensions) {
        this.animationParts = animationParts;
        this.textureDimensions = textureDimensions;
        this.currentState = AnimationState.IDLE;
        this.currentTextureIndex = 0;
        this.lastAnimation = System.currentTimeMillis();
    }

    public void setAnimationState(final AnimationState state) {
        if (!animationParts.containsKey(state)) {
            LOGGER.error("Animation state {} not found for {}", state, this);
        }
        currentState = state;
        currentTextureIndex = 0;
        lastAnimation = System.currentTimeMillis();
    }

    public void update() {
        final AnimationPart part = animationParts.get(currentState);
        final long delayBetweenAnimations = part.delayBetweenAnimations();
        if (delayBetweenAnimations == -1) {
            return;
        }
        if (System.currentTimeMillis() - lastAnimation > delayBetweenAnimations) {
            lastAnimation = System.currentTimeMillis();
            currentTextureIndex++;
            if (currentTextureIndex >= part.numSprites()) {
                currentTextureIndex = 0;
            }
        }
    }

    public List<Vertex> getVertices(final Vector2f position) {
        final AnimationPart part = animationParts.get(currentState);
        return Vertex.builder()
                .texturePosition(currentTextureIndex, part.row())
                .textureDimensions(textureDimensions.x(), textureDimensions.y())
                .position(position)
                .posDimensions(part.width(), part.height())
                .buildQuad();
    }

    private record AnimationPart(
            Vector2i dimensions,
            Vector2i animationDimensions,
            int delayBetweenAnimations
    ) {
        public int numSprites(){
            return animationDimensions.y();
        }
        public int row(){
            return animationDimensions.x();
        }
        public int width(){
            return dimensions.x();
        }
        public int height(){
            return dimensions.y();
        }
    }

    public static class Builder {
        private final Map<AnimationState, AnimationPart> animationParts;
        private Vector2i textureDimensions;

        public Builder() {
            this.animationParts = new HashMap<>();
        }

        public Builder addAnimationState(final AnimationState state,
                                         final int row,
                                         final int numSprites,
                                         final int width,
                                         final int height,
                                         final int delayBetweenAnimations) {
            final AnimationPart part = new AnimationPart(
                    new Vector2i(width, height),
                    new Vector2i(row, numSprites),
                    delayBetweenAnimations
            );
            animationParts.put(state, part);
            return this;
        }
        public Builder textureDimensions(final int totalColumns, final int totalRows){
            this.textureDimensions = new Vector2i(totalColumns, totalRows);
            return this;
        }

        public Animation build() {
            return new Animation(animationParts, textureDimensions);
        }
    }
    public static Builder builder(){
        return new Builder();
    }

}
