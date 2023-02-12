package cz.empatix.render.domain;

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
        private final Vector2i texDimensions;
        private final Vector2i texPosition;
        private final Vector2f texOffset;
        private final Vector2f posOffset;
        private boolean reverseTextureY;

        public Builder() {
            posDimensions = new Vector2i();
            position = new Vector2f();
            texDimensions = new Vector2i();
            texPosition = new Vector2i();
            texOffset = new Vector2f();
            posOffset = new Vector2f();
            reverseTextureY = false;
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

        public Builder textureDimensions(final int totalColumns, final int totalRows) {
            this.texDimensions.x = totalColumns;
            this.texDimensions.y = totalRows;
            return this;
        }

        public Builder texturePosition(final int textureCol, final int textureRow) {
            this.texPosition.x = textureCol;
            this.texPosition.y = textureRow;
            return this;
        }

        public Builder offsetTexture(final int textureWidth, final int textureHeight) {
            this.texOffset.x = 2f / textureWidth;
            this.texOffset.y = 2f / textureHeight;

            return this;
        }

        public Builder offsetPosition(final int x, final int y) {
            this.posOffset.x = x;
            this.posOffset.y = y;
            return this;
        }
        public Builder reverseTextureY(){
            this.reverseTextureY = true;
            return this;
        }


        public List<Vertex> buildQuad() {
            final Vector2f posBottomLeft = new Vector2f(position.x() + posOffset.x(), position.y() + posOffset.y());
            final Vector2f posBottomRight = new Vector2f(position.x() + posDimensions.x() + posOffset.x(), position.y() + posOffset.y());
            final Vector2f posTopRight = new Vector2f(position.x() + posDimensions.x() + posOffset.x(), position.y() + posDimensions.y() + posOffset.y());
            final Vector2f posTopLeft = new Vector2f(position.x() + posOffset.x(), position.y() + posDimensions.y() + posOffset.y());

            final Vector2f texBottomLeft = new Vector2f((float) texPosition.x() / texDimensions.x() + texOffset.x(), (float) texPosition.y() / texDimensions.y() + texOffset.y());
            final Vector2f texBottomRight = new Vector2f((float) (texPosition.x() + 1) / texDimensions.x() - texOffset.x(), (float) texPosition.y() / texDimensions.y() + texOffset.y());
            final Vector2f texTopRight = new Vector2f((float) (texPosition.x() + 1) / texDimensions.x() - texOffset.x(), (float) (texPosition.y() + 1) / texDimensions.y() - texOffset.y());
            final Vector2f texTopLeft = new Vector2f((float) texPosition.x() / texDimensions.x() + texOffset.x(), (float) (texPosition.y() + 1) / texDimensions.y() - texOffset.y());

            if(reverseTextureY){
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
    }

    public static Builder builder() {
        return new Builder();
    }
}