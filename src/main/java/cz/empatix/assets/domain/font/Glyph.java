package cz.empatix.assets.domain.font;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.TextBinding;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.util.List;

public record Glyph(
        int width,
        int height,
        int bearingX,
        int advance,
        int ascent,
        int descent,
        int lineGap,
        TextureAtlas.Entry texture
) {

    public static Glyph from(final RawGlyph rawGlyph, final TextureAtlas.Entry texture) {
        return new Glyph(
                rawGlyph.width(),
                rawGlyph.height(),
                rawGlyph.bearingX(),
                rawGlyph.advance(),
                rawGlyph.ascent(),
                rawGlyph.descent(),
                rawGlyph.lineGap(),
                texture
        );

    }


    public GlyphDrawable toDrawable(final Vector2f position,
                                    final Vector3f color,
                                    final int zDepth,
                                    final RenderScene scene) {
        return new GlyphDrawable(this, position, color, zDepth, scene);
    }

    public record GlyphDrawable(
            Glyph glyph,
            Vector2f position,
            Vector3f color,
            int zDepth,
            RenderScene scene
    ) implements Drawable {
        @Override
        public RenderData renderData() {
            return new RenderData(
                    vertices(),
                    glyph.texture().textureId(),
                    new TextBinding(color, Camera.getOriginWithoutZoomProjection())
            );
        }

        @Override
        public RenderScene renderScene() {
            return scene;
        }

        private List<Vertex> vertices() {
            return Vertex.builder()
                    .texture(glyph.texture())
                    .offsetPosition(glyph.bearingX(), -glyph.ascent())
                    .texPosition(0, 0)
                    .texPositionsDimensions(1, 1)
                    .position(position)
                    .posDimensions(glyph.width(), glyph.ascent() + glyph.descent())
                    .buildQuad();
        }

        @Override
        public boolean canRender() {
            return true; //Todo: remove
        }

        @Override
        public int getZIndex() {
            return zDepth;
        }
    }
}
