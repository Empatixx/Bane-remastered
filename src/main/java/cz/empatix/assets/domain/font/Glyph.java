package cz.empatix.assets.domain.font;

import cz.empatix.assets.builders.TextureBuilder;
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

    public static Glyph from(final RawGlyph rawGlyph) {
        final int textureId = rawGlyph.bitmap() != null ? TextureBuilder.builder()
                .width(rawGlyph.width())
                .height(rawGlyph.height())
                .channels(1)
                .applyClampToEdgeWrap()
                .applyLinearFilters()
                .buffer(rawGlyph.bitmap())
                .build() : RenderData.NO_TEXTURE;
        return new Glyph(
                rawGlyph.width(),
                rawGlyph.height(),
                rawGlyph.bearingX(),
                rawGlyph.advance(),
                rawGlyph.ascent(),
                rawGlyph.descent(),
                rawGlyph.lineGap(),
                textureId
        );

    }

    public void bindTexture() {
        GL30.glBindTexture(GL30.GL_TEXTURE_2D, textureId);
    }

    public GlyphDrawable toDrawable(final Vector2f position,
                                    final Vector3f color,
                                    final int zDepth) {
        return new GlyphDrawable(this, position, color, zDepth);
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
                    .position(position)
                    .offsetPosition(glyph.bearingX(), glyph.descent())
                    .posDimensions(glyph.width(), -glyph.ascent() - glyph.descent())
                    .reverseTextureY()
                    .buildQuadTriangles();
        }

        @Override
        public boolean shouldRender() {
            return false; //Todo: remove
        }

        @Override
        public int getZIndex() {
            return zDepth;
        }
    }
}
