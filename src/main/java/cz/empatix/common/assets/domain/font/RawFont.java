package cz.empatix.common.assets.domain.font;

import org.lwjgl.stb.STBTTFontinfo;

import java.nio.ByteBuffer;

public record RawFont (
        STBTTFontinfo fontInfo,
        ByteBuffer ttf
){
}
