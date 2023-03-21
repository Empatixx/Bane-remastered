package cz.empatix.render.domain;

import com.google.common.base.Preconditions;
import cz.empatix.textureatlas.domain.TextureAtlas;
import cz.empatix.textureatlas.domain.TextureAtlasInfo;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.List;

public record Vertex(
        Vector2f position,
        Vector2f texCoords
) {

    public static class Builder {
        private final Vector2i posDimensions;
        private final Vector2f position;
        private final Vector2f posOffset;
        private final Vector2i texPosition;
        private final Vector2i texPositionDimensions;
        private boolean reverseTexY;
        private TextureAtlas.Entry texture;

        public Builder() {
            posDimensions = new Vector2i();
            position = new Vector2f();
            posOffset = new Vector2f();
            texPosition = new Vector2i();
            texPositionDimensions = new Vector2i(1, 1);
        }

        public Builder position(final Vector2f position) {
            this.position.x = position.x;
            this.position.y = position.y;
            return this;
        }

        public Builder posDimensions(final int width, final int height) {
            this.posDimensions.x = width;
            this.posDimensions.y = height;
            return this;
        }


        public List<Vertex> buildQuad() {
            final Vector2f posBottomLeft = new Vector2f(position.x() + posOffset.x(), position.y() + posOffset.y());
            final Vector2f posBottomRight = new Vector2f(position.x() + posDimensions.x() + posOffset.x(), position.y() + posOffset.y());
            final Vector2f posTopRight = new Vector2f(position.x() + posDimensions.x() + posOffset.x(), position.y() + posDimensions.y() + posOffset.y());
            final Vector2f posTopLeft = new Vector2f(position.x() + posOffset.x(), position.y() + posDimensions.y() + posOffset.y());

            final int baseTexWidth = texture.width() / texPositionDimensions.x();
            final int baseTexHeight = texture.height() / texPositionDimensions.y();

            final float texOffsetX = 3f;
            final float texOffsetY = 3f;

            final TextureAtlasInfo info = texture.info();

            final float top = (info.textureStartPositions().y() + (texPosition.y() + 1) * baseTexHeight - texOffsetY) / (float) texture.atlasHeight();
            final float bottom = (info.textureStartPositions().y() + texPosition.y() * baseTexHeight + texOffsetY) / (float) texture.atlasHeight();
            final float left = (info.textureStartPositions().x() + texPosition.x() * baseTexWidth + texOffsetX) / (float) texture.atlasWidth();
            final float right = (info.textureStartPositions().x() + (texPosition.x() + 1) * baseTexWidth - texOffsetX) / (float) texture.atlasWidth();

            Preconditions.checkArgument(top <= 1);
            Preconditions.checkArgument(bottom >= 0);
            Preconditions.checkArgument(left >= 0);
            Preconditions.checkArgument(right <= 1);

            final Vector2f texBottomLeft = new Vector2f(left, bottom);
            final Vector2f texBottomRight = new Vector2f(right, bottom);
            final Vector2f texTopRight = new Vector2f(right, top);
            final Vector2f texTopLeft = new Vector2f(left, top);

            if (reverseTexY) {
                return List.of(
                        new Vertex(posBottomLeft, texTopLeft),
                        new Vertex(posBottomRight, texTopRight),
                        new Vertex(posTopRight, texBottomRight),
                        new Vertex(posTopLeft, texBottomLeft));
            }
            return List.of(
                    new Vertex(posBottomLeft, texBottomLeft),
                    new Vertex(posBottomRight, texBottomRight),
                    new Vertex(posTopRight, texTopRight),
                    new Vertex(posTopLeft, texTopLeft));
        }

        public Builder texture(TextureAtlas.Entry texture) {
            this.texture = texture;
            return this;
        }

        public Builder offsetPosition(final float offsetX,
                                      final float offsetY) {
            this.posOffset.x = offsetX;
            this.posOffset.y = offsetY;
            return this;
        }

        public Builder reverseTextureY() {
            this.reverseTexY = true;
            return this;
        }

        public Builder texPosition(final int column, final int row) {
            this.texPosition.x = column;
            this.texPosition.y = row;
            return this;
        }

        public Builder texPositionsDimensions(final int maxColumns, final int maxRows) {
            this.texPositionDimensions.x = maxColumns;
            this.texPositionDimensions.y = maxRows;
            return this;
        }
    }

    public static Builder builder() {
        return new Builder();
    }
}