package cz.empatix.common.assets.domain.metadata;

import com.google.common.base.Preconditions;
import cz.empatix.render.domain.AnimationState;

public record Row(
        AnimationState type,
        int numSprites
) {
    public Row{
        Preconditions.checkArgument(numSprites > 0, "Number of sprites must be greater than 0");
        Preconditions.checkNotNull(type, "Type must not be null");
    }
}