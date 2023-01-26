package cz.empatix.render.domain;

import cz.empatix.assets.domain.metadata.Row;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class Animation {
    private final static Logger LOGGER = LoggerFactory.getLogger(Animation.class);
    private final Map<AnimationState, Integer> numSprites;
    private final Map<AnimationState,Integer> rowSprites;
    private AnimationState currentState;
    private int currentTextureIndex;
    private long lastAnimation;
    private long delayBetweenAnimations;

    public Animation(final List<Row> sprites) {
        numSprites = sprites.stream()
                .map(row -> Map.entry(row.type(), row.numSprites()))
                .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
        rowSprites = IntStream.range(0, sprites.size())
                .mapToObj(i -> Map.entry(sprites.get(i).type(), i))
                .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
        delayBetweenAnimations = 100;

        sprites.stream()
                .map(Row::type)
                .findFirst()
                .ifPresentOrElse(this::setAnimationState, () -> LOGGER.error("No animation states found for {}", this));
    }

    public void setAnimationState(final AnimationState state) {
        if(!numSprites.containsKey(state)){
            LOGGER.error("Animation state {} not found for {}", state, this);
        }
        currentState = state;
        currentTextureIndex = 0;
        lastAnimation = System.currentTimeMillis();
    }

    public void update() {
        if (delayBetweenAnimations == -1) {
            return;
        }
        if (System.currentTimeMillis() - lastAnimation > delayBetweenAnimations) {
            lastAnimation = System.currentTimeMillis();
            currentTextureIndex++;
            if (currentTextureIndex >= numSprites.get(currentState)) {
                currentTextureIndex = 0;
            }
        }
    }
    public int getCurrentTextureRow(){
        return rowSprites.get(currentState);
    }
    public int getCurrentTextureColumn(){
        return currentTextureIndex;
    }

    public void setDelayBetweenAnimations(long delayBetweenAnimations) {
        this.delayBetweenAnimations = delayBetweenAnimations;
    }

}
