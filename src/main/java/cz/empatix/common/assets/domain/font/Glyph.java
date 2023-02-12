package cz.empatix.common.assets.domain.font;

import cz.empatix.render.Camera;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.shaderbindings.TextShaderBinding;
import org.joml.Vector2f;
import org.joml.Vector3f;
import org.lwjgl.opengl.GL30;

import java.util.List;

public record Glyph(
        int width,
        int height,
        int bearingX,
        int advance,
        int ascent,
        int descent,
        int lineGap,
        int textureId
) {
    public void destroy() {
        GL30.glDeleteTextures(textureId);
    }

    public static GlyphBuilder builder() {
        return new GlyphBuilder();
    }

    public void bindTexture() {
        GL30.glBindTexture(GL30.GL_TEXTURE_2D, textureId);
    }

    public GlyphDrawable toDrawable(final Vector2f position,
                                    final Vector3f color,
                                    final int zDepth) {
        return new GlyphDrawable(this, position, color, zDepth);
    }

    public static class GlyphBuilder {
        private int width;
        private int height;
        private int bearingX;
        private int advance;
        private int ascent;
        private int descent;
        private int lineGap;
        private int textureId;

        public GlyphBuilder() {
            textureId = RenderData.NO_TEXTURE;
        }

        public GlyphBuilder width(final int width) {
            this.width = width;
            return this;
        }

        public GlyphBuilder height(final int height) {
            this.height = height;
            return this;
        }

        public GlyphBuilder bearingX(final int bearingX) {
            this.bearingX = bearingX;
            return this;
        }

        public GlyphBuilder advance(final int advance) {
            this.advance = advance;
            return this;
        }

        public GlyphBuilder ascent(final int ascent) {
            this.ascent = ascent;
            return this;
        }

        public GlyphBuilder descent(final int descent) {
            this.descent = descent;
            return this;
        }

        public GlyphBuilder lineGap(final int lineGap) {
            this.lineGap = lineGap;
            return this;
        }

        public GlyphBuilder textureId(final int textureId) {
            this.textureId = textureId;
            return this;
        }

        public Glyph build() {
            return new Glyph(width, height, bearingX, advance, ascent, descent, lineGap, textureId);
        }
    }

    public record GlyphDrawable(
            Glyph glyph,
            Vector2f position,
            Vector3f color,
            int zDepth
    ) implements Drawable {
        @Override
        public RenderData renderData() {
            return new RenderData(
                    vertices(),
                    glyph.textureId(),
                    new TextShaderBinding(color, Camera.getProjection()),
                    RenderData.RenderDataType.DYNAMIC
            );
        }

        private List<Vertex> vertices() {
            return Vertex.builder()
                    .textureDimensions(1,1)
                    .texturePosition(0,0)
                    .position(position)
                    .offsetPosition(glyph.bearingX(), glyph.descent())
                    .posDimensions(glyph.width(), -glyph.ascent()-glyph.descent())
                    .reverseTextureY()
                    .buildQuad();
        }

        @Override
        public boolean shouldRender() {
            return true;
        }

        @Override
        public int getZIndex() {
            return zDepth;
        }
    }
}
