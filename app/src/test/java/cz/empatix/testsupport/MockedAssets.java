package cz.empatix.testsupport;

import cz.empatix.assets.domain.font.Font;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.ShaderProgram;
import cz.empatix.textureatlas.domain.TextureAtlas;
import cz.empatix.textureatlas.domain.TextureAtlasInfo;
import org.joml.Vector2i;
import org.mockito.MockedStatic;

import static org.mockito.ArgumentMatchers.any;
import static org.mockito.ArgumentMatchers.anyString;
import static org.mockito.Mockito.mock;

/**
 * Test support for headlessly instantiating classes that fetch GPU assets through the static
 * {@link AssetsService} in their constructors. Open a {@code mockStatic(AssetsService.class)} in a
 * try-with-resources and call {@link #stub(MockedStatic)} to make every asset accessor return a
 * GL-free fake — the returned {@link TextureAtlas.Entry} carries real pixel dimensions so the
 * sprite/atlas math run in HUD/animation constructors produces finite values.
 */
public final class MockedAssets {

    private MockedAssets() {
    }

    /** A GL-free atlas entry: a 16x16 sprite inside a 64x64 atlas. */
    public static TextureAtlas.Entry entry() {
        return new TextureAtlas.Entry(1, new Vector2i(64, 64), new TextureAtlasInfo(0, 0, 16, 16));
    }

    /** Wire the static asset accessors to return GL-free fakes for any name. */
    public static void stub(final MockedStatic<AssetsService> assets) {
        assets.when(() -> AssetsService.getTexture(anyString())).thenReturn(entry());
        assets.when(() -> AssetsService.getFont(anyString(), any())).thenReturn(mock(Font.class));
        assets.when(() -> AssetsService.getShader(anyString())).thenReturn(mock(ShaderProgram.class));
    }
}
