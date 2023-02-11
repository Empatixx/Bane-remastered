package cz.empatix.render.domain;

import org.joml.Vector2f;

import java.util.List;

public record Vertex (
    Vector2f position,
    Vector2f texCoords
){

    public static class Builder{
        private int posHeight;
        private int posWidth;
        private Vector2f position;

        private int totalColumns;
        private int totalRows;

        private int textureCol;
        private int textureRow;

        public Builder position(final Vector2f position){
            this.position = position;
            return this;
        }
        public Builder posDimensions(final int width, final int height){
            this.posWidth = width;
            this.posHeight = height;
            return this;
        }
        public Builder textureDimensions(final int totalColumns, final int totalRows){
            this.totalColumns = totalColumns;
            this.totalRows = totalRows;
            return this;
        }
        public Builder texturePosition(final int textureCol, final int textureRow){
            this.textureCol = textureCol;
            this.textureRow = textureRow;
            return this;
        }
        public List<Vertex> buildQuad(){
            return List.of(
                    new Vertex(new Vector2f(position.x + posWidth, position.y), new Vector2f((float)(textureCol + 1) / totalColumns, (float)textureRow / totalRows)),
                    new Vertex(new Vector2f(position.x, position.y), new Vector2f((float)textureCol / totalColumns, (float)textureRow / totalRows)),
                    new Vertex(new Vector2f(position.x + posWidth, position.y + posHeight), new Vector2f((float)(textureCol + 1) / totalColumns, (float)(textureRow + 1) / totalRows)),
                    new Vertex(new Vector2f(position.x, position.y + posHeight), new Vector2f((float)textureCol / totalColumns, (float)(textureRow + 1) / totalRows)
            ));
        }
    }
    public static Builder builder(){
        return new Builder();
    }
}