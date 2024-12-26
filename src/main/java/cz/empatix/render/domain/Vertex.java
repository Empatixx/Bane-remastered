package cz.empatix.render.domain;

import com.google.common.base.Preconditions;
import cz.empatix.textureatlas.domain.TextureAtlas;
import cz.empatix.textureatlas.domain.TextureAtlasInfo;
import org.dyn4j.geometry.Interval;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.joml.Vector3f;

import java.util.List;

public record Vertex(
        Vector3f position,
        Vector2f texCoords
) {
    public void rotateAround(final Vector2f point, final double radians){
        final float x = position.x() - point.x;
        final float y = position.y() - point.y;
        final double cos = Math.cos(radians);
        final double sin = Math.sin(radians);
        final float newX = (float) (x * cos - y * sin);
        final float newY = (float) (x * sin + y * cos);
        position.set(newX + point.x, newY + point.y, position.z());
    }

    public static class Builder {
        private final Vector2i posDimensions;
        private final Vector2f position;
        private final Vector2f posOffset;
        private final Vector2i texPosition;
        private final Vector2i texPositionDimensions;
        private boolean reverseTexY;
        private boolean rotatedX;
        private TextureAtlas.Entry texture;
        private float zDepth;

        public Builder() {
            posDimensions = new Vector2i();
            position = new Vector2f();
            posOffset = new Vector2f();
            texPosition = new Vector2i();
            texPositionDimensions = new Vector2i(1, 1);
            zDepth = 0;
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
        public Builder offset(final int width, final int height) {
            this.posOffset.x = 3f / width;
            this.posOffset.y = 3f / height;
            return this;
        }

        public Builder zDepth(final float zDepth) {
            this.zDepth = zDepth;
            return this;
        }

        public List<Vertex> buildQuad() {
            final Vector3f posBottomLeft = new Vector3f(position.x() + posOffset.x(), position.y() + posOffset.y(), zDepth);
            final Vector3f posBottomRight = new Vector3f(position.x() + posDimensions.x() + posOffset.x(), position.y() + posOffset.y(), zDepth);
            final Vector3f posTopRight = new Vector3f(position.x() + posDimensions.x() + posOffset.x(), position.y() + posDimensions.y() + posOffset.y(), zDepth);
            final Vector3f posTopLeft = new Vector3f(position.x() + posOffset.x(), position.y() + posDimensions.y() + posOffset.y(), zDepth);

            final int baseTexWidth = texture.width() / texPositionDimensions.x();
            final int baseTexHeight = texture.height() / texPositionDimensions.y();

            final TextureAtlasInfo info = texture.info();

            final float top = (info.textureStartPositions().y() + (texPosition.y() + 1) * baseTexHeight) / (float) texture.atlasHeight();
            final float bottom = (info.textureStartPositions().y() + texPosition.y() * baseTexHeight) / (float) texture.atlasHeight();
            final float left = (info.textureStartPositions().x() + texPosition.x() * baseTexWidth) / (float) texture.atlasWidth();
            final float right = (info.textureStartPositions().x() + (texPosition.x() + 1) * baseTexWidth) / (float) texture.atlasWidth();

            Preconditions.checkArgument(top <= 1);
            Preconditions.checkArgument(bottom >= 0);
            Preconditions.checkArgument(left >= 0);
            Preconditions.checkArgument(right <= 1);

            final Vector2f texBottomLeft = new Vector2f(left, bottom);
            final Vector2f texBottomRight = new Vector2f(right, bottom);
            final Vector2f texTopRight = new Vector2f(right, top);
            final Vector2f texTopLeft = new Vector2f(left, top);

            if (reverseTexY) {
                return getVertices(posBottomLeft, posBottomRight, posTopRight, posTopLeft, texBottomLeft, texBottomRight, texTopRight, texTopLeft);
            }
            return getVertices(posBottomLeft, posBottomRight, posTopRight, posTopLeft, texTopLeft, texTopRight, texBottomRight, texBottomLeft);
        }

        private List<Vertex> getVertices(final Vector3f posBottomLeft,
                                         final Vector3f posBottomRight,
                                         final Vector3f posTopRight,
                                         final Vector3f posTopLeft,
                                         final Vector2f texBottomLeft,
                                         final Vector2f texBottomRight,
                                         final Vector2f texTopRight,
                                         final Vector2f texTopLeft) {
            if(rotatedX){
                return List.of(
                        new Vertex(posBottomLeft, texTopRight),
                        new Vertex(posBottomRight, texTopLeft),
                        new Vertex(posTopRight, texBottomLeft),
                        new Vertex(posTopLeft, texBottomRight));
            }
            return List.of(
                    new Vertex(posBottomLeft, texTopLeft),
                    new Vertex(posBottomRight, texTopRight),
                    new Vertex(posTopRight, texBottomRight),
                    new Vertex(posTopLeft, texBottomLeft));
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

        public Builder rotatedX(final boolean rotateX) {
            this.rotatedX = rotateX;
            return this;
        }
        public Builder scale(final float scale){
            this.posDimensions.x *= scale;
            this.posDimensions.y *= scale;
            return this;
        }
    }

    public static Builder builder() {
        return new Builder();
    }
}