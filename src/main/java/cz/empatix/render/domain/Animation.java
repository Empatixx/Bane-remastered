package cz.empatix.render.domain;

import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

public class Animation {
    private final static Logger LOGGER = LoggerFactory.getLogger(Animation.class);
    private final Map<AnimationState, AnimationPart> animationParts;
    private final int totalRows;
    private final int scale;
    private final TextureAtlas.Entry texture;
    private AnimationState currentState;
    private int currentTextureIndex;
    private long lastAnimation;

    public Animation(final Map<AnimationState, AnimationPart> animationParts,
                     final TextureAtlas.Entry texture,
                     final int scale) {
        this.texture = texture;
        this.scale = scale;
        this.animationParts = animationParts;
        this.currentState = AnimationState.IDLE;
        this.currentTextureIndex = 0;
        this.lastAnimation = System.currentTimeMillis();
        this.totalRows = animationParts.values()
                .stream()
                .map(AnimationPart::row)
                .collect(Collectors.toSet())
                .size();
    }

    public void setAnimationState(final AnimationState state) {
        if(currentState == state){
            return;
        }
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
                .texture(texture)
                .texPosition(currentTextureIndex, part.row())
                .texPositionsDimensions(part.numSprites(), totalRows)
                .position(position)
                .posDimensions(part.width(), part.height())
                .offset(part.width(), part.height())
                .rotatedX(part.rotatedX())
                .scale(scale)
                .buildQuad();
    }
    public int getTextureId() {
        return texture.textureId();
    }


    private record AnimationPart(
            Vector2i dimensions,
            Vector2i animationDimensions,
            int delayBetweenAnimations,
            boolean rotatedX
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
        private TextureAtlas.Entry texture;
        private int scale;

        public Builder() {
            this.animationParts = new HashMap<>();
            scale = 1;
        }

        public Builder addAnimationState(final AnimationState state,
                                         final int row,
                                         final int numSprites,
                                         final int width,
                                         final int height,
                                         final int delayBetweenAnimations) {
            return addAnimationState(state, row, numSprites, width, height, delayBetweenAnimations, false);
        }
        public Builder addAnimationState(final AnimationState state,
                                         final int row,
                                         final int numSprites,
                                         final int width,
                                         final int height,
                                         final int delayBetweenAnimations,
                                         final boolean rotatedX) {
            final AnimationPart part = new AnimationPart(
                    new Vector2i(width, height),
                    new Vector2i(row, numSprites),
                    delayBetweenAnimations,
                    rotatedX
            );
            animationParts.put(state, part);
            return this;
        }
        public Builder texture(final TextureAtlas.Entry texture){
            this.texture = texture;
            return this;
        }
        public Builder scale(final int scale){
            this.scale = scale;
            return this;
        }

        public Animation build() {
            return new Animation(animationParts, texture, scale);
        }
    }
    public static Builder builder(){
        return new Builder();
    }

}
