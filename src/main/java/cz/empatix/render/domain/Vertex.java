package cz.empatix.render.domain;

import org.joml.Vector2f;

import java.util.List;

public record Vertex(
        Vector2f position,
        Vector2f texCoords
) {

    public static class Builder {
        private int posHeight;
        private int posWidth;
        private Vector2f position;

        private int totalColumns;
        private int totalRows;

        private int textureCol;
        private int textureRow;
        private float offsetX;
        private float offsetY;

        public Builder position(final Vector2f position) {
            this.position = position;
            return this;
        }

        public Builder posDimensions(final int width, final int height) {
            this.posWidth = width;
            this.posHeight = height;
            return this;
        }

        public Builder textureDimensions(final int totalColumns, final int totalRows) {
            this.totalColumns = totalColumns;
            this.totalRows = totalRows;
            return this;
        }

        public Builder texturePosition(final int textureCol, final int textureRow) {
            this.textureCol = textureCol;
            this.textureRow = textureRow;
            return this;
        }

        public Builder offsetTexture(final int textureWidth, final int textureHeight) {
            this.offsetX = 2f / textureWidth;
            this.offsetY = 2f / textureHeight;
            return this;
        }

        public List<Vertex> buildQuad() {
            final Vector2f posBottomLeft = new Vector2f(position.x, position.y);
            final Vector2f posBottomRight = new Vector2f(position.x + posWidth, position.y);
            final Vector2f posTopRight = new Vector2f(position.x + posWidth, position.y + posHeight);
            final Vector2f posTopLeft = new Vector2f(position.x, position.y + posHeight);

            final Vector2f texBottomLeft = new Vector2f((float) textureCol / totalColumns + offsetX, (float) textureRow / totalRows + offsetY);
            final Vector2f texBottomRight = new Vector2f((float) (textureCol + 1) / totalColumns - offsetX, (float) textureRow / totalRows + offsetY);
            final Vector2f texTopRight = new Vector2f((float) (textureCol + 1) / totalColumns - offsetX, (float) (textureRow + 1) / totalRows - offsetY);
            final Vector2f texTopLeft = new Vector2f((float) textureCol / totalColumns + offsetX, (float) (textureRow + 1) / totalRows - offsetY);

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