package cz.empatix.common.assets.domain;

import java.nio.ShortBuffer;
import static org.lwjgl.system.libc.LibCStdlib.free;

public record RawAudio (
        int format,
        int sampleRate,
        ShortBuffer data
){
    public void destroy() {
        free(data);
    }
}
